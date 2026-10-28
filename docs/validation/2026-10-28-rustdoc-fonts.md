# Rustdoc font validation

## Scope and licensing

- The generator code, Cargo.toml and root LICENSE files keep MIT OR Apache-2.0 unchanged. Rustdoc/normalize CSS retains its existing MIT notices; font files retain OFL-1.1 and original copyright/reserved-name notices.
- Default font roles: Fira Sans for UI, Source Serif 4 for body text, Source Code Pro for code and NanumBarunGothic for the original Hangul fallback ranges. Fira Mono is not included.
- 12 WOFF2 files, 1,386,044 bytes total, copied unchanged from this project's rustdoc 1.97.1 output. All 12 selected face declarations match the reference, including weight/style, local(), swap and Hangul unicode-range. Source Code Pro continues to avoid local().
- assets/vendor/rustdoc/FONTS.md records filenames, byte counts and SHA-256 for all fonts and four original license files. The vendored fonts directory disables Git text conversion, including for license texts, to preserve copied bytes.
- THIRD-PARTY-NOTICES.md identifies actual included resources. Original upstream COPYRIGHT is preserved and labeled as reference material, because it mentions other resources too. Generated reports have a linked license overview and original notices; executable release bundles must carry the notices described in THIRD-PARTY-NOTICES.md.

## Automated evidence

- Distribution integration test observed RED (missing bundled WOFF2) before implementation, then GREEN. It checks all 12 emitted files byte-for-byte against the vendor snapshot, WOFF2 signatures, total size, all four original license texts, stylesheet linkage and license overview.
- Missing-font and escaping-font CSS URL regressions observed RED before implementation, then GREEN. Remote HTTPS and data font URLs are additionally rejected. Missing font resources fail staged publication and preserve the old report.
- Supplemental preservation and notice-link tests are coverage, not separate claimed RED/GREEN cycles. The preservation test initially had a test-only Debug constraint error, corrected without changing production types.
- Full suite: 41 unit + 8 integration = 49 passed; fmt, strict Clippy and Rust 1.94 locked check passed.
- cargo package --list --locked --allow-dirty includes fonts.css, all 12 WOFF2 files, all 4 family notices, FONTS.md, THIRD-PARTY-NOTICES.md, root licenses and templates.
- cargo package --locked --allow-dirty packaged and verified the application from the package directory. All 16 packaged font/license files matched vendored SHA-256 values. No font read from target/doc occurs at build or report-generation time after vendoring.
- Cargo reported the existing missing documentation/homepage/repository metadata warning; this task does not change release metadata.

## Manual acceptance still needed

- Open target/doc/cargo_clippy_html/index.html and target/rustdoc-style-preview/index.html side by side, including lint and source pages. Check rendered fonts, rather than relying on computed font-family alone.
- Check body/UI/code roles, italic and emphasis, Korean syllables/jamo with Latin text, line numbers, long lint names and combined-sidebar alignment. Existing sidebar inset/table column rules are unchanged.
- Check file:// in the user's actual browsers, local font requests, offline operation and zoom. Rustdoc's local() policy may select an installed Fira/Source Serif; that matches the reference policy. Source Code Pro comes from the bundled file.
- The available browser surface blocks file URLs; no browser-policy workaround was used and no actual font-loading or visual-parity result is claimed.
- Exact pinned remote commit retrieval was unavailable during planning. Source provenance is the verified local toolchain output and its original notices, with the external source limitation recorded in FONTS.md.
