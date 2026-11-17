# FA-001 fixes — 2026-11-17

The original `.audit/full-audit/001/` report and state are preserved unchanged.
This record resolves its two confirmed findings; it does not repeat the audit.

## FA-001-003: new Windows output aliases

The new integration test first failed with `Output path changed during generation`
for `--output Report --metadata report/navigation.json`. Report-local detection now
uses the normal lexical prefix first. On Windows, when the report is missing and
that prefix fails, an isolated temporary sibling mirrors the missing directories.
Canonicalizing the requested prefix there asks the filesystem whether it is an
alias, including Unicode casing, without creating the real report directory.
The probe is removed by its TempDir owner. No blanket lowercase comparison or
new dependency is used. Existing symlink/reparse checks still run first.

Regression coverage includes first publication, matching-case and uppercase
retries, nested missing directories, Unicode aliases, aliases of the report/root
ancestors and probe cleanup. A platform-neutral conditional case-sensitive test
preserves distinct external paths where the filesystem distinguishes them; that
branch is not exercised on this machine's default case-insensitive directories.

The original audit probe now reports exit 0 with indexExists=true for the alias
case, exit 0 for the matching-case retry, and exit 0 for its control.

## FA-001-001: a source named settings

The new DOM regression first failed because a standalone form was inserted into
the source's main section. Settings now resolves its actual report-root URL using
the existing `data-report-root`, rather than checking only the pathname suffix.
Source `settings.html` pages retain the popover under HTTPS and file URLs; actual
standalone Settings still works with a query and fragment.

The generated rustdoc 1.97.1 reference (`target/doc/static.files/settings-170eb4bf.js`)
uses the suffix check. It assumes rustdoc's separate viewer structure; the shared
viewer needs this local adaptation. Tabbed Settings still references rustdoc
1.102.0-nightly (f5cedb83a, 2026-11-07). No layout/styles/control order changed.
The adaptation is also recorded in the bundled Settings provenance note.

The original audit's actual generated source-page probe now reports:
`panelInsideMain=false`, `popoverExists=true`, `menuHidden=false`.

## Verification

- Rust/Cargo 1.100.0: 80 unit + 15 integration + 7 metadata = 102 tests passed.
- Node: 50 tests passed.
- Format check and strict all-target Clippy passed.
- Rust 1.88.0 locked/offline all-target compile check passed.
- Original isolated audit probe rerun successfully; LF/CRLF/BOM controls still
  export validated L3 source links.
- One fresh-context read-only review found no actionable correctness/scope issues
  and no deferred findings.
- Actual browser layout and Linux/macOS execution were not performed locally.
- Delivery is local Git only; no push or other remote mutation is authorized.
