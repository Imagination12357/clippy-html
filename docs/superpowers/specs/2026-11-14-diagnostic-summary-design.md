# Diagnostic Summary / Show all

Approved A1: collapse diagnostic details while preserving severity, lint code,
message and a representative location. Source tree and full-source text remain
untouched. Only pages containing findings get the toolbar control. Metadata
export is not in this task.

Reference: this project's rustdoc 1.97.1 output, main-fcd733ba.js
expandAllDocs/collapseAllDocs/toggleAllDocs and generated struct.Finding.html.
Reuse details.toggle > summary > section and #toggle-all-docs > span.label,
will-expand state, Summary / Show all labels and existing arrow icon/CSS.
Keep each .finding section/id as the anchor owner and place an initially open
details.toggle.finding-toggle inside it. Summary contains the existing heading
and message, plus the first primary location (or first location, if available).
Hide that compact location when open because the detailed locations already
show it. No new per-finding Details label or explanatory legend.

Summary closes only finding toggles and their nested details. Show all opens
that same scope, including targets, suggestions and rendered diagnostic text.
Initial rendering retains the existing nested defaults. Sidebar details and
overview logs/status remain independent. If any finding is open, the toolbar
offers Summary; when all are closed, it offers Show all. Individual native
summary clicks update this aggregate state. Closing a focused body moves focus
to its visible summary so focus is not left inside hidden content.

Keys: - collapses, + / = expands. No _ command because there are no impl blocks.
Respect existing modifier, editable, IME, handled-event and disable-shortcuts
rules. Explicit button clicks still work with shortcuts disabled. During search,
disable/hide the control without changing underlying findings; restore on close.
Search emits a matching close event, and initial state also checks main.hidden
for query URLs opened before this script initializes.

Reveal a linked finding on normal same-document clicks (even unchanged hash),
initial hash, hashchange, history navigation and pageshow. Open that finding and
any detail ancestors of a nested target; do not expand unrelated findings or
sidebars. Keep modified clicks/download/new-tab links native. Preserve browser
anchor scrolling; no route rewriting or source-line collapse.

Summary appears after Help in the native toolbar order. Adjust both popup
arrow offsets only while this control is visible, including the reference's
850px compact Help/Settings widths. The native Summary button remains 80px
wide. Search hides it, so popup offsets must return to the no-Summary values.

Without JavaScript the findings start open and native details work; no inactive
global control is shown. Help gains the implemented + / = and - rows only.
No persistence setting or new dependency is introduced.
