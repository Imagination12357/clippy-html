# Navigation metadata implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Export optional, versioned navigation JSON with real report URLs and basic finding information.

**Architecture:** A dedicated metadata module builds a public DTO from Report,
SourceFile and PageMap. A publication module stages internal or external JSON and
uses the existing output path checks. Main coordinates outputs without changing
the default report contract.

**Tech Stack:** Existing Rust/serde/serde_json/tempfile/url/scraper; MSRV 1.88.

**Spec:** `docs/superpowers/specs/2026-11-16-navigation-metadata-design.md`

## Global constraints

- All URLs are report-root-relative; no report base URL or complete diagnostic dump.
- No default metadata, new dependencies, network requests or UI changes.
- Actual PageMap and HTML anchors are authoritative; check exported destinations.
- Main checkout, autonomous verified commits, user-authorized push.

## Review focus

- Existing external JSON survives HTML generation or final JSON replacement failure.
- Symlink/reparse/case aliases and publication bookkeeping cannot be overwritten.
- Missing/stale source links land on a real fallback anchor, never a fabricated line.
- Incomplete/failed collection stays distinct from a successful clean report.
- External metadata uses the report root supplied by consumers, not the JSON folder.

### Task 1: CLI and navigation schema

**Files:** `src/cli.rs`, new `src/metadata.rs`, `src/main.rs`, `tests/metadata.rs`.

**Interfaces:** `metadata::build(&Report, &BTreeMap<PathBuf, SourceFile>, &PageMap)
-> NavigationMetadata`; serialized structure follows spec. `RunOptions.metadata:
Option<PathBuf>`. Metadata owns URL enumeration and staged destination validation.

- [x] Add CLI tests for opt-in, missing value, Unicode and forwarded arguments;
  add metadata integration tests for default absence and the schema/link contract.
- [x] Run focused tests and observe failure from the missing option/export.
- [x] Add the option, build deterministic DTOs using actual PageMap, and test
  no-span, missing/stale source, category/origin and input-order behavior.
- [x] Verify every emitted URL path and fragment against staged generated HTML.

### Task 2: Safe publication and main integration

**Files:** new `src/metadata/output.rs`, `src/output.rs`, `src/main.rs`,
`tests/metadata.rs`.

**Interfaces:** `MetadataTarget::prepare(requested, report_target)`,
`stage(bytes) -> PreparedMetadata`; `PreparedMetadata::write_report(staging)` and
`publish_external()` coordinate report-local and external paths. Reuse shared
`output::resolve_path` rather than guessing filesystem layout.

- [x] Test regular-file external replacement, generated/reserved path collisions,
  symlink/reparse rejection, Unicode nested paths and preflight preservation.
- [x] Observe RED; implement internal staging and external tempfile replacement.
- [x] Add injected publication-error unit tests: old external JSON retained on
  replacement failure; real HTML staging failure leaves old JSON unchanged.
- [x] Integrate main: prepare before collection; build/serialize/stage; render,
  write internal JSON, validate links, publish HTML, then external JSON; open last.
- [x] Run full Rust tests; expected zero failures and unchanged default behavior.

### Task 3: Public documentation and final verification

**Files:** new `docs/metadata.md`, `README.md`, `ROADMAP.md`, validation record.

- [x] Document exact schema, null/absent semantics, root-relative URLs, counts/IDs,
  overwrites, failure states, and a short consumer example without HTML parsing.
- [x] Run Rust 1.100 full suite, fmt, strict Clippy, Node suite and Rust 1.88
  all-target check/process test; expected all pass. Generate a real metadata preview.
- [x] Fresh whole-change review once; fix important findings with RED/GREEN tests.
- [x] Commit verified changes; push main without force as authorized and report delivery state separately.
