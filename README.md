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

## CI

| Workflow | Runs on | What it does |
|---|---|---|
| `rust-static-analysis.yml` | push to `main`/`feat/**`/`fix/**`/`chore/**`/`dependabot/**`, PRs to `main`, weekly | `cargo clippy`, then CodeQL analysis for Rust |
| `python-static-analysis.yml` | same triggers | installs deps with `uv`, compiles and lints `scripts/` with `ruff`, then CodeQL analysis for Python |

## Dependabot

Weekly dependency update PRs (max 3 open at a time per ecosystem), configured in `.github/dependabot.yml`:
- **cargo** — repo root (`Cargo.toml`)
- **pip** — `scripts/` (`requirements.txt`)
- **github-actions** — the workflow files themselves
