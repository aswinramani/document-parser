# AGENTS.md

Rules for AI coding agents (and a quick orientation for humans) working in this repo.

## What this project is

A streaming Rust parser for the Problems section (LOINC 11450-4) of C-CDA documents, crate `clinical-document-parser`. Current work: converting the parsed section to FHIR R4 `Condition` resources. Later: expose the parser to agents through an MCP server, then an HTTP API.

## Commands

```
cargo build
cargo test                      # unit tests in src/, integration tests in tests/problems/
cargo clippy --all-targets      # must stay clean
cargo bench --bench parse       # needs generated inputs, see below
cargo run --example trace       # parse one file with tracing output
```

Benchmark inputs are generated, not committed (`sample-documents/generated/` is gitignored). Generate them with `scripts/generate_samples.py` (see `.github/workflows/benchmark.yml` for the exact commands and sizes).

## Layout

```
src/parser/          streaming parser (quick-xml)
src/utils/           data model structs
src/fhir/            FHIR R4 types and conversion (in progress)
tests/problems/      integration tests
sample-documents/    C-CDA fixtures; fhir/ holds expected FHIR JSON
benches/ examples/   Criterion benchmark; soak, trace and profile drivers
scripts/             Python sample generator
.github/workflows/   CI
```

## Who writes what (most important rule)

The repo owner writes all Rust by hand. This is how they learn the language, so it is not negotiable by default.

- **Do not create or edit `.rs` files** (src, tests, benches, examples) unless the owner explicitly asks for that specific change. A permission applies to that one change only, not to later ones.
- Instead, show code in the chat, explain it, review the owner's code, find bugs, and suggest tests.
- **You may write** Markdown, JSON and XML fixtures, YAML workflows, Python under `scripts/`, and config files, when asked.
- Work in **one small step at a time**. Give one step, wait for the owner to run it, then continue.
- **Do not commit, push, merge or force-push.** The owner makes all commits.

## Safety

- Never read, print or copy `.env`. Never write API keys or tokens into any file.
- Synthetic or published sample data only. No real patient data (PHI), in fixtures, logs or examples.
- Put throwaway experiments in a temp directory outside the repo, not in the working tree.

## Code conventions

- Match the style of the surrounding code. Errors use `thiserror`.
- No global state in the parser. Per-run statistics are returned by value (`ParseStats`).
- `tracing` is per document, never per XML element (it sits on the hot path).
- FHIR: **R4 only**. serde `rename_all = "camelCase"` and `skip_serializing_if = "Option::is_none"`. A struct means "all of these fields"; an enum means "one of these" (for example `onset[x]`).
- Every bug fix gets a test that fails without the fix.
- Fixtures live under `sample-documents/`. FHIR golden tests compare serialised JSON as `serde_json::Value`, not as strings.

## Definition of done

1. `cargo test` passes.
2. `cargo clippy --all-targets` is clean.
3. If the parser's hot path changed, run `cargo bench --bench parse` before and after and report the difference.
4. README or docs updated if behaviour or commands changed.

## Plans

The roadmaps (`cda-parser-plan.md`, `cda-parser-performance-plan.md`) are kept outside this repo, in the parent folder. If a task depends on them, ask the owner.
