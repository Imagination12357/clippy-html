# Bundled report assets

The generator's own code remains MIT OR Apache-2.0; see LICENSE-MIT and LICENSE-APACHE. Third-party files retain their own licenses. These notices do not change the licenses of source code inspected by the generator or of diagnostic content included in a report.

| Included component | License | Original notice in this repository | Notice in generated reports |
|---|---|---|---|
| Extracted official Clippy category mappings | MIT option | assets/vendor/clippy/LICENSE-MIT and README.md | licenses/clippy-MIT.txt and licenses/CLIPPY-CATEGORIES.md |
| Clippy HTML favicon, adapted from the Rust logo | CC-BY-4.0 | assets/favicon/README.md and CC-BY-4.0.txt | licenses/FAVICON.md and licenses/favicon-CC-BY-4.0.txt |
| rustdoc CSS and derived font declarations | MIT option | assets/vendor/rustdoc/LICENSE-MIT | licenses/rustdoc-MIT.txt |
| Adapted rustdoc lexical highlighting rules | MIT option | assets/vendor/rustdoc/LICENSE-MIT and HIGHLIGHTING.md | licenses/rustdoc-MIT.txt and licenses/HIGHLIGHTING.md |
| Adapted rustdoc search interface | MIT option | assets/vendor/rustdoc/LICENSE-MIT and SEARCH.md | licenses/rustdoc-MIT.txt and licenses/SEARCH.md |
| Adapted rustdoc Settings UI/CSS and theme behavior | MIT option | assets/vendor/rustdoc/LICENSE-MIT and SETTINGS.md | licenses/rustdoc-MIT.txt and licenses/SETTINGS.md |
| normalize.css | MIT | assets/vendor/rustdoc/normalize-LICENSE-MIT | licenses/normalize-MIT.txt |
| Fira Sans: regular, italic, medium, medium italic | OFL-1.1 | assets/vendor/rustdoc/fonts/Fira-LICENSE.txt | licenses/Fira-LICENSE.txt |
| Source Serif 4: regular, italic, semibold, bold | OFL-1.1 | assets/vendor/rustdoc/fonts/SourceSerif4-LICENSE.md | licenses/SourceSerif4-LICENSE.md |
| Source Code Pro: regular, italic, semibold | OFL-1.1 | assets/vendor/rustdoc/fonts/SourceCodePro-LICENSE.txt | licenses/SourceCodePro-LICENSE.txt |
| NanumBarunGothic: regular | OFL-1.1 | assets/vendor/rustdoc/fonts/NanumBarunGothic-LICENSE.txt | licenses/NanumBarunGothic-LICENSE.txt |

Suggestion patch copy controls also adapt the pinned rustdoc code-block button structure and selected-text copy behavior under its MIT option. See assets/vendor/rustdoc/SUGGESTIONS.md (licenses/SUGGESTIONS.md in generated reports) and the existing rustdoc-MIT.txt notice. No clipboard/checkmark SVG assets are copied.

Each font-family license file preserves the original copyright and Reserved Font Name declarations and the full OFL-1.1 text. The font binaries are unchanged copies of the pinned rustdoc output. See assets/vendor/rustdoc/FONTS.md (licenses/FONTS.md in generated reports) for file names, source details and SHA-256 values.

The original rustdoc COPYRIGHT.txt is retained unmodified as upstream reference material. It also discusses resources not copied from rustdoc, including Rust logos, the complete rustdoc JavaScript files and Fira Mono. Search UI behavior is adapted as described in SEARCH.md; the upstream search engine is not shipped. The favicon is a separate adaptation of user-supplied Rust logo artwork, not a rustdoc asset. It is not the inventory of this bundle; the table above is that inventory.

Source packages must include these notices, all listed original license texts and FONTS.md. Release archives containing the executable must carry the same notices and license texts alongside the generator's own licenses, since the executable embeds these report assets. Generated reports carry their own copies under licenses/ and a linked license overview. This file describes bundled report assets, rather than the complete dependency license inventory.
