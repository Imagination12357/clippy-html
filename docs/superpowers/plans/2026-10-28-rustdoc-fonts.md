# Rustdoc fonts implementation plan

**Approved design:** Restore rustdoc 1.97.1 default typography with 12 unmodified WOFF2 files (Fira Sans, Source Serif 4, Source Code Pro, NanumBarunGothic). Keep project code, root LICENSE files and Cargo.toml license = MIT OR Apache-2.0 unchanged. Font files retain OFL-1.1 and their original notices. Fira Mono is not part of the default font stack and is excluded.

**Source:** The project's local cargo-doc reference, rustdoc 1.97.1 commit 8bab26f4f68e0e26f0bb7960be334d5b520ea452. Exact remote commit contents were unavailable to the browsing tool; record that limit, original generated filenames, byte counts and SHA-256 values. Official Rust REUSE annotations and OFL text/FAQ were checked during planning.

**Boundaries:** fonts.css contains original selected @font-face declarations, preserving local(), font-display, weights and Hangul unicode-range. Fonts are binary-embedded, then written once per report. No runtime toolchain font lookup, network request, subsetting, conversion or preload JavaScript. Keep current layout offsets and table columns.

## Tasks

1. [x] Add actual-report font/license distribution and missing/escaping CSS-font-reference regressions; observe RED. Vendor selected bytes and original license texts; record hashes.
2. [x] Move static asset emission into src/render/assets.rs, add include_bytes fonts and font CSS, remove temporary font overrides, and add a license overview listing actual shipped components. Preserve upstream COPYRIGHT as clearly labeled reference material. Validate CSS font URLs locally and reject missing/nonlocal/out-of-report resources.
3. [x] Verify whole suite, fmt, strict Clippy, Rust 1.94, Cargo package contents, binary/source distribution notices and preview generation. Run one independent final review, document evidence and commit after final refresh.

## Files

- assets/vendor/rustdoc/fonts/: 12 original font binaries and 4 original license texts.
- assets/vendor/rustdoc/fonts.css and FONTS.md: selected original declarations and provenance/hashes.
- THIRD-PARTY-NOTICES.md: actual bundled components and applicable license locations; references are distinguished from included resources.
- src/render/assets.rs: static text/binary resource emission and font-resource validation.
- templates/base.html, templates/licenses.html, src/render/pages.rs: stylesheet and license overview integration.
- tests/integration.rs, README.md, existing design/validation docs: real output checks, packaging and user documentation.

## Review focus

- Code/Cargo/root licenses remain unchanged; each font and every distributed report carries its own original notice.
- Missing fonts or license files fail validation before output replacement; nested pages reach shared resources.
- Local URL handling never fetches remote fonts or permits path escape; font bytes and metadata are unchanged.
- Artifact builds do not depend on target/doc once the assets are vendored; Cargo packages include every asset/notice required for installation.
- Default CSS font roles and Unicode range match the reference; rendered-font verification on file:// remains manual because browser automation blocks that protocol.

## Progress

- Workspace: codex/phase-1, clean before work. User authorizes implementation and autonomous commits. Windows ledger is kept here instead of Bash-only skill scripts.
- Interfaces: asset writer -> renderer -> staged publication uses Result<(), String>; binary data must not pass through UTF-8 conversion. Existing HTML link validation will validate the new license overview; CSS font URL checks extend it.
- Task 1: actual font distribution, missing CSS font and escaping CSS font regressions observed RED; copied 12 fonts and 4 licenses with original and destination SHA-256 equality. Added scoped -text attributes to preserve their bytes through checkout.
- Task 2: original font roles restored, separate font licenses and actual asset overview added, static resources embedded, and staged CSS font URL validation connected. Initial 48-test suite GREEN; supplementary preservation/notice tests added afterward, not claimed as independent RED/GREEN cycles.
- Task 3: 41 unit + 8 integration = 49 passed; fmt, strict Clippy and Rust 1.94 checks passed. Cargo package --list confirmed required assets; cargo package --locked --allow-dirty packaged and verified the binary from packaged source. All 16 packaged font/notice SHA-256 values match the vendor snapshot. Existing missing package URL metadata warning is recorded, not changed.
- Independent final review: no Critical, Important or minor findings. Reviewer independently confirmed font/notice hashes and exact selected face declarations; no additional fix pass needed.
- Scope/limits: own code/Cargo/root licenses stay unchanged; local() policy matches pinned rustdoc. No new preload JS, configuration, font conversion, subsetting, runtime source lookup, push or release. Browser typography acceptance remains manual; exact remote commit provenance remains unverified and explicitly documented.
