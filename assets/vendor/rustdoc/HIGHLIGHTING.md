# Rustdoc-style lexical highlighting

Investigated on 2026-10-28:

- Rustdoc module documentation: https://doc.rust-lang.org/beta/nightly-rustc/rustdoc/html/highlight/index.html
- Inspected implementation: https://doc.rust-lang.org/nightly/nightly-rustc/src/rustdoc/html/highlight.rs.html
- Stable lexer distribution: ra-ap-rustc_lexer 0.178.0, automatically published from Rust commit 33295d734447c7a2c22c44190f11bce6164a035a by https://github.com/rust-analyzer/rustc-auto-publish .

The inspected highlighter uses compiler lexing, a Class-to-CSS mapping, and HTML token emission. It also supports compiler-context definition links and macro expansion. This project adapts the lexical classification and CSS names in src/render/highlight.rs; it does not link rustc-private libraries or copy the compiler-context renderer. Upstream Rust portions are used under the MIT option; the original notice is retained in LICENSE-MIT and emitted as licenses/rustdoc-MIT.txt.

The browsed nightly source is the investigation reference, not claimed to be the exact commit of the pinned rustdoc 1.97.1 CSS. The class names used here are verified against that pinned CSS and local generated source HTML.

Behavior:

- Tokenize an entire source file, then split fragments into display rows. Multiline literals and nested comments retain lexical context. CRLF is normalized only for display; diagnostic coordinate checks still use original source bytes.
- Source text remains text. Askama escapes each fragment; classification only supplies fixed CSS names. No raw HTML fragment or highlighting JavaScript is needed.
- Preserve whitespace, unknown/malformed input, raw identifiers, lifetime/char distinctions and source line anchors.
- Highlight keywords, reference/mut modifiers, literals, numeric literals, booleans, comments/doc comments, attributes, lifetimes, macros/metavariables, self and rustdoc's small special-name set. Other identifiers such as String remain ordinary identifiers; this is lexical rather than semantic highlighting.
- Keyword recognition uses a fixed modern Rust keyword list plus contextual handling for weak keywords. Compiler edition internals, type resolution, definition links, macro expansion and all future nightly syntax are not reproduced. Unknown token variants fall back to unclassified text.
- Diagnostic snippets/replacements stay escaped plain text in this task; the full source viewer receives syntax colors.

The lexer currently resolves unicode-properties 0.1.4 (Unicode 17) and asserts that unicode-ident uses the same Unicode version. unicode-ident is pinned to 1.0.22 (Unicode 17), because a previously selected 1.0.27 uses Unicode 18 and causes the lexer to fail at compile time. This manifest constraint prevents that observed mismatch with the currently resolved unicode-properties version; Cargo.lock fixes the tested combination. unicode-properties remains a transitive semver dependency, so future dependency updates must recheck both Unicode tables and the lexer together. Fresh resolution with all possible future releases is not claimed.

The lexer is consumed as a Cargo dependency under its MIT/Apache-2.0 license; unicode-ident retains its own MIT/Apache-2.0 and Unicode-3.0 terms. Existing project/root/Cargo licensing is unchanged. This document covers the highlighting implementation and does not replace the dependency licenses.
