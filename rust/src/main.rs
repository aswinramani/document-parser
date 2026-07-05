mod utils;
mod parser;

fn init_problems() {
    let dir = "../sample-documents/problems/";
    for entry in std::fs::read_dir(dir).expect("could not read dir") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("xml") {
            println!("\n--- parsing {:?} ---", path.file_name().unwrap());
            parser::problem_section(path.to_str().unwrap());
        }
    }
}

fn main() {
    init_problems();
}
