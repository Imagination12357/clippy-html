# Source highlighting validation

## Implementation and provenance

- Investigated rustdoc's generation-side token classification and CSS classes in official html/highlight.rs documentation. The full upstream renderer requires compiler-private context for definition links and macro expansion; this implementation adapts lexical coloring to a standalone published Rust lexer.
- Source references, selected MIT notice, dependency source commit and supported scope are in assets/vendor/rustdoc/HIGHLIGHTING.md. Reports include that record under licenses/ and link it from the license overview. Existing code/Cargo/root licensing remains unchanged.
- Uses ra-ap-rustc_lexer 0.178.0 and unicode-ident 1.0.22. Initial compilation with ident 1.0.27 failed the lexer's Unicode 17/18 table consistency assertion. The locked pair with unicode-properties 0.1.4 builds on stable and Rust 1.94. Future dependency updates must recheck both Unicode tables; arbitrary future fresh resolutions are not guaranteed.
- Whole-file tokens preserve nested block comments, multiline raw/string literals and UTF-8 boundaries. Fragments are split only after lexing; diagnostic matching still uses original source bytes. Display normalizes CRLF.
- Fragment text is escaped by Askama. Only internally selected classes become attributes, and raw source HTML is never inserted. No client-side highlighter or new theme CSS is introduced.

## Test evidence

- Initial actual-source keyword-color test and three lexical-category tests failed before implementation. After whole-file classification, keyword/literal/comment/attribute cases pass.
- Pointer const coloring failed a targeted regression, then was corrected to rustdoc's kw-2 category. Logical != and if ! are not treated as macro calls; weak union is not highlighted as a keyword in variable position.
- Tests cover raw identifiers, lifetimes vs Unicode chars, byte/C/raw literals, booleans, numbers, macros/metavariables, self and special Option/Result/Some names. Ordinary String identifiers are deliberately not given semantic type colors, matching the inspected rustdoc rule.
- Empty/trailing lines, BOM/Unicode, CRLF and incomplete source roundtrip through lexical fragments. Actual generated-source DOM text also matches original source after CRLF normalization.
- An actual Clippy fixture contains a multiline raw string with hostile HTML and fake Rust syntax, plus nested comments. No script/event element is created; fake fn remains string text, real fn is colored, and the clone_on_copy marker stays on source line 8. Supplemental assertions initially overconstrained marked-line counts and miscounted line 9; corrected test assumptions required no production change.
- Existing span validation, source tree, source/lint links, escaping, font/license distribution and safe report publication tests remain included.
- Final suite: 46 unit + 9 integration = 55 passed. fmt --check, strict Clippy and Rust 1.94 locked check passed. Cargo package build verified standalone source packaging with the lexer dependency. Both report previews were regenerated successfully with internal link/anchor validation.
- Static comparison on render/highlight.rs found the same used token-class set in report and native rustdoc output: kw, kw-2, string, number, comment, doccomment, macro, lifetime, prelude-ty, prelude-val, attr and bool-val. This does not establish identical classification at every source position or live visual parity.

## Limits and manual checks

- Syntax classes are reused from the pinned rustdoc CSS; the investigation's nightly highlighter source is not claimed to match that CSS commit byte-for-byte. There is no semantic type/name resolution, definition-link navigation, macro expansion or exact parity for all nightly syntax.
- This task colors full source views. Diagnostic snippets, replacements and rendered compiler fallback remain escaped plain text.
- Compare target/rustdoc-style-preview/src/main.rs.html with target/doc/src/cargo_clippy_html/main.rs.html. Check keywords, strings, comments, lifetimes, attributes and macros along with line numbers and diagnostic backgrounds. Span grouping can differ while colors match.
- Available browser automation blocks file URLs. Generated HTML/class/text and relative-link checks are automated; live browser color/layout comparison remains manual.

## Independent review

Read-only independent review found no Important/Critical regressions. A nonblocking note concerned future unicode-properties releases changing Unicode tables. Documentation was narrowed to the verified locked dependency combination and coordinated-update requirements; current locked builds are verified.
