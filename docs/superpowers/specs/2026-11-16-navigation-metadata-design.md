# Report navigation metadata

Approved direction: 2026-11-16 conversation. This replaces the original run-summary
purpose of `--metadata` with an optional machine-readable navigation index.

## Purpose and boundaries

Ordinary external programs can discover generated pages, filter diagnostics using
basic fields, and follow explicit URLs for detail. HTML remains the detailed
report. Do not export source text, snippets, child notes, suggestions, patches or
precise diagnostic ranges. Do not export rustc/Clippy versions just for navigation.
No UI change, runtime dependency, network request or default JSON output is needed.

Alternatives considered: a URL-only inventory still requires opening every page
to select findings; exporting the complete internal Report creates an unnecessarily
large public API. The selected approach combines explicit URLs with basic fields.

## Version 1 contract

- `schema_version`: integer `1`; incompatible changes require a new version.
- `generator`: `name` (`cargo-clippy-html`) and package `version`.
- `execution`: nullable `clippy_exit_code` and boolean `collection_complete`.
- `packages`: Cargo package `id`, `name`, `version`, sorted by ID. IDs are opaque
  report-local identifiers, not persistent cross-machine identities.
- `pages`: `kind` (`overview`, `lint`, `category`, `source`), `name`, `url`.
  Include generated navigation pages, not static assets, licenses, Help or Settings.
  Source pages include diagnostic-snippet fallback pages.
- `findings`: `id`, `package_id`, `origin`, nullable `code`, `level`, nullable
  `category`, `message`, `url`, and optional `location`.
- `location`: `path` (display path), one-based `line` and `column`, and `url`.
  Select first primary top-level span, else first top-level span. No span means
  no location field. Match HTML's validated line link or fallback finding anchor.
- A finding URL targets its canonical lint page's `finding-<id>` anchor. Category
  uses the same grouping as HTML: `unknown` for unknown Clippy, null for non-Clippy.
- All URLs are encoded report-root-relative references. No report base URL, local
  root directory or deployment URL is stored. Consumers supply the report root;
  metadata at that root can use its own containing directory. Moving external
  JSON alone does not define where its report lives.
- Findings are deduplicated and sorted by ID; pages have deterministic kind/name
  order. Consumers ignore unknown fields and do not interpret IDs or filenames.
- Existing PageMap and href encoding are authoritative. Validate every exported
  page and fragment against the staged HTML before publishing either artifact.

## CLI and publication

`--metadata <FILE>` is opt-in and requires a value. Relative paths use invocation
cwd, independently of `--output`. No extension restriction. Use UTF-8 pretty JSON.

Inside the report directory: write metadata into report staging after rendering,
before validation/publication. Refuse collisions with generated files/directories,
including case aliases on Windows, and the ownership marker/reserved output names.
Existing owned reports and their metadata are replaced together as usual.

Outside the report: explicitly selected regular files may be replaced. Prepare a
same-directory temporary file before publishing HTML, then replace the selected
file after successful HTML publication. Reject directories, symlinks/reparse points
and reserved publication paths; recheck before replacement. Missing parent folders
may be created. Hold the report publication lock through external JSON publication
to prevent successful cooperating writers from interleaving their two outputs.
An HTML failure leaves the old external JSON untouched. A final
external JSON publication failure keeps the finished HTML, preserves any previous
JSON, reports both states and fails the command; do not claim a two-file transaction.
Clippy failure/incomplete collection still permits exporting the available report
and preserves the existing exit policy. `--open` runs only after requested outputs
have been successfully published.

## Validation and delivery

Test default absence, CLI errors, relative/Unicode paths, workspace package and
dedup identity, clean/failed runs, URL encoding and actual fragment existence,
missing/stale source fallback, collisions, external replacement/failure preservation
and links/reparse points. Run Rust 1.100 full tests, fmt/Clippy, Node tests, MSRV
1.88 checks and the process regression. No browser layout claim is needed.
Work on local main; commit verified changes and push as authorized in conversation.
