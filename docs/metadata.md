# Navigation metadata, schema version 1

Pass `--metadata <FILE>` to write a UTF-8 JSON index of report pages and findings.
Scripts can filter findings in the JSON, then open the HTML links for details.
Omit the option to generate only the report. Its JavaScript search index is a
separate file for internal use.

```sh
# Keep JSON with its report (paths are relative to the invocation directory).
cargo clippy-html --output report --metadata report/metadata.json

# Save the index outside the report directory.
cargo clippy-html --output report --metadata results/navigation.json -- --all-targets
```

## URLs and identity

Resolve each `url` against the report root directory. Use the supplied path and
fragment as written, including percent escapes. When the index is at the report
root, use its containing directory. If you save the index elsewhere, supply the
report's directory or hosting URL separately. The index stores relative references
only. When moving a report, keep its directory structure intact.

Treat finding and package IDs as opaque keys within one report. They may change
across source edits, machines, or tool versions, and package IDs can contain
local paths. The package list comes from `cargo metadata --no-deps`, so a
dependency finding can refer to a package outside that list.

## Schema

This example uses placeholders for the IDs.

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
| `schema_version` | Integer `1`. Reject unsupported versions and ignore unknown fields. Incompatible changes require a new version. |
| `generator` | `cargo-clippy-html` and its version. |
| `execution.clippy_exit_code` | Actual Clippy process exit code, or `null` if no numeric code is available. |
| `execution.collection_complete` | Whether the generator finished collecting diagnostics without detecting a problem. Check this separately from the exit code. |
| `packages` | Cargo IDs, names, and versions from `cargo metadata --no-deps`. A package can appear here even if Clippy did not check it. |
| `pages` | `overview`, `lint`, `category`, and `source` pages. Source pages include snippet fallbacks. |
| `findings` | One entry per deduplicated finding, with a link to its anchor on the lint page. |
| `origin`, `code`, `level` | Diagnostic origin, nullable lint/error code and severity. Preserve unknown string values. |
| `category` | The report's Clippy category, `unknown` for unclassified Clippy findings, or `null` for other findings. |
| `location` | The first primary top-level span, or the first top-level span if none is primary. Absent when there is no top-level span. |
| `location.path` | Label for display, usually relative to the workspace. External paths can be absolute. |
| `location.line`, `location.column` | The diagnostic's one-based start line and column. |
| `location.url` | Validated source line anchor, or the source page's finding anchor when the original location cannot be validated against loaded source. |

The generator orders the arrays deterministically. Use IDs and URLs to look up
entries. A failed Clippy run can still produce HTML and JSON while returning a
nonzero status. Treat an empty `findings` array as a clean run only when collection
completed and Clippy returned zero.

Follow the finding URL for source text, snippets, child notes, suggestions, and
exact ranges. The generator checks each exported destination against the HTML
before publishing the report.

## Consumer example

This script lists warning URLs for a report hosted under a repository subpath.

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

Relative `--metadata` paths start at the directory where you run the command.
The tool can create parent directories and accepts any filename extension.

Inside the report directory, the tool stages JSON with the HTML and replaces them
together. Regenerating that report without `--metadata` drops the previous JSON.
Choose a path that does not collide with generated files or reserved output names.

For an external path, the tool prepares a temporary file next to the destination
and replaces the selected regular file after publishing HTML. It rejects
directories, symbolic links, and reparse points. If HTML publication fails, the
old external file stays in place. If the final JSON replacement fails, the tool
leaves the completed HTML and any previous JSON in place, reports the failure,
and returns a nonzero status.

Readers can see different HTML and JSON versions between the two writes or after
an interrupted run. The report lock covers both writes to stop overlapping runs
from interleaving them. With `--open`, the browser opens after both outputs are ready.
