// DOM behavior only: this does not launch a browser or establish file:// support.
// Install test-only jsdom as documented in README; NODE_PATH may locate it.
const { test } = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const { JSDOM } = require("jsdom");
const entry = (name, kind = "lint", href = "lint/clippy/copy.html") => ({ kind, name, href, context: "src/한글.rs", description: "message" });
function setup({ data = [entry("copy")], url = "https://example.test/report/src/nested/file.rs.html#L4", historyFails = false } = {}) {
    const dom = new JSDOM(`<!doctype html><title>Original</title><body data-report-root="../../"><nav class="sidebar"><a href="../../index.html">Overview</a></nav><main><section id="main-content" class="content" tabindex="-1"><div class="main-heading"><h1>Source</h1><rustdoc-toolbar class="report-toolbar hidden"><span id="search-button"><a href="?search="><span class="label">Search</span></a></span></rustdoc-toolbar></div><div id="L4">original source</div></section></main></body>`, { url, runScripts: "outside-only" });
    const w = dom.window;
    w.scrollTo = () => {};
    if (historyFails) for (const name of ["pushState", "replaceState"]) w.history[name] = () => { throw new Error("history unavailable"); };
    if (data !== null) w.CLIPPY_HTML_SEARCH_DATA = data;
    w.eval(fs.readFileSync("assets/search-engine.js", "utf8"));
    w.eval(fs.existsSync("assets/search.js") ? fs.readFileSync("assets/search.js", "utf8") : "");
    const doc = w.document;
    const key = (target, name, extra = {}) => target.dispatchEvent(new w.KeyboardEvent("keydown", { key: name, bubbles: true, cancelable: true, ...extra }));
    const open = () => { key(doc.body, "/"); assert.ok(doc.querySelector(".search-input"), "native search input must exist"); };
    const submit = text => {
        const input = doc.querySelector(".search-input"); input.value = text;
        input.form.dispatchEvent(new w.Event("submit", { bubbles: true, cancelable: true }));
    };
    return { dom, w, doc, key, open, submit };
}
test("native view replaces main, moves toolbar and restores original DOM/focus/hash", () => {
    const { dom, doc, w, key, open, submit } = setup();
    const button = doc.querySelector("#search-button a"); button.focus();
    const original = doc.querySelector("#L4");
    open(); submit("copy");
    assert.ok(doc.querySelector("#main-content").classList.contains("hidden"));
    assert.equal(doc.querySelector("#search-button .label").textContent, "Exit");
    assert.ok(doc.querySelector("#search .main-heading rustdoc-toolbar"));
    assert.equal(new URL(w.location.href).searchParams.get("search"), "copy");
    key(doc.querySelector(".search-input"), "Escape");
    assert.equal(doc.querySelector("#L4"), original);
    assert.ok(!doc.querySelector("#main-content").classList.contains("hidden"));
    assert.equal(doc.activeElement, button);
    assert.equal(w.location.hash, "#L4");
    assert.equal(w.document.title, "Original");
    dom.window.close();
});
test("safe text and nested local links; tabs, row navigation, counts and limit", () => {
    const payload = "<img src=x onerror=alert(1)>";
    const data = [entry(payload), entry("match", "file", "src/한글.rs.html"), ...Array.from({ length: 120 }, (_, i) => entry(`match ${i}`, "finding"))];
    const { dom, doc, key, open, submit } = setup({ data });
    open(); submit(payload);
    assert.equal(doc.querySelectorAll("#results img").length, 0);
    assert.ok(doc.querySelector(".search-results.active .path").textContent.includes(payload));
    submit("match");
    const buttons = doc.querySelectorAll("#search-tabs button");
    assert.equal(buttons.length, 4);
    buttons[1].click();
    const link = doc.querySelector(".search-results.active > a");
    assert.equal(decodeURI(link.href), "https://example.test/report/src/한글.rs.html");
    key(doc.querySelector(".search-input"), "ArrowDown"); assert.equal(doc.activeElement, link);
    key(link, "ArrowRight");
    assert.ok(buttons[2].classList.contains("selected"));
    assert.equal(doc.querySelectorAll(".search-results.active > a").length, 100);
    assert.ok(buttons[2].textContent.includes("120"));
    assert.ok(doc.querySelector(".search-results.active").textContent.includes("100 of 120"));
    key(doc.querySelector(".search-results.active > a"), "ArrowUp");
    assert.equal(doc.activeElement, doc.querySelector(".search-input"));
    dom.window.close();
});
test("editing, modifiers and composition do not trigger global shortcuts", () => {
    const { dom, doc, key } = setup();
    const editor = doc.createElement("textarea"); doc.body.append(editor); editor.focus();
    key(editor, "/"); key(doc.body, "s", { ctrlKey: true }); key(doc.body, "s", { isComposing: true });
    assert.ok(!doc.querySelector("#main-content").classList.contains("hidden"));
    key(doc.body, "s");
    assert.ok(doc.querySelector("#main-content").classList.contains("hidden"));
    dom.window.close();
});
test("deep query/popstate and denied history preserve usable search", () => {
    const { dom, doc, w, key } = setup({ url: "https://example.test/report/src/nested/file.rs.html?other=1&search=copy#L4" });
    assert.equal(doc.querySelector(".search-input")?.value, "copy");
    w.history.replaceState(null, "", "?other=1#L4"); w.dispatchEvent(new w.PopStateEvent("popstate"));
    assert.ok(!doc.querySelector("#main-content").classList.contains("hidden"));
    w.history.replaceState(null, "", "?other=1&search=copy#L4"); w.dispatchEvent(new w.PopStateEvent("popstate"));
    assert.ok(doc.querySelector("#main-content").classList.contains("hidden"));
    key(doc.body, "Escape"); assert.equal(new URL(w.location.href).searchParams.get("other"), "1");
    dom.window.close();
    const denied = setup({ historyFails: true }); denied.open(); denied.submit("copy");
    assert.equal(denied.doc.querySelectorAll(".search-results.active > a").length, 1);
    denied.key(denied.doc.body, "Escape");
    assert.ok(!denied.doc.querySelector("#main-content").classList.contains("hidden"));
    denied.dom.window.close();
});
test("missing data leaves toolbar hidden and basic document intact", () => {
    const { dom, doc, key } = setup({ data: null }); key(doc.body, "/");
    assert.ok(doc.querySelector("rustdoc-toolbar").classList.contains("hidden"));
    assert.ok(!doc.querySelector("#main-content").classList.contains("hidden"));
    assert.ok(doc.querySelector(".sidebar a")); dom.window.close();
});
test("input waits 500ms, IME text waits for completion, Exit cancels pending work", async () => {
    const { dom, doc, w, open } = setup(); open();
    const input = doc.querySelector(".search-input");
    assert.ok(doc.querySelector(".search-results.active").textContent.includes("Enter a search term"));
    input.dispatchEvent(new w.CompositionEvent("compositionstart", { bubbles: true }));
    input.value = "copy"; input.dispatchEvent(new w.Event("input", { bubbles: true }));
    await new Promise(resolve => setTimeout(resolve, 520));
    assert.equal(doc.querySelectorAll(".search-results.active > a").length, 0);
    input.dispatchEvent(new w.CompositionEvent("compositionend", { bubbles: true }));
    await new Promise(resolve => setTimeout(resolve, 200));
    assert.equal(doc.querySelectorAll(".search-results.active > a").length, 0);
    await new Promise(resolve => setTimeout(resolve, 350));
    assert.equal(doc.querySelectorAll(".search-results.active > a").length, 1);
    input.value = "absent"; input.dispatchEvent(new w.Event("input", { bubbles: true }));
    doc.querySelector("#search-button a").click();
    await new Promise(resolve => setTimeout(resolve, 520));
    assert.equal(doc.title, "Original");
    assert.ok(!doc.querySelector("#main-content").classList.contains("hidden"));
    dom.window.close();
});
test("unsafe index URLs fail closed without hiding the document", () => {
    for (const href of ["javascript:alert(1)", "https://evil.test/fake.html", "../outside.html"]) {
        const { dom, doc, key } = setup({ data: [entry("bad", "lint", href)] }); key(doc.body, "/");
        assert.ok(doc.querySelector("rustdoc-toolbar").classList.contains("hidden"));
        assert.ok(!doc.querySelector("#main-content").classList.contains("hidden")); dom.window.close();
    }
});
test("same-document sidebar and finding links leave search and reveal the target", () => {
    for (const [fromSidebar, historyFails] of [[true, false], [false, false], [false, true]]) {
        const { dom, doc, w, open, submit } = setup({ historyFails, data: [entry("copy", "finding", "src/nested/file.rs.html#L4")] });
        let scrolled = false; doc.querySelector("#L4").scrollIntoView = () => { scrolled = true; };
        const anchor = doc.createElement("a"); anchor.href = "#L4"; anchor.textContent = "Line 4";
        doc.querySelector(".sidebar").append(anchor);
        open(); submit("copy");
        (fromSidebar ? anchor : doc.querySelector(".search-results.active > a")).click();
        assert.ok(!doc.querySelector("#main-content").classList.contains("hidden"), "anchor target must be visible");
        assert.ok(scrolled, "even an unchanged hash must reveal its target");
        assert.equal(w.location.hash, "#L4");
        assert.ok(!new URL(w.location.href).searchParams.has("search"));
        dom.window.close();
    }
});
test("same-document handling preserves modified clicks and search skip link", () => {
    const { dom, doc, w, open } = setup(); open();
    const link = doc.createElement("a"); link.href = "#L4"; doc.querySelector(".sidebar").append(link);
    link.dispatchEvent(new w.MouseEvent("click", { button: 0, ctrlKey: true, bubbles: true, cancelable: true }));
    assert.ok(doc.querySelector("#main-content").classList.contains("hidden"));
    const skip = doc.createElement("a"); skip.href = "#alternative-display"; doc.body.append(skip); skip.click();
    assert.ok(doc.querySelector("#main-content").classList.contains("hidden"));
    dom.window.close();
});
