# Official Categories Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline, with one independent final review. User approved the concrete design with 2026-11-07 ㄱㄱ; continue through implementation and verification.

**Goal:** Default-online official category lookup with bundled offline/failure fallback and static category navigation.

**Architecture:** Enrich findings after deduplication; keep report-level provenance separate from execution/completeness. Parse official HTML with scraper; use blocking ureq with TLS, a 10s total timeout, 8MiB limit and automatic redirects disabled. Renderer consumes data only.

**Tech Stack:** Existing Rust 2024/Askama/serde/scraper/BLAKE3; add ureq 3.5.1 with rustls and no unnecessary optional features; verify Rust 1.94 locked build.

**Spec:** ../specs/2026-11-03-categories-design.md (approved).

## Global constraints

- Default online; tool --offline controls metadata only; forwarded Cargo --offline remains independent.
- Clean/compiler-only runs never fetch. One validated documentation selector per run, constructed official HTTPS URL only. No persistent cache or latest-version substitution.
- Download/parse failure selects the entire bundled table and records the reason; missing names in valid online data stay Unknown.
- Preserve IDs, actual severity, forwarding, Clippy exit code, collection completeness, basic no-JS navigation and common table alignment.
- No search/theme/CI/export implementation in this slice.

## Review focus

- Unsafe diagnostic URLs/selectors and redirects must never direct metadata requests elsewhere.
- HTTP/body/parse failures must not fail a successful Clippy run or corrupt prior reports.
- Category lookup must not change finding identity or inflate multi-target/multi-span counts.
- Unknown Clippy and non-applicable rustc/general findings must stay distinct.
- Nested category/lint/source/license links must resolve; renderer must not access the network.

## Task 1: Metadata, bundle and CLI

Files: src/lint/{mod,metadata,fetch}.rs; src/main.rs; src/cli.rs; src/diagnostic/model.rs; Cargo.toml/lock; assets/vendor/clippy/; examples/update_lint_metadata.rs; tests/support/mod.rs.

Interfaces: `metadata::resolve(findings: &mut [Finding], offline: bool) -> Option<CategoryMetadata>`; internal `resolve_with(findings, offline, fetch: impl FnOnce(&str)->Result<Vec<u8>,String>)` for deterministic tests. `parse_html(&[u8]) -> Result<BTreeMap<String,String>,String>` shared with the maintenance example. CategoryMetadata owns source mode/version/URL/hash/fallback reason. No error is added to diagnostic Execution.

- [x] Add primary failing parser/CLI regressions; add supplemental version, selector, skip-fetch, fallback/missing-name and ID checks.
- [x] Implement table parsing, safe selector extraction, bounded HTTPS transport, bundle and provenance after the primary observed RED cases.
- [x] Generate fallback from verified 1.97 HTML, preserve MIT notice and provenance/hash; maintenance tool consumes local HTML, not hard-coded mappings.
- [x] Test all four tool/Cargo offline combinations; production enriches after dedup, fixtures explicitly use offline to avoid public network.
- [x] Run targeted tests and verify HTTP configuration/limit handling. Commit with Task 2 after end-to-end checks.

## Task 2: Category report navigation

Files: src/render/{paths,pages,categories,assets}.rs; templates/{category,index,lint,licenses}.html; tests/integration.rs; existing report constructors; README/ROADMAP/THIRD-PARTY-NOTICES; validation record.

Interfaces: `paths::category_key(&Finding) -> Option<&str>` maps recognized Clippy category or unknown; `PageMap.category_pages` contains present categories only. Category renderer builds home rows and writes pages using the existing Page shell. Lint finding views carry optional relative category links.

- [x] Add failing complexity/provenance/navigation integration test, then supplemental mixed Unknown/compiler/count checks.
- [x] Implement category pages/home summaries/linked labels with the existing dl.item-table and common layout; escape all source/provenance values.
- [x] Emit original notice and provenance into generated licenses; update documentation to describe actual behavior.
- [x] Run full tests, fmt, strict Clippy, Rust1.94 check and package verification. Generate both offline and real online reports; inspect provenance and all relative links.
- [x] Independent final review, record validation and prepare verified change for commit. No push/merge/publish.

## Ledger

- Baseline: 61e7d9d, clean codex/phase-1. Existing in-place branch and autonomous commits are user-authorized.
- Pre-flight: metadata enriches after dedup; category renderer reads those values and fixed category paths. No shared-interface conflict.
- Ruling: disable automatic redirects rather than add a redirect framework; the approved canonical 1.97 URL was verified directly. Any redirect uses the documented bundled fallback.
- Task 1 complete: parser and offline CLI observed RED -> GREEN; resolver/transport boundary supplemental tests passed; initial fallback regenerated with shared Rust parser. 815 active entries, 16 deprecated records excluded.
- Task 2 complete: missing category page integration observed RED -> GREEN; mixed Unknown/compiler/target counts and recursive link validation passed.
- Final checks: 65 tests passed, fmt/strict Clippy/MSRV1.94 passed, package verified, real online and offline reports generated. The paired outputs preserve all 51 finding IDs and counts. See docs/validation/2026-11-07-categories.md.
- Final review: independent reviewer found no Important/Critical regressions.
- Final: minor (deferred): add a dedicated deprecated-entry regression test; actual-source extraction already exercised this branch. Actual TLS/timeout failure reproduction remains outside controlled error-injection tests, as recorded.
