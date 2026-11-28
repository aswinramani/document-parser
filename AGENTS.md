# AGENTS.md

Rules for AI coding agents (and a quick orientation for humans) working in this repo.

## What this project is

A streaming Rust parser for the Problems section (LOINC 11450-4) of C-CDA documents, crate `clinical-document-parser`. Current work: converting the parsed section to FHIR R4 `Condition` resources. Later: expose the parser to agents through an MCP server, then an HTTP API.

## Commands

```
cargo build --workspace
cargo test --workspace          # unit tests in src/, integration tests in tests/problems/ and tests/fhir/
cargo clippy --workspace --all-targets   # must stay clean
cargo build -p clinical-document-mcp     # builds the MCP server binary that .mcp.json points to
cargo bench --bench parse       # needs generated inputs, see below
cargo run --example trace       # parse one file with tracing output
```

Benchmark inputs are generated, not committed (`sample-documents/generated/` is gitignored). Generate them with `scripts/generate_samples.py` (see `.github/workflows/benchmark.yml` for the exact commands and sizes).

## Layout

```
src/parser/          streaming parser (quick-xml)
src/utils/           data model structs
src/fhir/            FHIR R4: types (datatypes, condition), shared mapping helpers (mapping), Problems conversion (problems, not written yet)
mcp-server/          separate workspace crate: exposes the parser to agents as MCP tools over stdio
.mcp.json            registers the MCP server with Claude Code (build the binary first)
tests/problems/      integration tests for the parser
tests/fhir/          integration tests for the FHIR types and mapping helpers
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
- Integration tests live in `tests/<area>/main.rs`, which declares one module per topic. Cargo finds the directory by itself, with no `Cargo.toml` entry. A bare file directly in a `tests/` subfolder without a `main.rs` is not run.
- **Code-system mappings** (OID to URI, status and code maps) go into code only after being checked against a named source: the HL7 FHIR R4 pages, the C-CDA on FHIR guide, or the code system's owner. Name the source in a comment. If a pair cannot be verified, leave it out. The `urn:oid:<oid>` fallback in `oid_to_uri` is our own assumption, not a rule from the spec.

## MCP server rules (`mcp-server/`)

- **stdout carries the protocol.** Never use `println!` or `print!` in the server. Log to stderr only, or a stray line corrupts the messages.
- **The parser library stays free of async and MCP.** `tokio` and `rmcp` belong in `mcp-server/` only, never in the root crate.
- Tool descriptions and parameter doc comments are read by the model. Write them as instructions to a new colleague.
- Return the smallest useful output. Once the FHIR conversion exists, prefer FHIR `Condition` JSON over the raw parse tree.
- Known gap: tool failures currently come back as ordinary text. They should become proper MCP tool errors.
- Test a tool without an agent: `npx @modelcontextprotocol/inspector --cli ./target/debug/clinical-document-mcp --method tools/list` (use `--method tools/call --tool-name <name> --tool-arg key=value` to call one).

## Definition of done

1. `cargo test --workspace` passes.
2. `cargo clippy --workspace --all-targets` is clean.
3. If the parser's hot path changed, run `cargo bench --bench parse` before and after and report the difference.
4. README or docs updated if behaviour or commands changed.

## Plans

The roadmaps (`cda-parser-plan.md`, `cda-parser-performance-plan.md`) are kept outside this repo, in the parent folder. If a task depends on them, ask the owner.
