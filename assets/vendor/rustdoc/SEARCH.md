# Search interface provenance

The interface follows this project's generated rustdoc 1.97.1 output from Rust
commit `8bab26f4f68e0e26f0bb7960be334d5b520ea452`. These are the reference
artifacts and their SHA-256 hashes.

| File | SHA-256 |
|---|---|
| `main-fcd733ba.js` | `baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db` |
| `search-fd9372ac.js` | `f765706d62d88eb8f83ca79eaba8158512a0d8f9955190d1c4f5313c675109a3` |
| `rustdoc-17e0aaed.css` | `264dc5a58dbfaf73bca6f1b5767413c775d1150c07855f6b399e8672ae3d9597` |

The adaptation uses the MIT option. The Rust Project Contributors' notice is
in `LICENSE-MIT` and appears as `licenses/rustdoc-MIT.txt` in the report.
[Upstream source](https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static)

`assets/search.js` follows rustdoc's toolbar movement, in-place results,
Search and Exit labels, input delay of 500 ms, query history, tab counts, and
keyboard navigation. Local scripts load in order. `search-engine.js` normalizes
Unicode and case, matches every term as a substring, and ranks the results
deterministically. The four tabs cover lints, files, findings, and categories.
Each tab shows up to 100 rows alongside its total count.

Results use text nodes and generator-owned URLs. The markup keeps rustdoc's CSS
selectors while using valid HTML for the result panels. Scoped styles account
for the report's shared source sidebar and compact toolbar. Search restores
focus and scroll on exit, respects IME composition, and closes before following
a same-document finding link. Basic navigation works without scripts.

The empty and no-match states use rustdoc's centered styling, “Example searches”
list, and “No results :(” text. The examples use names from the current report,
or sample queries when the report is empty. They run in the report's search.
