//! Apply rustdoc syntax classes and diagnostic ranges to plain source text.
// Classification follows rustdoc html/highlight.rs; see HIGHLIGHTING.md for
// source references, selected MIT licensing and compiler-context limitations.
use ra_ap_rustc_lexer::{FrontmatterAllowed, LiteralKind, TokenKind, strip_shebang, tokenize};
use std::collections::BTreeSet;

pub(super) struct SyntaxFragment {
    pub text: String,
    pub class: Option<&'static str>,
}

struct Token<'a> {
    kind: TokenKind,
    text: &'a str,
}
fn trivia(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Whitespace | TokenKind::LineComment { .. } | TokenKind::BlockComment { .. }
    )
}
fn keyword(text: &str, next: Option<&Token<'_>>) -> bool {
    matches!(
        text,
        "_" | "as"
            | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "pub"
            | "return"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "gen"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    ) || match text {
        "union" => next.is_some_and(|t| t.kind == TokenKind::Ident),
        "auto" => next.is_some_and(|t| t.text == "trait"),
        "raw" => next.is_some_and(|t| matches!(t.text, "const" | "mut")),
        "safe" => next.is_some_and(|t| t.text == "fn"),
        "default" => next.is_some_and(|t| {
            matches!(
                t.text,
                "fn" | "type" | "const" | "impl" | "unsafe" | "pub" | "async" | "extern"
            )
        }),
        _ => false,
    }
}
fn classify(tokens: &[Token<'_>]) -> Vec<Option<&'static str>> {
    let mut following = vec![None; tokens.len()];
    let mut significant = None;
    for i in (0..tokens.len()).rev() {
        following[i] = significant;
        if !trivia(tokens[i].kind) {
            significant = Some(i);
        }
    }
    let mut classes = Vec::with_capacity(tokens.len());
    let mut previous_significant: Option<usize> = None;
    let mut attribute_depth = 0usize;
    let mut pending_attribute = false;
    for (i, token) in tokens.iter().enumerate() {
        let next = following[i].map(|j| &tokens[j]);
        let adjacent_next = tokens.get(i + 1);
        let adjacent_previous = i.checked_sub(1).map(|j| &tokens[j]);
        if token.kind == TokenKind::Pound && attribute_depth == 0 {
            pending_attribute = following[i].is_some_and(|j| {
                tokens[j].kind == TokenKind::OpenBracket
                    || (tokens[j].kind == TokenKind::Bang
                        && following[j].is_some_and(|k| tokens[k].kind == TokenKind::OpenBracket))
            });
        }
        if token.kind == TokenKind::OpenBracket && (pending_attribute || attribute_depth > 0) {
            attribute_depth += 1;
            pending_attribute = false;
        }
        let class = match token.kind {
            TokenKind::LineComment { doc_style } | TokenKind::BlockComment { doc_style, .. } => {
                Some(if doc_style.is_some() {
                    "doccomment"
                } else {
                    "comment"
                })
            }
            TokenKind::Frontmatter { .. } => Some("comment"),
            TokenKind::Literal {
                kind: LiteralKind::Int { .. } | LiteralKind::Float { .. },
                ..
            } => Some("number"),
            TokenKind::Literal { .. } => Some("string"),
            TokenKind::Lifetime { .. }
            | TokenKind::RawLifetime
            | TokenKind::UnknownPrefixLifetime => Some("lifetime"),
            TokenKind::Question => Some("question-mark"),
            TokenKind::Ident | TokenKind::RawIdent => {
                let macro_name = next.is_some_and(|t| t.kind == TokenKind::Bang)
                    && !following[i]
                        .and_then(|j| tokens.get(j + 1))
                        .is_some_and(|t| t.kind == TokenKind::Eq);
                if previous_significant.is_some_and(|j| tokens[j].kind == TokenKind::Dollar) {
                    Some("macro-nonterminal")
                } else if token.kind == TokenKind::RawIdent {
                    macro_name.then_some("macro")
                } else {
                    match token.text {
                        "ref" | "mut" => Some("kw-2"),
                        "const" if adjacent_previous.is_some_and(|t| t.kind == TokenKind::Star) => {
                            Some("kw-2")
                        }
                        "self" | "Self" => Some("self"),
                        "true" | "false" => Some("bool-val"),
                        "Option" | "Result" => Some("prelude-ty"),
                        "Some" | "None" | "Ok" | "Err" => Some("prelude-val"),
                        text if keyword(text, next) => Some("kw"),
                        _ => macro_name.then_some("macro"),
                    }
                }
            }
            TokenKind::Bang
                if previous_significant.is_some_and(|j| classes[j] == Some("macro")) =>
            {
                Some("macro")
            }
            TokenKind::Dollar
                if next
                    .is_some_and(|t| matches!(t.kind, TokenKind::Ident | TokenKind::RawIdent)) =>
            {
                Some("macro-nonterminal")
            }
            TokenKind::And
                if !adjacent_next.is_some_and(|t| {
                    matches!(
                        t.kind,
                        TokenKind::And | TokenKind::Eq | TokenKind::Whitespace
                    )
                }) && !adjacent_previous.is_some_and(|t| t.kind == TokenKind::And) =>
            {
                Some("kw-2")
            }
            TokenKind::Star
                if !adjacent_next
                    .is_some_and(|t| matches!(t.kind, TokenKind::Eq | TokenKind::Whitespace)) =>
            {
                Some("kw-2")
            }
            _ => None,
        }
        .or_else(|| {
            ((attribute_depth > 0 || pending_attribute) && !trivia(token.kind)).then_some("attr")
        });
        classes.push(class);
        if token.kind == TokenKind::CloseBracket && attribute_depth > 0 {
            attribute_depth -= 1;
        }
        if !trivia(token.kind) {
            previous_significant = Some(i);
        }
    }
    classes
}
fn append(rows: &mut Vec<Vec<SyntaxFragment>>, text: &str, class: Option<&'static str>) {
    for (i, part) in text.split('\n').enumerate() {
        if i > 0 {
            rows.push(Vec::new());
        }
        if !part.is_empty() {
            rows.last_mut().expect("initial row").push(SyntaxFragment {
                text: part.into(),
                class,
            });
        }
    }
}
pub(super) fn lines(source: &str) -> Vec<Vec<SyntaxFragment>> {
    let source = source.replace("\r\n", "\n");
    let mut rows = vec![Vec::new()];
    let shebang = strip_shebang(&source).unwrap_or(0);
    append(&mut rows, &source[..shebang], Some("comment"));
    let mut offset = shebang;
    let tokens: Vec<_> = tokenize(&source[shebang..], FrontmatterAllowed::No)
        .map(|token| {
            let text = &source[offset..offset + token.len as usize];
            offset += token.len as usize;
            Token {
                kind: token.kind,
                text,
            }
        })
        .collect();
    for (token, class) in tokens.iter().zip(classify(&tokens)) {
        append(&mut rows, token.text, class);
    }
    rows
}

pub(super) struct Range {
    pub start: usize,
    pub end: usize,
    pub primary: bool,
    pub title: String,
}
pub(super) struct AnnotatedFragment {
    pub text: String,
    pub class: String,
    pub title: String,
    pub insertion: bool,
}

// Split lexical fragments at validated diagnostic boundaries, preserving text.
pub(super) fn annotate(fragments: Vec<SyntaxFragment>, ranges: &[Range]) -> Vec<AnnotatedFragment> {
    let mut text = String::new();
    let mut syntax = vec![];
    let mut boundaries = BTreeSet::from([0]);
    for fragment in fragments {
        let start = text.len();
        text.push_str(&fragment.text);
        syntax.push((start, text.len(), fragment.class));
        boundaries.insert(text.len());
    }
    // The caller validates spans against source before projecting UTF-8 offsets.
    for range in ranges {
        boundaries.extend([range.start, range.end]);
    }
    let boundaries: Vec<_> = boundaries.into_iter().collect();
    let mut result = vec![];
    let mut token = 0;
    for (i, &start) in boundaries.iter().enumerate() {
        let carets: Vec<_> = ranges
            .iter()
            .filter(|r| r.start == start && r.end == start)
            .collect();
        if !carets.is_empty() {
            result.push(marked("", None, &carets, true));
        }
        let Some(&end) = boundaries.get(i + 1) else {
            continue;
        };
        while syntax.get(token).is_some_and(|s| s.1 <= start) {
            token += 1;
        }
        let class = syntax.get(token).and_then(|s| s.2);
        let active: Vec<_> = ranges
            .iter()
            .filter(|r| r.start < end && r.end > start)
            .collect();
        result.push(marked(&text[start..end], class, &active, false));
    }
    result
}
fn marked(
    text: &str,
    class: Option<&str>,
    ranges: &[&Range],
    insertion: bool,
) -> AnnotatedFragment {
    let mut class = class.unwrap_or_default().to_owned();
    if !ranges.is_empty() {
        class.push_str(if ranges.iter().any(|r| r.primary) {
            " diagnostic-span primary"
        } else {
            " diagnostic-span secondary"
        });
        if insertion {
            class.push_str(" insertion");
        }
    }
    AnnotatedFragment {
        text: text.into(),
        class: class.trim().into(),
        title: ranges
            .iter()
            .map(|r| r.title.as_str())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .join("\n"),
        insertion,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn underscore_patterns_and_inferred_types_use_rustdoc_keyword_class() {
        let source = "let _: Option<_> = None; let _name = 1; let s = \"_\"; // _\n";
        assert_eq!(
            classified(source, "kw")
                .iter()
                .filter(|text| text.as_str() == "_")
                .count(),
            2
        );
        assert!(!classified(source, "kw").contains(&"_name".into()));
        assert_eq!(classified(source, "string"), ["\"_\""]);
        assert_eq!(classified(source, "comment"), ["// _"]);
    }
    #[test]
    fn pointer_modifiers_and_logical_operators_keep_rustdoc_categories() {
        let source = "let p: *const u8; let r: &mut u8; if !true {} let x = a != b; let union = 1;";
        assert!(classified(source, "kw-2").contains(&"const".into()));
        assert!(classified(source, "kw-2").contains(&"mut".into()));
        assert!(!classified(source, "macro").contains(&"if".into()));
        assert!(!classified(source, "macro").contains(&"a".into()));
        assert!(!classified(source, "kw").contains(&"union".into()));
    }
    fn classified(source: &str, class: &str) -> Vec<String> {
        lines(source)
            .into_iter()
            .flatten()
            .filter(|fragment| fragment.class == Some(class))
            .map(|fragment| fragment.text)
            .collect()
    }
    #[test]
    fn keywords_literals_and_rustdoc_special_names_receive_native_classes() {
        let source = "pub fn demo<'a>(&mut self) -> Option<String> { let n = 42_u8; println!(\"<tag>&\"); Some(true) }";
        assert!(classified(source, "kw").contains(&"fn".into()));
        assert_eq!(classified(source, "string"), ["\"<tag>&\""]);
        assert_eq!(classified(source, "number"), ["42_u8"]);
        assert_eq!(classified(source, "prelude-ty"), ["Option"]);
        assert_eq!(classified(source, "prelude-val"), ["Some"]);
        assert_eq!(classified(source, "bool-val"), ["true"]);
        assert_eq!(classified(source, "lifetime"), ["'a"]);
        assert_eq!(classified(source, "self"), ["self"]);
        assert!(classified(source, "macro").contains(&"println".into()));
        assert!(classified(source, "kw-2").contains(&"mut".into()));
    }
    #[test]
    fn multiline_tokens_keep_their_class_and_original_line_boundaries() {
        let source = "let s = r##\"first\r\n/* not a comment */\nlast\"##;\n/* outer\n /* nested */ end */\nfn next() {}\n";
        let rows = lines(source);
        assert_eq!(rows.len(), 7);
        assert!(
            rows[1]
                .iter()
                .any(|fragment| fragment.class == Some("string")
                    && fragment.text.contains("not a comment"))
        );
        assert!(
            rows[4]
                .iter()
                .any(|fragment| fragment.class == Some("comment")
                    && fragment.text.contains("nested"))
        );
        assert!(
            rows[5]
                .iter()
                .any(|fragment| fragment.class == Some("kw") && fragment.text == "fn")
        );
        let roundtrip = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|fragment| fragment.text)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(roundtrip, source.replace("\r\n", "\n"));
    }
    #[test]
    fn attributes_raw_identifiers_chars_and_macro_metavariables_are_distinct() {
        let source = "#![allow(dead_code)]\n#[cfg(any(feature = \"x\", feature = \"y\"))]\nfn r#match<'a>() { let c = '한'; let p = c\"cstr\"; let b = br#\"bytes\"#; }\nmacro_rules! m { ($x:expr) => { $x }; }\n";
        assert!(classified(source, "attr").contains(&"allow".into()));
        assert!(!classified(source, "kw").contains(&"r#match".into()));
        assert!(classified(source, "string").contains(&"'한'".into()));
        assert!(classified(source, "string").contains(&"c\"cstr\"".into()));
        assert!(classified(source, "string").contains(&"br#\"bytes\"#".into()));
        assert!(classified(source, "macro-nonterminal").contains(&"x".into()));
    }
    #[test]
    fn arbitrary_unicode_html_and_incomplete_input_preserve_text() {
        for source in [
            "",
            "\n",
            "\u{feff}#!/bin/rust\nfn 한글() {}\n",
            "/// docs <script>&\nlet s = \"unfinished\n한글",
            "/* unfinished\n nested /* */",
            "r###\"unterminated\n#",
            "'a '\\n' & && *= != # \u{1f600}",
        ] {
            let text = lines(source)
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|fragment| fragment.text)
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert_eq!(text, source.replace("\r\n", "\n"));
        }
    }
    #[test]
    fn overlapping_ranges_keep_text_classes_and_both_descriptions() {
        let line = lines("let 한글 = value.clone();").remove(0);
        let ranges = [
            Range {
                start: 0,
                end: 7,
                primary: false,
                title: "Secondary: related".into(),
            },
            Range {
                start: 4,
                end: 10,
                primary: true,
                title: "Primary: problem".into(),
            },
        ];
        let result = annotate(line, &ranges);
        assert_eq!(
            result.iter().map(|f| f.text.as_str()).collect::<String>(),
            "let 한글 = value.clone();"
        );
        assert!(
            result.iter().any(|f| f.class.contains("kw")
                && f.class.contains("secondary")
                && f.text == "let")
        );
        let overlap = result.iter().find(|f| f.text == "한").unwrap();
        assert!(overlap.class.contains("primary"));
        assert!(
            overlap.title.contains("Primary: problem")
                && overlap.title.contains("Secondary: related")
        );
    }
    #[test]
    fn caret_preserves_tabs_and_unicode_at_start_end_and_empty_lines() {
        for text in ["\t한글", ""] {
            let line = lines(text).remove(0);
            let ranges = [Range {
                start: text.len(),
                end: text.len(),
                primary: true,
                title: "Primary: insert".into(),
            }];
            let result = annotate(line, &ranges);
            assert_eq!(
                result.iter().map(|f| f.text.as_str()).collect::<String>(),
                text
            );
            assert_eq!(result.iter().filter(|f| f.insertion).count(), 1);
            assert!(result.last().unwrap().insertion);
        }
    }
}
