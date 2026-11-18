# Bundled report assets

Clippy HTML's code and paperclip icon are available under MIT or Apache-2.0.
See `LICENSE-MIT` and `LICENSE-APACHE`. The table covers bundled third-party
report assets. Inspected source and diagnostic content keep their original
license terms.

| Included component | License | Original notice in this repository | Notice in generated reports |
|---|---|---|---|
| Extracted official Clippy category mappings | MIT option | assets/vendor/clippy/LICENSE-MIT and README.md | licenses/clippy-MIT.txt and licenses/CLIPPY-CATEGORIES.md |
| rustdoc CSS and derived font declarations | MIT option | assets/vendor/rustdoc/LICENSE-MIT | licenses/rustdoc-MIT.txt |
| Adapted rustdoc lexical highlighting rules | MIT option | assets/vendor/rustdoc/LICENSE-MIT and HIGHLIGHTING.md | licenses/rustdoc-MIT.txt and licenses/HIGHLIGHTING.md |
| Adapted rustdoc search interface | MIT option | assets/vendor/rustdoc/LICENSE-MIT and SEARCH.md | licenses/rustdoc-MIT.txt and licenses/SEARCH.md |
| Adapted rustdoc Settings UI/CSS and theme behavior | MIT option | assets/vendor/rustdoc/LICENSE-MIT and SETTINGS.md | licenses/rustdoc-MIT.txt and licenses/SETTINGS.md |
| Adapted rustdoc code-block copy control | MIT option | assets/vendor/rustdoc/LICENSE-MIT and SUGGESTIONS.md | licenses/rustdoc-MIT.txt and licenses/SUGGESTIONS.md |
| Adapted rustdoc Help and keyboard behavior | MIT option | assets/vendor/rustdoc/LICENSE-MIT and HELP.md | licenses/rustdoc-MIT.txt and licenses/HELP.md |
| Adapted rustdoc Summary structure and toolbar control | MIT option | assets/vendor/rustdoc/LICENSE-MIT and SUMMARY.md | licenses/rustdoc-MIT.txt and licenses/SUMMARY.md |
| normalize.css | MIT | assets/vendor/rustdoc/normalize-LICENSE-MIT | licenses/normalize-MIT.txt |
| Fira Sans: regular, italic, medium, medium italic | OFL-1.1 | assets/vendor/rustdoc/fonts/Fira-LICENSE.txt | licenses/Fira-LICENSE.txt |
| Source Serif 4: regular, italic, semibold, bold | OFL-1.1 | assets/vendor/rustdoc/fonts/SourceSerif4-LICENSE.md | licenses/SourceSerif4-LICENSE.md |
| Source Code Pro: regular, italic, semibold | OFL-1.1 | assets/vendor/rustdoc/fonts/SourceCodePro-LICENSE.txt | licenses/SourceCodePro-LICENSE.txt |
| NanumBarunGothic: regular | OFL-1.1 | assets/vendor/rustdoc/fonts/NanumBarunGothic-LICENSE.txt | licenses/NanumBarunGothic-LICENSE.txt |

The report includes the original font binaries and full OFL-1.1 notices,
including copyright and Reserved Font Name declarations. `FONTS.md` lists the
file names and hashes. Find it at `assets/vendor/rustdoc/FONTS.md` in the source
package and `licenses/FONTS.md` in a report.

The unmodified rustdoc `COPYRIGHT.txt` describes the broader upstream bundle.
Its entries include Rust logos, Fira Mono, and complete rustdoc scripts that
Clippy HTML does not ship. Clippy HTML supplies its own search engine and icon.

Include these notices, the listed license texts, and `FONTS.md` in source and
executable distributions, alongside the generator's licenses. The executable
embeds the report assets and writes their notices to `licenses/` in each report.
For library dependencies, consult their respective licenses.
