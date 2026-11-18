# Rustdoc-style lexical highlighting

Clippy HTML adapts rustdoc's lexical token classes in `src/render/highlight.rs`.
The nightly implementation inspected on 2026-10-28 was the starting point.
The class names were checked against the pinned rustdoc 1.97.1 stylesheet and
locally generated source pages. The adaptation uses rustdoc's MIT option.
Reports include the original notice as `licenses/rustdoc-MIT.txt`.
[Highlighter source](https://doc.rust-lang.org/nightly/nightly-rustc/src/rustdoc/html/highlight.rs.html) · [Module documentation](https://doc.rust-lang.org/beta/nightly-rustc/rustdoc/html/highlight/index.html)

The renderer tokenizes the full file before splitting it into display rows.
This preserves lexical context within multiline literals and nested comments.
It normalizes CRLF for display and uses original bytes when validating
diagnostic coordinates. Askama escapes each fragment, and the highlighter emits
fixed CSS classes. Diagnostic snippets and replacements remain plain text.

The lexer distinguishes keywords, literals, comments, attributes, lifetimes,
macros, and rustdoc's small set of special names. It handles weak keywords in
context. Other names such as `String` remain ordinary identifiers. Unknown
tokens retain their text. The report uses lexical colors only and provides no
compiler-derived definition links, type resolution, or macro expansions.

The project uses `ra-ap-rustc_lexer` 0.178.0, published from Rust commit
`33295d734447c7a2c22c44190f11bce6164a035a`. The locked `unicode-properties`
0.1.4 and pinned `unicode-ident` 1.0.22 both use Unicode 17. The lexer checks
that they match at compile time. Check both tables together when updating these
dependencies. The lexer retains its MIT or Apache-2.0 license. `unicode-ident`
retains its MIT or Apache-2.0 and Unicode-3.0 terms. [Lexer publishing source](https://github.com/rust-analyzer/rustc-auto-publish)
