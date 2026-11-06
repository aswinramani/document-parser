use clinical_document_parser::parser::problem_section_from_reader;
use hdrhistogram::Histogram;
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

const SAMPLE_SECS: u64 = 10;

// (name, file, weight out of 20)
const CORPUS: [(&str, &str, usize); 3] = [
    ("100kb", "sample-documents/generated/normal_0.1mb.xml", 14),
    ("1mb", "sample-documents/generated/normal_1mb.xml", 5),
    ("5mb", "sample-documents/generated/medium_5mb.xml", 1),
];

// n-th command-line argument as a number, or the default
fn arg(n: usize, default: u64) -> u64 {
    std::env::args()
        .nth(n)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn main() {
    let rate = arg(1, 200);
    let duration_secs = arg(2, 30);
    let warmup_secs = arg(3, 5);
    let worker_count = arg(4, 4) as usize;
    let burst_factor = arg(5, 1);
    let burst_start_secs = arg(6, 30);
    let burst_len_secs = arg(7, 30);
    println!(
        "rate={rate}/s duration={duration_secs}s warmup={warmup_secs}s workers={worker_count} \
         burst={burst_factor}x from {burst_start_secs}s for {burst_len_secs}s"
    );

    let docs: Vec<Vec<u8>> = CORPUS
        .iter()
        .map(|(_, path, _)| std::fs::read(path).expect("generate the files first"))
        .collect();

    // index of the document to use at each step of a 20-long cycle
    let mut cycle = Vec::new();
    for (i, (_, _, weight)) in CORPUS.iter().enumerate() {
        cycle.extend(std::iter::repeat_n(i, *weight));
    }

    let docs = Arc::new(docs);
    let (tx, rx) = mpsc::channel::<(usize, Instant, bool)>();
    let rx = Arc::new(Mutex::new(rx));

    let handles: Vec<_> = (0..worker_count)
        .map(|_| {
            let rx = Arc::clone(&rx);
            let docs = Arc::clone(&docs);
            std::thread::spawn(move || {
                let mut total = Histogram::<u64>::new(3).unwrap();
                let mut wait = Histogram::<u64>::new(3).unwrap();
                loop {
                    let job = rx.lock().unwrap().recv();
                    let Ok((doc_index, intended, measured)) = job else { break };
                    if measured {
                        wait.record(intended.elapsed().as_micros() as u64).unwrap();
                    }
                    problem_section_from_reader(docs[doc_index].as_slice()).unwrap();
                    if measured {
                        total.record(intended.elapsed().as_micros() as u64).unwrap();
                    }
                }
                (total, wait)
            })
        })
        .collect();

    // prints resident memory every SAMPLE_SECS; never joined, it dies when main returns
    let pid = std::process::id().to_string();
    std::thread::spawn(move || {
        let t0 = Instant::now();
        loop {
            let out = std::process::Command::new("ps")
                .args(["-o", "rss=", "-p", &pid])
                .output()
                .unwrap();
            let kb: f64 = String::from_utf8_lossy(&out.stdout)
                .trim()
                .parse()
                .unwrap_or(0.0);
            println!("t={:>4}s rss={:.1} MB", t0.elapsed().as_secs(), kb / 1024.0);
            std::thread::sleep(Duration::from_secs(SAMPLE_SECS));
        }
    });

    let total_time = Duration::from_secs(duration_secs);
    let warmup = Duration::from_secs(warmup_secs);
    let burst_start = Duration::from_secs(burst_start_secs);
    let burst_end = burst_start + Duration::from_secs(burst_len_secs);

    let start = Instant::now();
    let mut offset = Duration::ZERO; // intended send time, relative to start
    let mut i = 0usize;
    while offset < total_time {
        let intended = start + offset;
        // sleep until ~2 ms early, then spin: thread::sleep alone wakes ~1 ms late on macOS
        loop {
            let now = Instant::now();
            if now >= intended {
                break;
            }
            let remaining = intended - now;
            if remaining > Duration::from_millis(2) {
                std::thread::sleep(remaining - Duration::from_millis(2));
            } else {
                std::hint::spin_loop();
            }
        }
        tx.send((cycle[i % cycle.len()], intended, offset >= warmup)).unwrap();

        // the gap to the next send depends on which phase we are in
        let current_rate = if offset >= burst_start && offset < burst_end {
            rate * burst_factor
        } else {
            rate
        };
        offset += Duration::from_secs_f64(1.0 / current_rate as f64);
        i += 1;
    }
    drop(tx); // closes the channel so workers see Err and exit

    let mut latency = Histogram::<u64>::new(3).unwrap();
    let mut waited = Histogram::<u64>::new(3).unwrap();
    for h in handles {
        let (t, q) = h.join().unwrap();
        latency.add(t).unwrap();
        waited.add(q).unwrap();
    }
    println!(
        "wait: p50={}µs p99={}µs max={}µs",
        waited.value_at_quantile(0.50),
        waited.value_at_quantile(0.99),
        waited.max(),
    );
    println!(
        "n={} mean={:.0}µs p50={}µs p99={}µs p99.9={}µs max={}µs",
        latency.len(),
        latency.mean(),
        latency.value_at_quantile(0.50),
        latency.value_at_quantile(0.99),
        latency.value_at_quantile(0.999),
        latency.max(),
    );
}
