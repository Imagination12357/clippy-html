# Readable lint URL plan

**Approved decision:** A1: `lint/clippy/clone_on_copy.html`, `lint/rustc/E0308.html`, `lint/rustc/unused_variables.html`; code-less diagnostics retain `lint/general.html`.

**Design:** PageMap remains the only path authority. Encode individual lint names with the existing filename protection. Share the source-page collision allocator with lint paths so Windows case aliases, reserved names, shortened components and generated suffixes cannot overwrite one another. Keep finding IDs and grouping semantics unchanged. Calculate lint-page root links from actual depth. No redirect copies for obsolete hashed URLs.

1. Add readable-path, namespace, unsafe-name, case-collision and input-order regressions; observe failures.
2. Replace hash candidates, reuse collision assignment, and update lint root links.
3. Verify real Clippy/compiler output, full tests, fmt, strict Clippy and Rust 1.94; refresh previews and commit.

**Compatibility:** Regenerated report links change; old externally saved hash URLs are not retained. Internal links and finding anchors are regenerated consistently. Browser visual confirmation remains manual.

## Results

- Readable lint naming unit test and actual Clippy integration regression observed RED against hashed filenames, then GREEN. Actual compiler-error fixture confirms `lint/rustc/E0308.html` while preserving its failing exit code.
- Coverage includes Clippy/compiler namespace separation, code-less general page, reserved devices, unsafe path characters, ASCII/Unicode case aliases and input-order independence.
- Strengthening the forced collision test exposed Windows separator spelling differences after `set_file_name`. Collision keys now normalize separators before Unicode case folding; the test reserves an original name equal to a generated suffix and verifies neither is overwritten.
- Shared the existing collision allocator between source and lint mappings. Source naming regressions continue to pass. Lint-page assets/home/license links use actual page depth; generated report validation checks all internal links and anchors recursively.
- cargo test --locked: 38 unit + 7 integration = 45 passed. cargo clippy --all-targets --locked -- -D warnings and cargo +1.94.0 check --locked passed. Final fmt and report preview refresh follow before commit.
