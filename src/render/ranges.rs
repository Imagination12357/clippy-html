//! Split lexical fragments at validated diagnostic boundaries, preserving text.
use super::highlight;
use std::collections::BTreeSet;

pub(super) struct Range {
    pub start: usize,
    pub end: usize,
    pub primary: bool,
    pub title: String,
}
pub(super) struct Fragment {
    pub text: String,
    pub class: String,
    pub title: String,
    pub insertion: bool,
}

pub(super) fn annotate(fragments: Vec<highlight::Fragment>, ranges: &[Range]) -> Vec<Fragment> {
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
fn marked(text: &str, class: Option<&str>, ranges: &[&Range], insertion: bool) -> Fragment {
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
    Fragment {
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
    use crate::render::highlight;
    #[test]
    fn overlapping_ranges_keep_text_classes_and_both_descriptions() {
        let line = highlight::lines("let 한글 = value.clone();").remove(0);
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
            let line = highlight::lines(text).remove(0);
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
