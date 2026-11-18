# Help and shortcuts

Help follows the toolbar, popover, and standalone page in rustdoc 1.97.1 (Rust
commit `8bab26f4f68e0e26f0bb7960be334d5b520ea452`). The reference artifacts are
`help.html`, `main-fcd733ba.js` (SHA-256
`baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db`),
`storage-41dd4d93.js` (SHA-256
`41ae6c52806867b0888b909a815fd147c2b012b10211a772747f49c10f3fece9`),
and `rustdoc-17e0aaed.css`. The existing stylesheet supplies the Help icon.
The adaptation uses the MIT option. Find the Rust Project Contributors' notice
in `LICENSE-MIT` or `licenses/rustdoc-MIT.txt` in a report. [Upstream source](https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static)

A shared partial renders the standalone page and the template cloned into
the popup. The Help link works without JavaScript and modified clicks follow
the link. Back defaults to the overview. With JavaScript it uses history when
the previous page belongs to the same report.

The shortcut list covers `?`, `S` or `/`, result arrows, `⏎`, `-`, and
`+` or `=`. Like the pinned reference, Help handles Escape without adding a
row for it. The footer uses the generator's package version. Search Tricks
describes the report's case-insensitive matching and how it combines terms.

Search, Settings, and Help follow rustdoc's toolbar order. Summary follows Help
on pages with findings. Opening either popover closes the other. Escape closes
the active popover before leaving search, and opening search closes popovers.
Custom shortcuts respect text input, IME composition, modifier keys, and the
disable-shortcuts setting. Native Tab and Enter keep working.

Popup offsets account for Summary when it is visible, including at the compact
850px breakpoint. At 700px and below, the report keeps the Help link available
in its own toolbar and scrolls long content within the popover.
