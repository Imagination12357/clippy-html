# Navigation metadata, schema version 1

`--metadata <FILE>` writes an optional UTF-8 JSON navigation index. It helps external
tools select findings and discover generated HTML pages without parsing every page.
HTML remains the source of detail. Without this option, no public metadata file is
generated; the report's private JavaScript search index is unrelated.

```sh
# Keep JSON with its report (paths are relative to the invocation directory).
cargo clippy-html --output report --metadata report/metadata.json

# Alternatively, keep the JSON elsewhere.
cargo clippy-html --output report --metadata results/navigation.json -- --all-targets
```

## URLs and identity

Every `url` is relative to the **report root directory**, not the JSON file's
directory. URLs use forward slashes and percent encoding and may have a fragment.
Use them as provided; do not reconstruct filenames, anchors, or identifiers.

There is no stored report base URL, deployment URL or report root filesystem path.
The consumer supplies the root. If metadata is stored at the report root, its
containing directory is the root. Otherwise the consumer must know where the
corresponding report was saved or published. Preserve relative layout when moving
the HTML report. A JSON file alone does not locate a report.

Finding IDs identify deduplicated diagnostics within a report. Package IDs are
opaque Cargo identifiers and may contain local paths. Neither is a promise of
persistent identity across source edits, machines or tool versions. The package
list is the package information available from Cargo metadata (`--no-deps`), so a
diagnostic from a dependency can have a `package_id` absent from that list.

## Shape

This illustrative excerpt abbreviates opaque IDs:

```json
{
  "schema_version": 1,
  "generator": { "name": "cargo-clippy-html", "version": "0.1.0" },
  "execution": { "clippy_exit_code": 0, "collection_complete": true },
  "packages": [
    { "id": "<cargo-package-id>", "name": "example", "version": "0.1.0" }
  ],
  "pages": [
    { "kind": "overview", "name": "Report overview", "url": "index.html" },
    { "kind": "lint", "name": "clippy::clone_on_copy", "url": "lint/clippy/clone_on_copy.html" },
    { "kind": "category", "name": "complexity", "url": "category/complexity.html" },
    { "kind": "source", "name": "src/lib.rs", "url": "src/lib.rs.html" }
  ],
  "findings": [
    {
      "id": "<finding-id>",
      "package_id": "<cargo-package-id>",
      "origin": "Clippy",
      "code": "clippy::clone_on_copy",
      "level": "warning",
      "category": "complexity",
      "message": "using `clone` on a type which implements the `Copy` trait",
      "url": "lint/clippy/clone_on_copy.html#finding-<finding-id>",
      "location": {
        "path": "src/lib.rs",
        "line": 3,
        "column": 5,
        "url": "src/lib.rs.html#L3"
      }
    }
  ]
}
```

| Field | Contract |
|---|---|
| `schema_version` | Integer `1`. Reject unsupported versions. Incompatible changes require a new version; consumers should ignore unknown fields. |
| `generator` | Generating tool's name and version, not the inspected crate or compiler version. |
| `execution.clippy_exit_code` | Actual Clippy process exit code, or `null` if no numeric code is available. |
| `execution.collection_complete` | Whether diagnostic collection completed without detected damage. Inspect this independently of the exit code. |
| `packages` | Known packages with Cargo ID, name and version. Not a list of packages proven to have been fully checked. |
| `pages` | Generated navigation pages: `overview`, `lint`, `category`, `source`. Includes source fallback pages; excludes Help, Settings, licenses and static assets. |
| `findings` | Deduplicated findings, each linked to its canonical lint-page anchor. Count this array for the number of findings. |
| `origin`, `code`, `level` | Diagnostic origin, nullable lint/error code and severity. Preserve unknown string values. |
| `category` | The HTML category group: a Clippy category or `unknown`; `null` for non-Clippy findings. |
| `location` | Optional representative top-level location: first primary span, else first span. Absent when no top-level span exists. It is not a complete list of locations. |
| `location.path` | Display path, normally workspace-relative. External paths can be absolute. Use `location.url` for report navigation, not this label. |
| `location.line`, `location.column` | One-based coordinates reported by the diagnostic, not an exported precise range. |
| `location.url` | Validated source line anchor, or the source page's finding anchor when the original location cannot be validated against loaded source. |

Packages, pages and findings are ordered deterministically, but consumers should
not depend on array positions. Empty reports have an empty finding array. A failed
Clippy run can still produce metadata and HTML and retain its failing exit status.
An incomplete run must not be treated as a successful clean inspection just
because its finding array is empty.

Source text, snippets, child notes, suggestion replacements, patches and exact
ranges are not duplicated in JSON. Follow the finding URL to read those details.
Every exported destination is checked against staged HTML before publication.

## Consumer example

List warning URLs for a report hosted under a repository subpath, using only the
Python standard library:

```python
import json
from pathlib import Path
from urllib.parse import urljoin

data = json.loads(Path("results/navigation.json").read_text(encoding="utf-8"))
if data["schema_version"] != 1:
    raise ValueError("Unsupported navigation schema")
if not data["execution"]["collection_complete"]:
    raise RuntimeError("Diagnostic collection is incomplete")

report_root = "https://example.github.io/my-project/"  # Keep the trailing slash.
for finding in data["findings"]:
    if finding["level"] == "warning":
        print(finding["code"], finding["message"], urljoin(report_root, finding["url"]))
```

For a local report, use `Path("report").resolve().as_uri() + "/"` as the root.

## Existing files and failures

- Relative `--metadata` paths use the invocation directory, just like `--output`.
  Parent directories may be created. No particular filename extension is required.
- JSON inside the HTML output directory is staged and replaced together with that
  report. A subsequent run without `--metadata` replaces the report without retaining
  the previous JSON. Generated-file collisions and reserved publication paths fail.
- External regular files explicitly selected by `--metadata` may be replaced.
  Directories and symlink/reparse-point paths are rejected. A temporary file in the
  destination directory is prepared first, then replaces the old file after HTML
  publication succeeds. HTML failure leaves the old external file unchanged.
- If final external metadata publication fails, the completed HTML remains, the
  command fails, and the error names the published HTML and the metadata failure.
  The old external JSON is retained if it still exists. HTML and external JSON do
  not form an atomic transaction; simultaneous readers can see different snapshots.
  The report's writer lock remains held through external JSON publication to prevent
  cooperating concurrent writers from interleaving the two successful publications.
- `--open` opens the report after all requested outputs have been published.
