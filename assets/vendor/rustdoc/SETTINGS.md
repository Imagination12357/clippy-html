# Settings and theme provenance

Reference: this project's generated rustdoc 1.97.1, Rust commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452. Original sources:
https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static/js
and its generated settings.html. Adapted under the MIT option, Rust Project
Contributors; original full notice is LICENSE-MIT, copied to
licenses/rustdoc-MIT.txt in reports. The generator remains MIT OR Apache-2.0.

Generated reference hashes (SHA256):

- settings-170eb4bf.js: 7a5bc91b83a8c9e590f3ed56fe7a0ea5c6f3bdce5a37fd5c785fcaeab74b2b03
- storage-41dd4d93.js: 41ae6c52806867b0888b909a815fd147c2b012b10211a772747f49c10f3fece9
- main-fcd733ba.js: baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db

assets/settings-storage.js adapts theme/default/preference storage, system media
selection and setting-dependent root classes. assets/settings.js adapts the
Settings button/popup and standalone page, form DOM, labels, option order and
visibility of preferred-theme controls. The original CSS already supplies all
three palettes (light/dark/ayu), the gear icon and settings controls. No complete
upstream JavaScript, Stringdex, custom theme loader or extra theme file is shipped.

Ported in original order: Theme; Preferred light theme; Preferred dark theme;
Directly go to item in search if there is only one result; Hide persistent
navigation bar; Hide table of contents; Disable keyboard shortcuts; Use sans
serif fonts; Word wrap source code. Theme values are light/dark/ayu/system
preference, with the same native defaults and preferred light/dark choices.

Omitted because this report has no matching target: the three API-item/method/
trait auto-hide settings, line numbers on code examples, module navigation and
deprecated API items. Source line numbers remain structural navigation and no
diagnostic is hidden as a deprecated item.

Necessary report adaptations:

- Classic local scripts instead of lazy script requests; synchronous theme boot.
- clippy-html- storage namespace, validated values and current-page fallback when
  storage is denied; cross-page file-URL persistence remains browser-dependent.
- Fixed DOM labels use createElement/textContent; no project text becomes HTML.
- One-result search counts all report kinds; remove the query before following
  the sole destination. Existing search/sidebar/finding behavior remains.
- A valid report-index Back href, with history used for a same-report referrer.
- Independently activated Search/Settings controls; popover dismissal integrates
  with the existing moving search toolbar and document shortcut policy.
- Keep the approved unified sidebar and lack of native mobile topbar/Help/
  Summary/source toggle. Scoped CSS keeps Settings visible, places the arrow at
  the actual last button, bounds/scrolls the native form, honors sidebar hiding
  and removes the unused source/sidebar-toggle search offset.
- Retain bundled Source Code Pro in sans-serif mode; Fira Mono is not shipped.
- Diagnostic-specific overlays use translucent theme focus colors in dark/Ayu;
  severity text inherits the theme foreground for readability. Pinned CSS and
  original lexical token colors are unchanged.
