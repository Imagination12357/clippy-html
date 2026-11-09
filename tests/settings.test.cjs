// DOM/state checks only; no browser is launched.
const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const { JSDOM } = require("jsdom");
function setup({ stored = {}, store = new Map(Object.entries(stored).map(([k, v]) => [`clippy-html-${k}`, v])), denied = false, dark = false, page = false, ui = false } = {}) {
    const dom = new JSDOM(`<!doctype html><html data-theme="light"><head><title>Report</title></head><body class="rustdoc mod" data-report-root="${page ? "" : "../../"}"><a class="skip-main-content" href="#main-content">Skip</a><nav class="sidebar"><section id="rustdoc-toc">Contents</section></nav><main><section id="main-content" class="content" tabindex="-1"><div class="main-heading"><h1>Report</h1>${page ? '<a id="back" href="index.html">Back</a>' : '<rustdoc-toolbar class="report-toolbar hidden"><span id="search-button" class="hidden"><a href="?search="><span class="label">Search</span></a></span><div class="settings-menu hidden" tabindex="-1"><a href="../../settings.html"><span class="label">Settings</span></a></div></rustdoc-toolbar>'}</div></section></main></body></html>`, { url: page ? "https://example.test/report/settings.html" : "https://example.test/report/src/nested/file.rs.html", runScripts: "outside-only" });
    const w = dom.window, doc = w.document;
    Object.defineProperty(w, "localStorage", { get() {
        if (denied) throw new w.DOMException("Denied", "SecurityError");
        return { getItem: k => store.get(k) ?? null, setItem: (k, v) => store.set(k, String(v)), removeItem: k => store.delete(k) };
    } });
    const listeners = new Set();
    const media = { matches: dark, addEventListener: (_, fn) => listeners.add(fn) };
    w.matchMedia = () => media; w.scrollTo = () => {};
    const load = name => w.eval(fs.existsSync(`assets/${name}.js`) ? fs.readFileSync(`assets/${name}.js`, "utf8") : "");
    load("settings-storage"); if (ui) load("settings");
    const changeDark = value => { media.matches = value; for (const fn of listeners) fn(); };
    return { dom, w, doc, store, changeDark, load, api: w.ClippyHtmlSettings };
}
test("native theme defaults follow system; light, dark and Ayu can be fixed", () => {
    const s = setup({ dark: true }); assert.ok(s.api, "settings bootstrap must initialize");
    assert.equal(s.doc.documentElement.dataset.theme, "dark");
    assert.equal(s.api.get("use-system-theme"), "true");
    for (const theme of ["light", "dark", "ayu"]) {
        s.api.set("theme", theme); assert.equal(s.doc.documentElement.dataset.theme, theme);
        assert.equal(s.api.get("use-system-theme"), "false");
    }
    s.changeDark(false); assert.equal(s.doc.documentElement.dataset.theme, "ayu"); s.dom.window.close();
});
test("preferred system themes include Ayu and follow OS changes", () => {
    const s = setup(); s.api.set("preferred-dark-theme", "ayu");
    s.api.set("theme", "system preference"); s.changeDark(true);
    assert.equal(s.doc.documentElement.dataset.theme, "ayu");
    s.changeDark(false); assert.equal(s.doc.documentElement.dataset.theme, "light");
    s.api.set("preferred-light-theme", "dark"); assert.equal(s.doc.documentElement.dataset.theme, "dark");
    s.dom.window.close();
});
test("preferences persist on other report pages; corrupt values fall back safely", () => {
    const s = setup(); s.api.set("theme", "ayu"); s.api.set("word-wrap-source-code", "true");
    const next = setup({ store: s.store, page: true });
    assert.equal(next.doc.documentElement.dataset.theme, "ayu");
    assert.ok(next.doc.documentElement.classList.contains("word-wrap-source-code"));
    assert.ok([...s.store.keys()].every(k => k.startsWith("clippy-html-")));
    const bad = setup({ stored: { theme: "https://evil.test/x.css", "use-system-theme": "false", "preferred-dark-theme": "oops", "hide-sidebar": "yes" } });
    assert.equal(bad.doc.documentElement.dataset.theme, "light");
    assert.equal(bad.api.set("theme", "oops"), false);
    assert.ok(!bad.doc.documentElement.classList.contains("hide-sidebar"));
    for (const p of [s, next, bad]) p.dom.window.close();
});
test("denied storage still applies current-page themes and boolean classes", () => {
    const s = setup({ denied: true }); s.api.set("theme", "ayu");
    assert.equal(s.doc.documentElement.dataset.theme, "ayu");
    for (const name of ["hide-sidebar", "hide-toc", "sans-serif-fonts", "word-wrap-source-code"]) {
        s.api.set(name, "true"); assert.ok(s.doc.documentElement.classList.contains(name));
        s.api.set(name, "false"); assert.ok(!s.doc.documentElement.classList.contains(name));
    }
    s.dom.window.close();
});
