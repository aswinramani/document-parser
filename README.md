C-CDA Parser

Sample documents were taken from https://github.com/HL7/C-CDA-Examples.

LOINC Coverage Status:

11450-4 -> Problems section [In Progress]

## Layout

```
document-parser/
├── Cargo.toml          ← Cargo's Manifest Configuration File and strictly enforces only Tom's Obvious Minimal Language format.  
├── src/
│   ├── main.rs         ← entry point, runs the parser against sample documents
│   ├── parser/         ← streaming C-CDA parser using quick-xml
│   └── utils/          ← data model structs
├── sample-documents/   ← C-CDA XML fixtures
├── scripts/            ← Python script for generating sample C-CDA documents
└── .github/
    ├── workflows/      ← CI — see CI section below
    └── dependabot.yml  ← see Dependabot section below
```

## Running

```
cargo build
cargo run
cargo test
cargo clippy
```

## Performance

All published numbers come from a GitHub Linux runner (`ubuntu-latest`), produced by `benchmark.yml` on every merge to `main`. Shared runners drift by 10 to 30 percent between runs, so read the results as a trend and as a check for large regressions, not as exact figures.

**Available now:** Criterion parse benchmarks (100 KB, 1 MB, 5 MB, 10 MB, 25 MB and 50 MB documents, parsed from memory and from a path), with a trend chart updated on every merge to `main`: [aswinramani.github.io/document-parser/dev/bench](https://aswinramani.github.io/document-parser/dev/bench/).

**Coming once the CI jobs exist** (not published yet, to avoid quoting numbers from a different machine):
- steady load at 200 documents/s (errors, latency percentiles, memory, CPU)
- burst and capacity behaviour
- large documents (50 MB and 500 MB): parse time and peak memory

Known property, already measured locally: peak memory is about twice the input size, because the parser is not streaming and the whole result stays in memory.

## CI

| Workflow | Runs on | What it does |
|---|---|---|
| `rust-static-analysis.yml` | push to `main`/`feat/**`/`fix/**`/`chore/**`/`dependabot/**`, PRs to `main`, weekly | `cargo clippy`, then CodeQL analysis for Rust |
| `parser-tests.yml` | push to `main`/`feat/**`/`fix/**`/`chore/**`/`dependabot/**`, PRs to `main` | `cargo test` — unit tests (`src/`) plus the integration tests in `tests/problems/` |
| `python-static-analysis.yml` | same triggers as `rust-static-analysis.yml` | installs deps with `uv`, compiles and lints `scripts/` with `ruff`, then CodeQL analysis for Python |
| `benchmark.yml` | push to `main`, manual | generates the benchmark inputs with `scripts/generate_samples.py`, runs Criterion and stores the results on the `gh-pages` branch (trend chart); nothing is committed to `main` |

## Dependabot

Weekly dependency update PRs (max 3 open at a time per ecosystem), configured in `.github/dependabot.yml`:
- **cargo** — repo root (`Cargo.toml`)
- **pip** — `scripts/` (`requirements.txt`)
- **github-actions** — the workflow files themselves
