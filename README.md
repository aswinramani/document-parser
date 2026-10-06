C-CDA Parser

Sample documents were taken from https://github.com/HL7/C-CDA-Examples.

LOINC Coverage Status:

11450-4 -> Problems section [In Progress]

## Layout

```
document-parser/
├── Cargo.toml          ← Rust parser (crate root)
├── src/
│   ├── main.rs         ← entry point, runs the parser against sample documents
│   ├── parser/         ← streaming C-CDA parser using quick-xml
│   └── utils/          ← data model structs
├── sample-documents/   ← C-CDA XML fixtures
└── scripts/            ← sample generation and static analysis helpers
```

## Running

```
cargo build
cargo run
cargo test
cargo clippy
```
