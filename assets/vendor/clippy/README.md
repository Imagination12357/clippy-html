# Official Clippy category data

Derived from the Rust Project Contributors' official Clippy lint documentation, using the MIT option of its MIT OR Apache-2.0 license. Original notice: LICENSE-MIT (clippy-MIT.txt in generated reports).

- Source: https://clippy.rust-lang.org/rust-1.97.0/index.html
- Upstream: https://github.com/rust-lang/rust-clippy
- Retrieved: 2026-11-07
- Snapshot: rust-1.97.0; 815 active lint/category pairs, with 16 deprecated entries excluded.
- Source BLAKE3 hash, source URL and retrieval date are preserved in categories.json. It contains only sorted names/categories and provenance, not full upstream descriptions, HTML, scripts or styles. The table fingerprint shown in a report is BLAKE3 of serde_json's compact serialized sorted entries.
- Modifications: extract article IDs and their own lint-group labels; omit deprecated rows and explanations. The generator does not infer a category from the diagnostic's level or lint name.

Regeneration, using a downloaded local official HTML document:

```
cargo run --locked --example update_lint_metadata -- INPUT.html rust-1.97.0 YYYY-MM-DD assets/vendor/clippy/categories.json
```

The maintained table and parser are validated by tests. Online mode obtains the official table selected by diagnostic help links; tool --offline and online failure use this bundle by lint name even across version differences. The generated report records which source/table was actually used and any fallback reason. Missing names stay Unknown. Category provenance is separate from diagnostic collection completeness and Clippy's exit status.

The generator's code license remains MIT OR Apache-2.0; this does not change licenses of inspected source or diagnostic content. Include this provenance and the original MIT notice in source/executable distributions, alongside the existing third-party notices.
