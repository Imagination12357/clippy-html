# Diagnostic Summary provenance

Reference: this project's generated rustdoc 1.97.1, Rust commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452. Inspected main-fcd733ba.js
(SHA-256 baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db),
storage-41dd4d93.js and struct.Finding.html. Source under the MIT option:
https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static
Original Rust Project Contributors notice and license remain in LICENSE-MIT /
generated licenses/rustdoc-MIT.txt.

Native structure: details.toggle > summary > section, with the item heading
inside summary and detailed content below it. The toolbar uses button
#toggle-all-docs > span.label, Summary/Show all labels and will-expand state.
Existing vendored CSS supplies the chevrons; no new icon asset was copied.
Native expandAllDocs/collapseAllDocs target managed .toggle blocks, not every
details in the document. This report scopes the corresponding operation to
finding toggles and nested diagnostic details only.

The report preserves each finding section/id. Its summary retains severity,
lint code, message and the first primary location (first available fallback).
The compact location is hidden when expanded because the original detailed
locations already show it. Initial findings are open and native toggles work
without JavaScript; the global control is created only by summary.js on pages
with findings. No inactive control is shown on overview/Help/Settings pages.

The finding wrapper misses rustdoc's direct-child mobile gutter selectors.
Source pages and <=700px views reserve an internal 20px finding gutter and
position the native 16px chevron inside it, including source views whose main
element has zero left padding. Other desktop finding alignment is unchanged.

Summary closes that diagnostic scope; Show all opens it, including targets,
suggestions and original diagnostic text. Sidebar trees, full source, overview
status/logs and search results are excluded. Mixed state offers Summary; all
closed offers Show all. Native manual toggles update the button. Focus in a
closing body moves to its visible summary. No new per-finding Details label or
legend is introduced.
Expansion also moves focus off the compact location that becomes hidden when
open, including self-referential fallback-location links.

The native - and +/= commands operate on the same scope. _ is omitted because
there are no impl blocks. Existing editable/IME/modifier/handled-event and
disable-shortcuts rules apply. Search disables/hides the control and removes
its toolbar-width contribution; closing search restores it. Deep links reveal
only their target finding/ancestors and retain native link navigation.

Summary follows Help in the toolbar. While visible it contributes the native
80px button plus button-left-margin to both popup offsets. Help/Settings become
compact at 850px, while the Summary width remains 80px. Search hides Summary,
so offsets return to their no-Summary values. Browser geometry and actual
assistive-technology behavior require separate acceptance.
