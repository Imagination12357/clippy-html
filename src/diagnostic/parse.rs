//! Classify JSON stream extensions separately from damaged known messages.
#[derive(Debug)]
pub enum ParsedLine {
    Message(Box<cargo_metadata::Message>),
    Extension,
    Text(String),
    Malformed(String),
}
pub fn parse_line(line: &str) -> ParsedLine {
    let line = line.trim();
    if !line.starts_with('{') {
        return ParsedLine::Text(line.to_owned());
    }
    let value: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => return ParsedLine::Malformed(e.to_string()),
    };
    match value.get("reason").and_then(|v| v.as_str()) {
        Some(
            "compiler-message" | "compiler-artifact" | "build-script-executed" | "build-finished",
        ) => match serde_json::from_value(value) {
            Ok(m) => ParsedLine::Message(Box::new(m)),
            Err(e) => ParsedLine::Malformed(e.to_string()),
        },
        _ => ParsedLine::Extension,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinguishes_json_extensions_text_and_damage() {
        assert!(matches!(
            parse_line(&super::super::sample().to_string()),
            ParsedLine::Message(_)
        ));
        assert!(matches!(
            parse_line("{\"reason\":\"future\",\"extra\":true}"),
            ParsedLine::Extension
        ));
        assert!(matches!(parse_line("build log"), ParsedLine::Text(_)));
        assert!(matches!(
            parse_line("{\"reason\":"),
            ParsedLine::Malformed(_)
        ));
        assert!(matches!(
            parse_line("{\"reason\":\"compiler-message\"}"),
            ParsedLine::Malformed(_)
        ));
    }
}
