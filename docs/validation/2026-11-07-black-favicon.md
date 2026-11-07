# Black favicon — 2026-11-07

User requested a quieter black favicon matching the official black Rust logo. Downloaded https://raw.githubusercontent.com/rust-lang/rust-artwork/main/logo/rust-logo.svg and inspected its explicit `stroke="black"` declarations. Use the equivalent #000000, replacing #D34516 in visible gear/clip strokes and fills. The mask is unchanged.

- Updated favicon.svg and regenerated all 16/32/48px ICO entries from it. Updated attribution/color descriptions in the asset notice and generated license page.
- Compared the previous committed SVG with the new one, normalizing line endings and reversing only the color substitution: geometry is unchanged.
- At 16/32/48px, alpha channels match the previous icon exactly and all visible raster pixels are black. ICO dimensions/pixels match SVG rasterizations.
- Regenerated target/rustdoc-style-preview through the report publisher using explicit tool offline mode. Recursive internal links/anchors validated; additional inspection verified both favicon links across all 32 HTML pages and emitted asset/notice byte equality.
- No runtime logic or dependencies changed; no new tests or repeat full suite for this asset-only change. Actual browser favicon cache/display remains manual.
