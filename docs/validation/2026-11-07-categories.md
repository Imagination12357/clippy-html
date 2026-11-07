# Official categories validation — 2026-11-07

## Implemented behavior

- Default online: select one official version/channel from diagnostic help notes; retrieve the canonical HTTPS page once. No Clippy findings means no fetch.
- Tool --offline or retrieval/parse failure uses the complete bundled rust-1.97.0 table by name. Forwarded Cargo --offline is independent. No persistent cache, silent latest substitution or per-name online/bundle mixing.
- 815 active entries extracted with the shared Rust parser/maintenance tool from the official 1.97.0 HTML. Sixteen deprecated entries are excluded. Original MIT notice and source/hash information accompany the bundle and generated license overview.
- Category lists/counts and category -> lint -> source navigation reuse existing tables and page shell. Unknown is a Clippy bucket; rustc/general are not applicable. Counts use deduplicated findings, not targets or span occurrences.
- Category enrichment happens after fingerprint assignment; ID tests cover different source tables and fallback modes. Metadata failures do not change diagnostic Execution or exit status.

## Evidence so far

- New offline CLI regression failed before implementation; all four tool/Cargo combinations now parse/forward independently.
- Official parser regression failed before implementation and now handles the observed table structure and rejects malformed/conflicting records.
- New actual-report category navigation test failed at the missing category page before rendering changes; now passes with relative category/lint links and bundled provenance.
- Controlled fetch tests cover failure fallback (timeout/404/TLS/body-limit error values), missing/conflicting version selection, zero-fetch paths, online-only missing names and ID preservation. These test resolver behavior rather than emulating every HTTP/TLS library failure on a public server.
- Transport reads are bounded to 8 MiB, TLS verification is enabled, HTTPS-only requests, a 10-second global network timeout and no automatic redirects. A real official online fetch succeeded; the report records online rust-1.97.0 and its source/table fingerprints.
- Synthetic mixed-category test validates Unknown vs compiler/general separation, multi-target deduplication, counts and recursive relative-link validation.
- Rust 1.94 locked compile check passed with ureq 3.5.1 and scraper 0.24.

## Final checks

- `cargo test --locked`: 55 unit + 10 integration tests passed (65 total). The later strengthened two-version selector/ID assertion also passed its targeted rerun; it changes tests only.
- `cargo clippy --all-targets --locked -- -D warnings` and `cargo fmt --check`: passed. `cargo +1.94.0 check --locked`: passed.
- `cargo package --locked --allow-dirty`: packaged 97 files, 1.9 MiB (1.5 MiB compressed), then built the packaged crate successfully. Existing missing documentation/homepage/repository metadata warning only.
- Actual online report: target/rustdoc-style-preview/index.html, provenance online rust-1.97.0. Offline report: target/categories-offline-preview/index.html, provenance bundled rust-1.97.0 with explicit offline reason. Both generated successfully through publication's recursive link/anchor validation.
- Parsed both generated outputs: 32 HTML pages each, identical sets of 51 unique finding IDs, with 51 pedantic findings counted consistently on home, category and lint pages. Fixture and synthetic tests cover complexity, Unknown, compiler/general and multi-target cases not represented by this self-report.
- Online source fingerprint matches the bundled snapshot source fingerprint: blake3:d24d10832bdbc3f19b96d4ff6f8db8f343830d0e05858578ecdfe33a7dcc696e. Online table fingerprint: blake3:6e50484614a3879e3a2ba2c3153cecbf1ce6951e001fecd17c37642091948d10.
- Independent read-only review: no Important or Critical regressions. Minor coverage note: deprecated exclusion was exercised by the actual 831-record extraction producing 815 active mappings, but lacks a dedicated small regression test; deferred. Controlled transport-error injection is not a claim to have reproduced real TLS/timeout failures.

## Limits

Public network availability and upstream HTML structure can change; both trigger an explicit bundled fallback. Moving main/master selectors identify a moving snapshot, not an exact nightly build. Missing diagnostic documentation links also use the bundle. Automatic redirects are disabled by design.

No actual browser tab/visual review or Linux/macOS execution is claimed. Static HTML and links are checked; the existing file-URL automation restriction remains. No push, merge or publication is part of this change.
