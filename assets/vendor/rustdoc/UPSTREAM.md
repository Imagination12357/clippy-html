# rustdoc static styles

- Source: https://github.com/rust-lang/rust
- Release: 1.90.0
- Commit: 1159e78c4747b02ef996e55082b704c09b970588
- Retrieved: 2026-10-27
- CSS: src/librustdoc/html/static/css/rustdoc.css and normalize.css
- Theme: light/dark/ayu declarations are embedded in rustdoc.css in this release.
- License: Rust Project Contributors, MIT option; see LICENSE-MIT.
- normalize.css carries its own MIT notice, retained in that file.
- Full normalize.css v8.0.1 license: https://github.com/necolas/normalize.css/blob/8.0.1/LICENSE.md (normalize-LICENSE-MIT).

Modifications to rustdoc.css: removed all @font-face declarations and the
clipboard/checkmark SVG custom properties. The latter assets have separate
source references and are not used by this report. Fonts and their binaries
are not distributed. System fonts and report-specific rules live in
assets/clippy-html.css. The remaining upstream styles are retained.

No rustdoc JavaScript, logo or font assets are included. Upstream API and DOM
compatibility across releases are not promised.
