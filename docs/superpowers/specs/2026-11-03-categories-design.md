# Official Clippy categories — design and research

Date: 2026-11-03. Status: online/offline policy approved in conversation; this concrete design awaits review before product implementation.

## Goal and approved policy

Classify Clippy findings by their official category and provide category-to-lint-to-source navigation. Report generation normally retrieves version-appropriate official metadata online. Explicit tool-level --offline uses bundled metadata by lint name even if its source version differs (the user's A2 behavior). Network failure also falls back to the bundle, with the reason and source recorded. Generated HTML remains self-contained and usable offline.

This replaces the earlier plan's requirement that report generation itself never access the network. It does not change basic no-JavaScript navigation, read-only source handling, severity, Cargo forwarding, exit semantics, or the generator's code license. Search, theme, precise spans, diff/copy, public metadata export and CI implementation are outside this slice.

## Research evidence

Fetched and inspected official HTML:

- https://clippy.rust-lang.org/rust-1.94.0/index.html : HTTP 200, 817 article records, 801 active lints and 16 deprecated entries.
- https://clippy.rust-lang.org/rust-1.97.0/index.html : HTTP 200, 831 article records, 815 active lints and 16 deprecated entries.
- Every article in both samples has exactly one `h2.lint-title .lint-group` label. The lint name is `article[id]`. Example: `article#clone_on_copy` -> `complexity`.
- Confirmed version differences: nonminimal_bool changes complexity -> pedantic; overly_complex_bool_expr changes correctness -> pedantic between these two samples. Fourteen names are added and no article names removed in these samples. This does not establish the history of every intermediate version.
- Current local rustdoc/report toolchain is 1.97.1, but actual Clippy diagnostic help links use `rust-1.97.0`. Use the emitted documentation selector, not the full rustc patch version or the generator's own version.
- `master/lints.json` and `stable/lints.json` candidates were not successfully retrieved in this investigation. No assumption of a supported JSON endpoint is made. Use the observed HTML structure, guarded by validation; do not describe it as a stable upstream API.
- [Official Clippy overview](https://github.com/rust-lang/rust-clippy#readme) describes the nine active categories and MIT OR Apache-2.0 licensing. Category remains independent of the diagnostic's actual severity.

Research files and parser experiments are under ignored target/category-research/. Source downloads were decoded and saved as UTF-8 for inspection; any hashes there describe saved research copies, not untouched HTTP payloads. Production provenance hashes the actual downloaded bytes or exact bundled metadata bytes.

## Approaches considered

1. Bundled-only, exact-version matching: rejected as the default by the user; unsupported versions would unnecessarily remain Unknown.
2. Bundled-only, name matching: retained for explicit offline mode and online failure.
3. Online version-specific metadata with the bundled name-matching fallback: selected by the user. It handles actual category changes while keeping failures nonfatal.

Scraping Rust lint declarations would require tracking repository layout/macros. The official rendered list already supplies the required name/group pairs in one request. Adopt the latter, validate it, and retain the bundled fallback if its shape changes.

## Data flow and version selection

1. Collect Clippy output using the existing Cargo context and normalize/deduplicate findings with category still None.
2. If no Clippy findings exist, do not make a metadata request. rustc/general findings have category not applicable.
3. Inspect help-note trees on Clippy findings for official documentation links. Only HTTPS links on clippy.rust-lang.org or rust-lang.github.io/rust-clippy are eligible. Reject credentials, ports other than the default HTTPS port, queries and unexpected paths. Parse only an exact `rust-X.Y.Z/index.html` selector or the explicitly supported main/master channel selector. Construct the request URL on clippy.rust-lang.org from that validated selector; never request the arbitrary diagnostic URL or its fragment.
4. Use the single unambiguous selector found across the run. A missing link on some findings is fine if the others agree. With no recognized selector or conflicting selectors, use the bundle and record that version selection was unavailable/ambiguous. No extra toolchain-probing process is needed for this slice.
5. Main/master is an upstream moving-channel snapshot, not proof of the exact nightly commit. Label it accordingly. Do not silently substitute latest/stable for a missing version-specific page.
6. Explicit tool --offline skips selection-based requests and uses the bundle. Otherwise retrieve one full table once per run, with one attempt and a 10-second total timeout. No persistent cache/configuration knobs are introduced in the first version.
7. Accept only a successfully validated table. Use that table for the whole run. A name absent from a valid online table stays Unknown; do not silently mix bundled entries into it.
8. On retrieval/parse/validation failure, use the entire bundled table by exact lint name. Keep a visible fallback reason and bundle version; no claim that its category is the executing compiler's exact category.
9. Enrich already-assigned findings and render category pages. Never recompute IDs from enriched findings.

## Retrieval and parsing boundaries

- A blocking HTTPS client fits the synchronous pipeline. Exact client/version is an implementation-plan choice after checking Rust 1.94 compatibility. Keep TLS certificate verification enabled and do not shell out to curl/PowerShell in the product.
- Promote the existing scraper HTML parser from dev-only use if suitable; do not parse HTML with regular expressions or execute its scripts/resources.
- Limit response body to 8 MiB; timeout covers the total operation. Manually restrict redirects to the same approved HTTPS hosts/path selector and at most three hops; if that cannot be enforced by the chosen client, disable automatic redirects. No custom endpoints or supplied credentials.
- Read only `article[id]` and its own direct `label > h2.lint-title .lint-group`, not arbitrary labels in explanatory docs. Validate lint-name characters (lowercase ASCII letters/digits/underscore), unique names, one label per article and the active category allowlist. Empty tables, conflicting records, unknown category labels or changed/unrecognizable structure reject the online table and trigger fallback.
- Treat upstream `deprecated` records separately: recognize them as non-active entries, do not invent their former active category, and leave an emitted matching finding Unknown. Do not add a tenth active category or render empty category pages.
- No downloaded HTML, scripts, styles, source, diagnostic text or package names are sent elsewhere or copied as executable report content. The only network request contains the chosen official documentation path.

## Bundle and attribution

Seed the initial fallback from the verified rust-1.97.0 document (815 active mappings). Generate a sorted name/category-only JSON table with a small maintenance tool using a local official HTML input; do not hand-maintain mappings or embed the full explanations. Record source URL/selector, retrieval date, payload/table hashes and the original upstream MIT notice under assets/vendor/clippy/.

The runtime uses the same validated schema as the maintenance tool. Embed the table and attribution into the executable/report license output. Keep root MIT OR Apache-2.0 and Cargo license metadata unchanged. Runtime fetched data is represented as parsed mappings plus provenance, not redistributed upstream HTML. The source/legal notice selected for the derived bundle must be retained alongside the release's existing third-party inventory.

## CLI contract

| Command | Tool metadata retrieval | Cargo behavior |
|---|---|---|
| cargo clippy-html | Online, then bundle on failure | Existing default |
| cargo clippy-html --offline | Bundle only | Existing default; not automatically Cargo offline |
| cargo clippy-html -- --offline | Online, then bundle on failure | Forward Cargo --offline unchanged |
| cargo clippy-html --offline -- --offline | Bundle only | Cargo --offline |

Explicit --frozen or other forwarded flags remain Cargo's concern; their meanings are not repurposed to control metadata. Document the distinction in help/README. Users needing no network in either component should use both offline options. Cargo environment/configuration is still honored by Cargo itself.

## Model and module boundaries

- src/lint/metadata.rs: table/schema validation, safe selector extraction, online retrieval, bundle fallback and provenance. It consumes findings/options and returns metadata plus nonfatal warnings; it does not render pages or run Cargo. Split network parsing helpers only if the implementation actually needs it; no generic provider framework.
- src/main.rs: resolve metadata after existing diagnostic deduplication, attach categories without regenerating finding IDs, then pass the report to rendering.
- src/diagnostic/model.rs: retain Finding.category and add report-level category-source/provenance state. Keep category warnings separate from Execution.issues/complete; unavailable metadata is not incomplete diagnostic collection and does not alter Clippy exit status.
- src/render/paths.rs and pages.rs: fixed category path mapping and counts, category pages, linked category labels, home summary and metadata-source description. All links stay relative and pass existing validation.
- templates/category.html plus existing index/lint/base templates: use native rustdoc item lists/section headings. No new layout system or inert controls.
- assets/vendor/clippy/: sorted fallback metadata, provenance and original notice. Renderer copies relevant notices through the existing asset list.

Store source mode (online/bundled/not-needed), official selector/URL, source/table digest, and any fallback reason. A dynamic channel's retrieval timestamp may vary; it does not enter finding identity. Renderer tests should accept constructed reports without initiating network requests. Expose fetch/body parsing separately enough to test deterministically, not as a new public plugin API.

## Report behavior

- Home: category list with unique finding counts and distinct lint-kind counts. Sum of categories plus Unknown equals Clippy finding count only; rustc/general are excluded. Reuse existing shared dt/dd column alignment.
- Category page: `category/complexity.html` etc., listing distinct lint codes with finding counts and links to existing lint pages. Each finding is counted once after deduplication, regardless of target/span count.
- Only categories present in the report are generated, including `category/unknown.html` when needed. No category pages for rustc/general-only or clean reports.
- Lint pages: category label links to the corresponding category page for Clippy findings. Compiler/general retain not applicable and are not put into Unknown.
- Unknown remains visible/reachable. A source summary tells the reader whether classification came from online versioned docs, a moving channel, or bundled data; detailed failure reason belongs with that provenance, not repeated on every finding.
- Existing source navigation/finding anchors remain unchanged. No search UI is introduced yet.

## Acceptance and checks

- Representative official categories, absent names, deprecated entries, malformed table and duplicate/conflicting records.
- 1.94 vs 1.97 category changes above select the correct table in a controlled test.
- Tool offline never calls the fetch function; all four CLI combinations preserve Cargo forwarding.
- Timeout/404/TLS/body-limit/parse failures use the bundle, produce useful provenance, and preserve Clippy exit/completeness semantics.
- Reject attacker-controlled request hosts, unsupported selectors and redirect targets; parsing never creates raw HTML or executes scripts.
- Online tables are fetched at most once per run; clean/rustc-only runs do not fetch. Unit/integration tests use controlled data/fetches, not the public network.
- Identical diagnostic input produces the same IDs for online, offline and different metadata versions.
- Category counts are based on deduplicated findings; multi-target/multi-span fixtures do not inflate totals. Unknown and compiler/general cases stay distinct.
- Category -> lint -> source -> finding links and all asset/license links validate at nested paths. Actual browser review remains separately recorded.

## Next step

Independent builder check: no important unanswered decisions before implementation planning. This does not substitute for user review of the concrete design.

Review this design, then write the concrete implementation plan and implement the metadata/category slice. This research step changes no CLI behavior, dependency, diagnostic ID or generated report.
