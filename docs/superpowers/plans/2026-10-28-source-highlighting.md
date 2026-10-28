# Rustdoc-style source highlighting plan

**Goal:** Generate Rust source syntax colors using rustdoc's token-to-CSS-class approach while preserving existing line numbers, anchors and diagnostic markers.

**Investigation:** Rustdoc html/highlight.rs tokenizes on the generation side, classifies tokens and emits classes already supported by our pinned rustdoc CSS. Its complete renderer depends on rustc_span, rustc compiler context, definition links and macro-expansion machinery. This tool has no compiler context, so adapt the lexical classifier and existing class names, replacing the compiler-private lexer dependency with the stable published ra-ap-rustc_lexer 0.178.0 (Rust source commit 33295d734447c7a2c22c44190f11bce6164a035a, MIT/Apache-2.0). Verify Rust 1.94 rather than assuming MSRV compatibility.

**Reference:** https://doc.rust-lang.org/nightly/nightly-rustc/src/rustdoc/html/highlight.rs.html (inspected on 2026-10-28; nightly reference is not claimed to be the exact pinned CSS commit). Source of CSS remains rustdoc 1.97.1. A generic highlighter would require mapping another grammar/theme; compiler-private linking would tie installation to rustc internals. Selected approach preserves the existing CSS and standalone static generation.

**Boundary:** Syntax classes for keywords, reference/mut modifiers, strings/chars/byte/C/raw literals, numbers, booleans, comments/doc comments, lifetimes, attributes, macro invocations/metavariables, self and rustdoc's special Option/Result/Some/None/Ok/Err names. Other identifiers, including String, remain ordinary identifiers as in the inspected rustdoc classifier. Definition links and macro expansion are outside scope.

**Safety and rendering:** Tokenize the whole file once to retain multiline state, then split fragments at line boundaries. Normalize CRLF only for display; validate diagnostic coordinates against original source. Pass fragment text through Askama escaping, with fixed internal CSS classes; no raw source HTML or client-side scripts. Preserve source text, empty/trailing lines, malformed source and source-unavailable fallback. Color the full source view; keep diagnostic snippet/fallback rendering unchanged in this task.

## Tasks

1. [x] Add real generated-source and lexical-fragment tests; observe current missing-color failures. Pin lexer and verify its license/API and MSRV.
2. [x] Implement src/render/highlight.rs, integrate escaped fragments into source_rows/source template, preserve multiline and navigation behavior. Keep CSS/theme definitions unchanged.
3. [x] Test whole suite, strict Clippy, fmt, Rust 1.94 and packaged build; compare generated spans with local rustdoc, update provenance/docs, regenerate previews, independently review and commit after final refresh.

## Review focus

- Source text survives highlighting exactly after CRLF normalization; HTML-significant characters cannot escape the code element.
- Multiline raw strings/nested block comments are not tokenized anew per line; attributes/macros do not consume unrelated following code.
- UTF-8 offsets, raw identifiers, lifetimes vs chars, literals with prefixes and unterminated input remain safe.
- Existing L anchors, diagnostic line markers, source tree and lint links remain intact.
- Work and allocations scale with input/output size; lookahead must not repeatedly rescan long trivia sequences.

## Progress

- Workspace codex/phase-1; user authorizes investigation and port, with autonomous commits. Existing code/Cargo license stays MIT OR Apache-2.0. No source-browser JS or compiler-private dependency is introduced.
- Interface: highlighter -> per-line escaped fragments -> SourceRow -> Askama; diagnostic byte-coordinate validation stays on original SourceFile text. Windows tracked ledger used instead of Bash-only skill scripts.
- Investigation/decision: port the lexical classification and original class names using the standalone lexer; compiler-private definition-link and macro-expansion machinery has no equivalent report context and is omitted. Cost: syntax coloring has no type resolution and is not a complete compiler-context rustdoc renderer.
- Dependency evidence: initial lexer build failed because unicode-ident 1.0.27 uses Unicode 18 but unicode-properties 0.1.4 uses Unicode 17. Pin ident 1.0.22; Rust 1.94 check and strict Clippy passed with the resolved pair. Cargo.lock changes only the new lexer/properties and ident version.
- RED/GREEN: generated-source missing keyword color and three lexical-category tests failed before implementation; multiline/raw/comment/attribute cases then passed. Pointer const classification failed an extra regression and was corrected to rustdoc's kw-2 category.
- Supplementary real-source safety/roundtrip test passed text/escaping/color checks but initially assumed exactly one marked diagnostic line; existing display spans can mark more than one row. The next assertion had an incorrectly hand-counted line 9. Corrected it to check the specific clone_on_copy marker on actual source line 8, without constraining other marked rows. No production change was needed for these test assumptions.
- Independent final review: no Important/Critical regressions. Nonblocking note: a future unicode-properties patch can change Unicode tables. Narrowed the provenance documentation to the tested locked combination and recorded coordinated-update requirements; no guarantee about all future registry resolutions is made.
- Final scope: full-source lexical colors only; compiler semantic links, macro expansion and snippet coloring are outside this port. Browser visual validation remains manual due file URL policy. Current source span/anchor/escaping tests and font/license distribution remain required.
- Final checks: 46 unit + 9 integration = 55 passed; fmt --check, strict Clippy and Rust 1.94 locked check passed. cargo package --locked --allow-dirty packaged and verified the current binary (existing missing package URL metadata warning only). Both report previews and comparison docs regenerated. Static comparison of the highlighter's own source found the same used class set in the report and native rustdoc: kw, kw-2, string, number, comment, doccomment, macro, lifetime, prelude-ty, prelude-val, attr, bool-val. This is class-level evidence, not a browser pixel comparison.
