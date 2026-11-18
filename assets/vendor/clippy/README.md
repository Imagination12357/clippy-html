# Clippy category data

The bundled table comes from Clippy's official lint documentation for
`rust-1.97.0`, retrieved on 2026-11-07. It has 815 active lint names and their
categories. The extractor skips 16 deprecated entries and reads each article ID
and its lint group. It leaves out the explanations, scripts, and styles.
[Source snapshot](https://clippy.rust-lang.org/rust-1.97.0/index.html) · [Clippy repository](https://github.com/rust-lang/rust-clippy)

`categories.json` stores the sorted names and categories, source URL, retrieval
date, and BLAKE3 hash of the source HTML. The table hash shown in reports is
BLAKE3 of the sorted entries serialized as compact JSON by serde_json.

To regenerate the table, save the official HTML locally and run this command
from a repository checkout.

```sh
cargo run --locked --example update_lint_metadata -- INPUT.html rust-1.97.0 YYYY-MM-DD assets/vendor/clippy/categories.json
```

By default the generator fetches the table for the Clippy documentation version
in diagnostic help links. It uses the bundle if the version is unclear, fetching
or validation fails, or you pass the tool's `--offline` option. The fallback
matches by lint name across versions. Names missing from the selected table appear
under Unknown. The overview lists the table used and any fallback reason.

The table uses the MIT option of Clippy's MIT OR Apache-2.0 license. Distribute
this note and Clippy's original MIT notice with packaged source and binaries.
Generated reports carry the notice as `licenses/clippy-MIT.txt`.
