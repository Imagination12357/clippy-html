# Rustdoc/report comparison — 2026-10-30

## Reference and scope

Reference: this repository's own `cargo doc --no-deps --document-private-items --locked`, rustdoc 1.97.1, commit 8bab26f4f68e0e26f0bb7960be334d5b520ea452. Report: `cargo run --locked -- --output target/rustdoc-style-preview -- --workspace --all-targets -- -W clippy::pedantic`.

This is an element-by-element comparison of generated HTML, CSS, local font assets and source text/classification. It is not a claim of pixel-identical rendering or executed rustdoc sidebar JavaScript. Browser file-URL automation remains blocked; no server or alternate-browser workaround was attempted.

## Paired outputs

| Native rustdoc output | Report output | Compare |
|---|---|---|
| target/doc/cargo_clippy_html/index.html | target/rustdoc-style-preview/index.html | Page shell, crate/version, section headings, item lists |
| target/doc/cargo_clippy_html/render/pages/struct.FindingView.html | target/rustdoc-style-preview/lint/clippy/*.html | Nested-page breadcrumbs, headings, code blocks and section anchors; content types differ |
| target/doc/src/cargo_clippy_html/main.rs.html | target/rustdoc-style-preview/src/main.rs.html | Source shell, gutter, code tokens and sidebar |
| target/doc/src/cargo_clippy_html/{cargo,cli,source}.rs.html and diagnostic/normalize.rs.html and render/{assets,highlight,pages,paths}.rs.html | Matching paths under target/rustdoc-style-preview/src/ | All nine shared source files and source text |
| target/doc/static.files/rustdoc-17e0aaed.css and font files | assets/vendor/rustdoc/ and report static.files/ | Pinned resource identity, declarations and extra CSS |

Rustdoc has no direct equivalent of a diagnostic finding/category/status page. The paired item page supplies shared structural conventions, not a reason to copy Rust API-specific UI.

## Findings and decisions

| Element | Native behavior | Report decision |
|---|---|---|
| CSS snapshot | rustdoc-17e0aaed.css | Same pinned base. Original SHA256 is 264dc5a58dbfaf73bca6f1b5767413c775d1150c07855f6b399e8672ae3d9597; documented removal of font faces/clipboard/checkmark properties remains |
| Fonts | Fira Sans/Source Serif 4/Source Code Pro/NanumBarunGothic | All 12 selected font declarations match the native faces after whitespace normalization; all 12 font files match byte for byte. Excluded Fira Mono remains unused |
| Main shell | main > width-limiter > section.content; source omits width limiter | Already shared; no replacement shell needed |
| Header | main-heading, breadcrumbs/h1; native toolbar and API source links | Keep shared layout. Keep report/source titles; omit inert copy/API buttons and rustdoc toolbar |
| Crate/version | .sidebar-crate h2 .version, 1rem, normal weight | Remove report-only .875rem/grey crate-version override. Let native .version supply sizing/color |
| Crate identity | Documentation crate name and one version | Keep Clippy HTML product name plus Cargo package name/version context; workspaces can have multiple packages |
| Item lists | dl.item-table with dt/dd and shared responsive column rules | Already shared. Preserve user-requested common alignment; no lint-specific wider column |
| Directory tree | src-script creates details.dir-entry, summary, folders then files, a.selected; opens current ancestors | Report already emits the matching native structure statically. JS inspection establishes shape, not runtime browser parity |
| Combined sidebar | Native docs and source pages have distinct sidebars | Keep report's combined product/package, Files and Findings sections. Preserve the explicitly approved source-files inset |
| File inventory | Rustdoc source tree includes documented crate files | Report tree intentionally contains diagnostic-referenced source files and per-file finding counts |
| Source rows | One pre/code stream, direct line-number anchors and newlines | Keep block code-line wrappers for diagnostic row coloring/markers and escaped line-text fragments. Native data-nosnippet supplies gutter spacing |
| Source whitespace | pre preserves whitespace | Remove redundant .source-code .line-text white-space:pre declaration; inheritance already provides it |
| Source scroll | Native source layout assumes its own source viewer | Keep source-scroll overflow for the report's combined source view; not redundant with native non-source-only pre overflow rule |
| Line URLs | Numeric #1 and rustdoc range selection driven by JS | Keep stable #L1 report anchors and plain relative links; do not break finding URLs just to copy native IDs |
| Lexical colors | Native underscore is kw | Correct missing `_` keyword classification, including inferred types and ignored patterns, preserving _name/string/comment behavior |
| Qualified macros | Native colors serde_json::json! including path prefix | Report colors the invocation name/bang but leaves serde_json:: plain. Keep as documented lexical cosmetic limitation; no semantic/path-tracking expansion in this audit |
| Diagnostic UI | No native finding status/severity/suggestion equivalent | Keep execution/completeness, severity, origin/category, notes, locations, grouped replacements and fallback snippets |
| Small screens/no JS | Native toolbar/sidebar runtime; native noscript CSS hides source sidebar | Do not copy noscript.css wholesale: it would hide the required file/finding navigation. Retain report's static mobile sidebar |
| Theme | Native runtime/storage plus optional no-script system preference | Keep explicit current light theme; Light/Dark/Auto is a later complete milestone, not a partial toolbar copy |
| Focus/access | Report adds focus outline, skip link and accessible navigation labels | Keep these additions; behavioral keyboard/browser acceptance is still pending |
| Icons/notices | Native Rust favicon and upstream notices | Keep the approved Clippy HTML gear/clip favicon and actual bundled-component license overview |

## Source comparison method

Parse native `pre.rust code`, remove only its `a[data-nosnippet]` line-number elements; compare its text with the newline-joined report `.line-text` nodes. All nine shared files matched; ignore only a possible final newline when comparing, not source characters or internal whitespace. Compare classification per non-whitespace character, walking inherited syntax classes in each DOM. Before the correction, differences were underscore kw and qualified macro prefixes; after the correction, only the qualified macro-prefix differences should remain for this corpus. This is a comparison of these files, not proof of every Rust token/context.

## Changes and verification

- No navigation, page naming, table widths, JavaScript, dependency or licensing-policy changes.
- Removed two unnecessary CSS overrides and corrected the missing underscore keyword class.
- `underscore_patterns_and_inferred_types_use_rustdoc_keyword_class`: RED (0 classified underscores instead of 2) before the correction; GREEN afterward, including negative string/comment/identifier checks.
- Updated ROADMAP.md, README status and the original design's current-status pointers; earlier plans/ledgers remain historical.
- `cargo test --locked`: 47 unit + 9 integration tests passed (56 total).
- `cargo fmt --check` and `cargo clippy --all-targets --locked -- -D warnings`: passed after formatting the added keyword entry.
- Regenerated both cargo docs and the workspace/all-targets report after the correction. All nine paired source texts still match. All selected font declarations/files still match; the original reference CSS hash is unchanged.
- Of the compared non-whitespace syntax classes, only qualified macro path prefixes differ: 36 characters in cargo.rs and 12 each in diagnostic/normalize.rs and render/pages.rs. This count describes the current source corpus, not a general semantic-equivalence guarantee.
- Report publication's recursive internal-link/anchor validation passed; supplementary output inspection confirmed both favicon paths on all 25 HTML pages and emitted icon/notice byte equality.
- Independent read-only review found no Important/Critical code, scope or roadmap issues. Its request to replace pending verification/status placeholders is reflected in this final record and plan.

## Manual checks still outstanding

Actual side-by-side browser rendering in Chrome/Edge and Firefox, font loading/fallback, focus/keyboard navigation, long-source horizontal scrolling, narrow layouts and static-host repository subpaths. DOM/CSS comparison does not establish these. The roadmap keeps category/search first among remaining product features and brings baseline platform CI forward separately.
