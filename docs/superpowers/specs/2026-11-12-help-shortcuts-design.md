# Help and keyboard shortcuts

Approved scope: add ? / Help and reconcile existing S / /, search navigation,
Enter and Escape. Expand/collapse + / = / - / _ belongs to a later
Summary/Show All decision; do not implement or advertise it now.

Reference: generated rustdoc 1.97.1 help.html and main-fcd733ba.js from this
project. Reuse toolbar .help-menu > a, #help.popover > .content > .side-by-side,
.shortcuts > h2 + dl (kbd/dt/dd), .infos and a standalone help.html with Back.
Omit the rustdoc handbook, API search syntax and version footer; explain only
the report's four search kinds, AND words and case-insensitive matching.
Do not add new explanatory paragraphs elsewhere in the report.

Render Help content once as an Askama partial, both in a non-JS standalone page
and an inert template for the JS popover. The visible Help link navigates to
help.html without JS; modified clicks keep normal navigation. Theme bootstrap
continues to apply. The popup participates in existing focus/outside-click
dismissal, not a new modal or focus trap. Search, Help and Settings share the
same toolbar; Help and Settings exclude each other while search stays open.

Escape consumes one layer: Help/Settings first, then search. Capture listeners
and defaultPrevented prevent double handling; a central UI framework is not
needed. Search activation closes popovers. Restore trigger focus on Escape.
? and search shortcuts ignore editable input/textarea/select/contenteditable,
IME composition, browser modifiers and already handled events. Checkbox/radio
controls follow the existing rustdoc convention of allowing global commands.
Disable keyboard shortcuts suppresses custom ?, search, Escape and arrow
handlers; native Tab/Enter/button activation remains available. Buttons and
links work whether that setting is enabled or disabled.

Settings stays the last toolbar button so its existing arrow anchor remains
valid. Help uses the native popup layout and a scoped arrow offset accounting
for the following Settings control. Narrow screens use the existing rustdoc
responsive layout, with scrolling only where needed.

Validation: DOM tests for layered Escape, focus, popup exclusion, modified
links, denied storage, missing search data, composition/editing/modifiers,
shortcut disable and standalone no-JS content. Generated relative links and
scripts must validate. Browser layout/assistive technology remain distinct
from automated DOM evidence. Work on main; no push.
