const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const vm = require("node:vm");
const context = vm.createContext({});
const source = fs.existsSync("assets/search-engine.js") ? fs.readFileSync("assets/search-engine.js", "utf8") : "";
vm.runInContext(source, context);
const api = context.ClippyHtmlSearch;
const entry = (name, context = "", description = "", kind = "lint") => ({ kind, name, context, description, href: "index.html" });

test("ranks exact name before prefix, substring and contextual matches", () => {
    assert.ok(api, "local search engine must be available");
    const index = api.createIndex([
        entry("other", "copy"), entry("clone_copy"), entry("copy_more"), entry("copy"),
    ]);
    assert.deepEqual(Array.from(api.query(index, "COPY").lint, e => e.name), ["copy", "copy_more", "clone_copy", "other"]);
});
test("matches AND terms across fields without interpreting query syntax", () => {
    const index = api.createIndex([entry("clone_on_copy", "src/lib.rs", "Copy trait", "finding"), entry("other", "src/main.rs")]);
    assert.equal(api.query(index, "clone src/lib").finding.length, 1);
    assert.equal(api.query(index, "clone missing").finding.length, 0);
    assert.equal(api.query(index, "\"clone\"").finding.length, 0);
});
test("normalizes composed Unicode and case, preserves deterministic ties", () => {
    const index = api.createIndex([entry("한글/É.rs", "", "", "file"), entry("한글/é.rs", "", "", "file")]);
    assert.deepEqual(Array.from(api.query(index, "한글/e\u0301").file, e => e.name), ["한글/É.rs", "한글/é.rs"]);
});
test("empty and no-match queries have all four empty groups; totals remain uncapped", () => {
    const index = api.createIndex(Array.from({ length: 125 }, (_, i) => entry(`match-${i}`)));
    for (const query of ["", "  ", "absent"]) {
        const result = api.query(index, query);
        for (const kind of ["lint", "file", "finding", "category"]) assert.equal(result[kind].length, 0);
    }
    assert.equal(api.query(index, "match").lint.length, 125);
});
