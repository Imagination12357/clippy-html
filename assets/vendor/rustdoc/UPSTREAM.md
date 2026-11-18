# rustdoc static styles

The main stylesheet comes from rustdoc 1.97.1 at Rust commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`. We ran
`cargo doc --no-deps --document-private-items` on this project and copied
`rustdoc-17e0aaed.css` and `normalize-9960930a.css` from
`target/doc/static.files` on 2026-10-27. Upstream CSS lives in
`src/librustdoc/html/static/css/`. [Rust repository](https://github.com/rust-lang/rust)

The original rustdoc CSS has SHA-256
`264dc5a58dbfaf73bca6f1b5767413c775d1150c07855f6b399e8672ae3d9597`.
It supplies the light, dark, and Ayu palettes. We moved the selected
`@font-face` rules to `fonts.css` and removed the clipboard and checkmark SVG
properties. `FONTS.md` lists the 12 original WOFF2 files and their hashes.

`assets/clippy-html.css` contains the report styles. Insertion points use an
upward triangle with a dedicated color in each theme (`#7c3aed` in light,
`#b69aff` in dark, and `#c4a7ff` in Ayu). The marker preserves source text and
has an accessible label. Browser titles follow rustdoc's ASCII ` - ` separator
and use the `Clippy HTML` suffix for document and search views.

The Settings tabs and `settings-tabs.css` come from rustdoc 1.102.0-nightly as
served by docs.rs. See `SETTINGS.md` for the source URLs and hashes. The other
provenance notes cover the adapted search, Help, Summary, and copy controls.
The report ships its own search engine and uses the bundled styles whenever
it generates a report.

The rustdoc styles use the MIT option. `LICENSE-MIT` and `COPYRIGHT.txt` carry
the original attribution. The latter covers the upstream bundle, including
assets that Clippy HTML omits. See `THIRD-PARTY-NOTICES.md` for this project's
inventory. Normalize.css 8.0.1 retains its own MIT notice, which is also in
`normalize-LICENSE-MIT`. [Normalize.css license](https://github.com/necolas/normalize.css/blob/8.0.1/LICENSE.md)
