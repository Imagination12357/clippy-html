# Settings and theme provenance

The Settings tabs follow rustdoc 1.102.0-nightly, build `f5cedb83a` dated
2026-11-07, as seen in the clap documentation on 2026-11-09. The stylesheet
and theme behavior for the rest of the report come from rustdoc 1.97.1.
[clap documentation](https://docs.rs/clap/latest/clap/)

| Tabbed Settings reference | SHA-256 |
|---|---|
| [`settings-9b351432.js`](https://docs.rs/-/rustdoc.static/settings-9b351432.js) | `38fdd256c2644b0a968e7e0b449889a84b5aa3a18ea7f7f5ffec29bf7f50657a` |
| [`rustdoc-1f5a765c.css`](https://docs.rs/-/rustdoc.static/rustdoc-1f5a765c.css) | `21b9806fbc35469ae6ca54f2917b3474d8c5ba33083babbb8fff03a81da0f446` |

Appearance contains the theme controls and sans-serif option. Interface contains
the sidebar, table of contents, shortcuts, and single-result search options.
Content contains source wrapping. `settings-tabs.css` keeps the relevant
declarations and the 700px breakpoint. The report positions the popover arrow
behind the tabs and adjusts its offset for the toolbar's visible buttons.

## Theme storage and popovers

The original reference is this project's rustdoc 1.97.1 output at Rust commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`. The adaptation uses the MIT option
and retains the Rust Project Contributors' notice in `LICENSE-MIT` and
`licenses/rustdoc-MIT.txt` in reports. [Upstream JavaScript](https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static/js)

| Generated reference | SHA-256 |
|---|---|
| `settings-170eb4bf.js` | `7a5bc91b83a8c9e590f3ed56fe7a0ea5c6f3bdce5a37fd5c785fcaeab74b2b03` |
| `storage-41dd4d93.js` | `41ae6c52806867b0888b909a815fd147c2b012b10211a772747f49c10f3fece9` |
| `main-fcd733ba.js` | `baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db` |

`settings-storage.js` applies the theme at startup, validates preferences, and
stores them under `clippy-html-` keys. If storage is blocked, changes still work
on the current page. Persistence across `file://` pages depends on the browser.
The original stylesheet supplies the light, dark, and Ayu palettes, the gear
icon, and the form controls. Sans-serif mode keeps Source Code Pro for code.

`settings.js` builds the labels and toggles the toolbar popover. It uses
`data-report-root` to identify the standalone page and distinguish it from a
source page named `src/settings.html`. The standalone Back link goes to the
overview or uses browser history for a same-report referrer.

Search and Settings initialize independently. When the single-result setting
is enabled, search counts matches across all tabs and removes the query before
following the only result. The report keeps a shared sidebar across page types,
with scoped CSS for toolbar placement, scrolling, and the sidebar preference.
Diagnostic ranges use translucent focus colors in dark and Ayu, and severity
text uses the theme foreground.
