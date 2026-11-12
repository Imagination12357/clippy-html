// DOM/state checks only; no browser is launched.
const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const { JSDOM } = require("jsdom");
function setup({ stored = {}, store = new Map(Object.entries(stored).map(([k, v]) => [`clippy-html-${k}`, v])), denied = false, dark = false, page = false, ui = false, url = null } = {}) {
    const dom = new JSDOM(`<!doctype html><html data-theme="light"><head><title>Report</title></head><body class="rustdoc mod" data-report-root="${page ? "" : "../../"}"><a class="skip-main-content" href="#main-content">Skip</a><nav class="sidebar"><section id="rustdoc-toc">Contents</section></nav><main><section id="main-content" class="content" tabindex="-1"><div class="main-heading"><h1>Report</h1>${page ? '<a id="back" href="index.html">Back</a>' : '<rustdoc-toolbar class="report-toolbar hidden"><span id="search-button" class="hidden"><a href="?search="><span class="label">Search</span></a></span><div class="settings-menu hidden" tabindex="-1"><a href="../../settings.html"><span class="label">Settings</span></a></div></rustdoc-toolbar>'}</div></section></main></body></html>`, { url: url || (page ? "https://example.test/report/settings.html" : "https://example.test/report/src/nested/file.rs.html"), runScripts: "outside-only" });
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
test("native settings rows, theme choices and preferred visibility are preserved", () => {
    const s = setup({ ui: true });
    const panel = s.doc.getElementById("settings"); assert.ok(panel, "settings form must be built");
    assert.equal(panel.querySelectorAll(".settings > .setting > .setting-line").length, 9);
    assert.deepEqual(Array.from(panel.querySelectorAll('input[name="theme"]'), i => i.value), ["light", "dark", "ayu", "system preference"]);
    assert.deepEqual(Array.from(panel.querySelectorAll('input[type="checkbox"]'), i => i.id),
        ["sans-serif-fonts", "hide-sidebar", "hide-toc", "disable-shortcuts", "go-to-only-result", "word-wrap-source-code"]);
    assert.equal(panel.querySelector('input[name="theme"]:checked').value, "system preference");
    panel.querySelector('input[name="theme"][value="ayu"]').click();
    assert.equal(s.doc.documentElement.dataset.theme, "ayu");
    assert.ok(panel.querySelector("#preferred-light-theme").classList.contains("hidden"));
    panel.querySelector('input[name="theme"][value="system preference"]').click();
    assert.ok(!panel.querySelector("#preferred-dark-theme").classList.contains("hidden"));
    panel.querySelector('input[name="preferred-dark-theme"][value="ayu"]').click(); s.changeDark(true);
    assert.equal(s.doc.documentElement.dataset.theme, "ayu");
    panel.querySelector('.tab[data-name="Interface"]').click();
    panel.querySelector("#hide-sidebar").click(); assert.ok(s.doc.documentElement.classList.contains("hide-sidebar"));
    assert.equal(panel.querySelector(".setting-check > input + span").textContent, "Use sans serif fonts");
    s.dom.window.close();
});
test("native popover toggles, dismisses, restores focus and preserves modified links", async () => {
    const s = setup({ ui: true }), button = s.doc.querySelector(".settings-menu > a"), panel = s.doc.getElementById("settings");
    button.click(); assert.equal(panel.style.display, ""); assert.ok(panel.classList.contains("popover"));
    button.click(); assert.equal(panel.style.display, "none");
    const modified = new s.w.MouseEvent("click", { button: 0, ctrlKey: true, bubbles: true, cancelable: true });
    button.dispatchEvent(modified); assert.ok(!modified.defaultPrevented); assert.equal(panel.style.display, "none");
    button.click(); panel.querySelector("input").focus();
    s.doc.dispatchEvent(new s.w.KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
    assert.equal(panel.style.display, "none"); assert.equal(s.doc.activeElement, button);
    button.click(); s.doc.body.click(); assert.equal(panel.style.display, "none");
    button.click(); panel.querySelector("input").focus(); s.doc.getElementById("main-content").focus();
    await Promise.resolve(); assert.equal(panel.style.display, "none"); s.dom.window.close();
});
test("internal pointer focus transitions keep the popup open during the body-focus interval", async () => {
    for (const selector of ['#word-wrap-source-code', 'input[name="theme"][value="ayu"]', '.setting-radio-name', '.tab[data-name="Interface"]']) {
        const s = setup({ ui: true }), button = s.doc.querySelector(".settings-menu > a"), panel = s.doc.getElementById("settings");
        button.focus(); button.click();
        const target = panel.querySelector(selector);
        if (selector === '#word-wrap-source-code') panel.querySelector('.tab[data-name="Content"]').click();
        const nextFocus = target instanceof s.w.HTMLInputElement || target instanceof s.w.HTMLButtonElement ? target : panel;
        // Browsers may run microtasks between focusout and the next focus event.
        // jsdom's .focus() completes both synchronously, so expose that interval.
        Object.defineProperty(s.doc, "activeElement", { configurable: true, get: () => s.doc.body });
        button.dispatchEvent(new s.w.FocusEvent("focusout", { bubbles: true, relatedTarget: nextFocus }));
        await Promise.resolve();
        assert.equal(panel.style.display, "", `${selector}: focus is moving inside the popup`);
        delete s.doc.activeElement;
        nextFocus.focus(); target.click(); await Promise.resolve();
        assert.equal(panel.style.display, "", `${selector}: internal activation must not dismiss`);
        assert.equal(button.getAttribute("aria-expanded"), "true");
        s.dom.window.close();
    }
});
test("standalone settings uses native section structure and applies preferences", () => {
    const s = setup({ ui: true, page: true }); const panel = s.doc.getElementById("settings");
    assert.ok(panel, "standalone settings section exists"); assert.equal(panel.tagName, "SECTION");
    assert.ok(!panel.classList.contains("popover"));
    panel.querySelector('.tab[data-name="Content"]').click();
    panel.querySelector("#word-wrap-source-code").click(); assert.ok(s.doc.documentElement.classList.contains("word-wrap-source-code"));
    assert.equal(s.doc.querySelector("#back").getAttribute("href"), "index.html"); s.dom.window.close();
});
test("settings and search share the toolbar; disable shortcuts and single-result navigation work", () => {
    const s = setup({ ui: true });
    s.w.CLIPPY_HTML_SEARCH_DATA = [{ kind: "lint", name: "copy", context: "", description: "", href: "lint/clippy/copy.html" }];
    s.load("search-engine"); s.load("search");
    const key = name => s.doc.body.dispatchEvent(new s.w.KeyboardEvent("keydown", { key: name, bubbles: true, cancelable: true }));
    s.api.set("disable-shortcuts", "true"); key("/");
    assert.ok(!s.doc.getElementById("main-content").classList.contains("hidden"));
    s.api.set("disable-shortcuts", "false");
    s.doc.querySelector(".settings-menu > a").click(); key("/");
    assert.equal(s.doc.getElementById("settings").style.display, "none");
    assert.ok(s.doc.querySelector("#search .main-heading .settings-menu"));
    s.doc.querySelector(".settings-menu > a").click();
    s.doc.getElementById("settings").querySelector('input[name="theme"][value="ayu"]').click();
    assert.equal(s.doc.documentElement.dataset.theme, "ayu");
    key("Escape");
    assert.equal(s.doc.getElementById("settings").style.display, "none");
    assert.ok(s.doc.getElementById("main-content").classList.contains("hidden"));
    key("Escape"); assert.ok(!s.doc.getElementById("main-content").classList.contains("hidden"));
    s.api.set("go-to-only-result", "true"); key("/");
    let navigated = null;
    s.doc.addEventListener("click", e => { const a = e.target.closest("#results .search-results > a"); if (a) { e.preventDefault(); navigated = a.href; } });
    const input = s.doc.querySelector(".search-input"); input.value = "copy";
    input.form.dispatchEvent(new s.w.Event("submit", { cancelable: true }));
    assert.equal(navigated, "https://example.test/report/lint/clippy/copy.html");
    assert.ok(!new URL(s.w.location.href).searchParams.has("search")); s.dom.window.close();
});
test("single-result setting also applies to deep queries, but not multiple report kinds", () => {
    for (const count of [1, 2]) {
        const s = setup({ ui: true, stored: { "go-to-only-result": "true" }, url: "https://example.test/report/src/nested/file.rs.html?search=copy#L4" });
        s.w.CLIPPY_HTML_SEARCH_DATA = [{ kind: "lint", name: "copy", context: "", description: "", href: "lint/clippy/copy.html" }];
        if (count === 2) s.w.CLIPPY_HTML_SEARCH_DATA.push({ kind: "finding", name: "copy", context: "", description: "", href: "lint/clippy/copy.html#finding-one" });
        let navigations = 0;
        s.doc.addEventListener("click", e => { if (e.target.closest("#results .search-results > a")) { e.preventDefault(); navigations++; } });
        s.load("search-engine"); s.load("search");
        assert.equal(navigations, count === 1 ? 1 : 0);
        assert.equal(new URL(s.w.location.href).searchParams.has("search"), count !== 1);
        assert.equal(s.w.location.hash, "#L4"); s.dom.window.close();
    }
});
test("settings activates independently of search data and handles persisted pages", () => {
    const s = setup({ ui: true });
    assert.ok(!s.doc.querySelector("rustdoc-toolbar").classList.contains("hidden"));
    assert.ok(s.doc.getElementById("search-button").classList.contains("hidden"));
    s.api.set("theme", "ayu");
    s.store.set("clippy-html-theme", "dark");
    s.w.dispatchEvent(new s.w.PageTransitionEvent("pageshow", { persisted: true }));
    assert.equal(s.doc.documentElement.dataset.theme, "dark");
    assert.equal(s.doc.querySelector('input[name="theme"]:checked').value, "dark"); s.dom.window.close();
});
test("docs.rs rustdoc tabs preserve grouping, checkbox order and selection in popup and page", () => {
    for (const page of [false, true]) {
        const s = setup({ ui: true, page }); const panel = s.doc.getElementById("settings");
        if (!page) s.doc.querySelector('.settings-menu > a').click();
        const tabs = Array.from(panel.querySelectorAll('.settings > .tabs > button.tab'));
        assert.deepEqual(tabs.map(tab => tab.textContent), ['Appearance', 'Interface', 'Content']);
        const expected = { Appearance: ['sans-serif-fonts'], Interface: ['hide-sidebar', 'hide-toc', 'disable-shortcuts', 'go-to-only-result'], Content: ['word-wrap-source-code'] };
        assert.equal(panel.querySelector('.setting.selected').dataset.name, 'Appearance');
        for (const tab of tabs) {
            const group = panel.querySelector(`.setting[data-name="${tab.dataset.name}"]`);
            assert.deepEqual(Array.from(group.querySelectorAll('input[type="checkbox"]'), input => input.id), expected[tab.dataset.name]);
            tab.click(); assert.equal(panel.querySelectorAll('.tab.selected').length, 1);
            assert.equal(panel.querySelectorAll('.setting.selected').length, 1);
            assert.ok(group.classList.contains('selected'));
            if (!page) assert.equal(panel.style.display, '');
        }
        s.dom.window.close();
    }
});
