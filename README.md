C-CDA Parser

Sample documents were taken from https://github.com/HL7/C-CDA-Examples.

LOINC Coverage Status:

11450-4 -> Problems section [In Progress]

FHIR R4 conversion (Problems to Condition): in progress.

## Layout

```
document-parser/
├── Cargo.toml          ← workspace root and manifest of the parser crate
├── src/
│   ├── main.rs         ← entry point, runs the parser against sample documents
│   ├── parser/         ← streaming C-CDA parser using quick-xml
│   ├── utils/          ← data model structs
│   └── fhir/           ← FHIR R4 types and mapping helpers (in progress)
├── mcp-server/         ← separate crate: exposes the parser to AI agents as an MCP tool
├── tests/              ← integration tests (problems/, fhir/)
├── benches/ examples/  ← Criterion benchmark; load and profiling drivers
├── sample-documents/   ← C-CDA XML fixtures and expected FHIR JSON
├── scripts/            ← Python script for generating sample C-CDA documents
├── AGENTS.md           ← rules for AI coding agents (CLAUDE.md imports it)
└── .github/
    ├── workflows/      ← CI — see CI section below
    └── dependabot.yml  ← see Dependabot section below
```

## Running

```
cargo build --workspace
cargo run
cargo test --workspace
cargo clippy --workspace
```

## MCP server

`mcp-server/` is a small separate crate that lets AI agents call the parser as a tool, using the Model Context Protocol over stdio. Currently there is one tool: `parse_problems(path)`, which returns the parsed Problems section as JSON.

```
cargo build -p clinical-document-mcp
```

`.mcp.json` registers the server with Claude Code for this project. Approve it on first use; `claude mcp list` should then show it as connected. To try a tool without an agent:

```
npx @modelcontextprotocol/inspector --cli ./target/debug/clinical-document-mcp --method tools/list
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
| `rust-static-analysis.yml` | push to `main`/`feat/**`/`fix/**`/`chore/**`/`dependabot/**`, PRs to `main`, weekly | `cargo clippy --workspace`, then CodeQL analysis for Rust |
| `clinical-doc-processing-tests.yml` | push to `main`/`feat/**`/`fix/**`/`chore/**`/`dependabot/**`, PRs to `main` | `cargo test --workspace` — unit tests (`src/`) plus the integration tests for the parser (`tests/problems/`) and the FHIR types and mapping helpers (`tests/fhir/`), and the MCP crate |
| `python-static-analysis.yml` | same triggers as `rust-static-analysis.yml` | installs deps with `uv`, compiles and lints `scripts/` with `ruff`, then CodeQL analysis for Python |
| `benchmark.yml` | push to `main`, manual | generates the benchmark inputs with `scripts/generate_samples.py`, runs Criterion and stores the results on the `gh-pages` branch (trend chart); nothing is committed to `main` |

## Dependabot

Weekly dependency update PRs (max 3 open at a time per ecosystem), configured in `.github/dependabot.yml`:
- **cargo** — repo root (`Cargo.toml`)
- **pip** — `scripts/` (`requirements.txt`)
- **github-actions** — the workflow files themselves
