//! Generate the bundled category table from a local official HTML snapshot.
//! Usage: cargo run --example update_lint_metadata -- INPUT SELECTOR DATE OUTPUT
#[path = "../src/diagnostic/mod.rs"]
#[allow(dead_code)]
mod diagnostic;
#[path = "../src/lint/mod.rs"]
#[allow(dead_code)]
mod lint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("expected INPUT SELECTOR DATE OUTPUT".into());
    }
    let bytes = std::fs::read(&args[0])?;
    let bundle = lint::metadata::Bundle {
        schema_version: 1,
        selector: args[1].clone(),
        url: format!("https://clippy.rust-lang.org/{}/index.html", args[1]),
        retrieved: args[2].clone(),
        source_hash: format!("blake3:{}", blake3::hash(&bytes).to_hex()),
        entries: lint::metadata::parse_html(&bytes)?,
    };
    std::fs::write(
        &args[3],
        format!("{}\n", serde_json::to_string_pretty(&bundle)?),
    )?;
    Ok(())
}
