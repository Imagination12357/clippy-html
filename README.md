# Clippy HTML

Explore Clippy diagnostics in a portable, rustdoc-styled HTML report. Browse
findings by lint, category or source file; search the report; and inspect suggested
changes as unified diffs. The generator does not modify your source files.

## Install from source

Requires Rust 1.88 or newer and the Clippy component for the toolchain used to
inspect your project. Node.js is only needed for contributor tests.

```sh
rustup component add clippy
git clone https://github.com/Imagination12357/clippy-html.git
cd clippy-html
cargo install --path . --locked
```

Run the installed command in the Cargo project you want to inspect:

```sh
cargo clippy-html --open
```

The default report is `<Cargo target directory>/clippy-html/index.html`. Open that
file directly in a browser, or serve the generated directory on a static host.

## Usage

```sh
cargo clippy-html --output report
cargo clippy-html --output report --metadata report/metadata.json
cargo clippy-html -- --workspace --all-targets
cargo clippy-html -- --all-targets --all-features -- -W clippy::pedantic
```

| Option | Purpose |
|---|---|
| `--output <DIR>` | Choose the report directory. Relative paths use the invocation directory. |
| `--open` | Open the report after all requested outputs have been published. |
| `--metadata <FILE>` | Write an optional machine-readable navigation index. |
| `--offline` | Use bundled lint categories without fetching the online table. |
| `--help`, `--version` | Show command help or version. |

Arguments after the first `--` go to Cargo Clippy. A second `--` separates Cargo
options from rustc/Clippy options. `--all-targets` and `--all-features` are opt-in.
Forwarded `--target-dir` also changes the default report location. Cargo's
`--fix` and `--message-format` options are rejected: source edits and alternate
diagnostic output formats are not supported.

### Offline generation and reading

By default, the generator fetches official category data for the version identified
by diagnostic documentation links. It falls back to the bundled table if retrieval
or validation fails. Category lookup does not change diagnostic severity or exit
status; unmatched Clippy lints appear under Unknown.

The tool's `--offline` controls category lookup; Cargo's `--offline` is separate:

```sh
# Disable both category fetching and Cargo network access.
cargo clippy-html --offline -- --offline
```

Cargo still needs the project's dependencies available locally. Once generated,
the report uses bundled scripts, styles and fonts and needs no network connection.

## Read the report

- **Overview:** execution status, collection completeness, packages and diagnostic
  counts before and after target deduplication.
- **Lints and categories:** related findings, locations, notes and grouped
  suggestions. Readable URLs include `lint/clippy/clone_on_copy.html`.
- **Source files:** file tree, syntax colors, line links, stronger primary and
  softer secondary range backgrounds, and purple triangles for insertion points.
- **Suggestions:** unified diffs and whole-suggestion **Copy patch** controls.
  Applicability is shown as reported by Clippy; copying a patch does not apply it.
- **Search:** lint names, file paths, messages and categories. Search terms are
  case-insensitive and all terms must match. Each tab displays up to 100 results
  and reports the full match count.
- **Settings and Help:** light, dark, Ayu and system themes; navigation and source
  wrapping preferences; keyboard and search help. **Summary / Show all** folds or
  expands diagnostic details while retaining each finding's identity.

Basic pages, links, syntax colors, diffs and individual diagnostic toggles work
without JavaScript. Search, settings, global Summary controls and patch copying
require JavaScript. Clipboard permissions and settings persistence across `file://`
pages depend on the browser.

Useful shortcuts: `S` or `/` opens search, `?` opens Help, `-` folds diagnostics,
and `+` or `=` expands them. Escape closes an active Help/Settings popover before
closing search. See Help for result navigation and Settings to disable shortcuts.

Identical diagnostics from multiple targets are merged; different packages,
severities, spans or suggestions remain distinct. Only eligible workspace source
files are copied in full. External dependencies, files in Cargo target/build
directories, and unreadable or mismatched source use snippets or fallback locations. Valid suggestion
diffs require matching source; unavailable or oversized patches retain the original
replacement details and a reason.

## Navigation metadata

`--metadata <FILE>` exports page and finding URLs plus basic selection fields for
external tools. No public metadata JSON is written by default. All URLs are
relative to the **report root**, regardless of where the JSON is saved. Consumers
supply the report location; no deployment URL is stored. Source text and complete
diagnostic details remain in HTML. See the [schema and consumer example](docs/metadata.md).

## Failures and existing output

A failed Clippy run can still produce a report, and its nonzero exit status is
preserved. Incomplete diagnostic collection is shown separately. Report-generation
or requested metadata-output errors also fail the command.

Reports are staged and validated before replacement. An existing destination must
be a report owned by this tool (`.clippy-html-owned`); unrelated directories and
project, package, target and build roots are protected. Concurrent report writers
use a publication lock. If an interrupted run leaves a lock or backup, inspect the
reported path and confirm no generator is running before attempting recovery.

Report-local metadata is replaced with HTML. External metadata explicitly selected
by `--metadata` may overwrite a regular file; it is published after HTML succeeds.
If that final replacement fails, completed HTML remains and the command fails.
HTML and external JSON are separate publications, not a power-loss-safe transaction.

## Static hosting

Upload the **entire report directory**, including `static.files/`, `licenses/`,
lint/category pages and source pages. Internal links are relative, so reports can
live under a repository subpath. Keep report-local metadata at the report root if
consumers should resolve its URLs relative to the JSON's own directory.

Reports can contain **workspace source code and local paths**. Review their contents
before sharing an artifact or publishing a site. The repository's own CI produces
a self-report and publishes it to Pages on successful main builds; see the
[contributor and CI guide](https://github.com/Imagination12357/clippy-html/blob/main/CONTRIBUTING.md).

## License and contributing

Project code and original circle-and-paperclip artwork are licensed under
[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE). Bundled rustdoc styles and adapted
UI retain their MIT notices; fonts retain their OFL-1.1 licenses. See
[third-party notices](THIRD-PARTY-NOTICES.md) and
[font provenance](assets/vendor/rustdoc/FONTS.md). Generated reports include the
corresponding notices under `licenses/`. These licenses do not change the license
of inspected source code.

The global style reference is rustdoc 1.97.1, with the tabbed Settings structure
adapted from rustdoc 1.102 nightly. This is a Clippy report viewer, not API
documentation or a semantic Rust source browser.

For repository tests, packaging checks and CI setup, see
[CONTRIBUTING](https://github.com/Imagination12357/clippy-html/blob/main/CONTRIBUTING.md).
