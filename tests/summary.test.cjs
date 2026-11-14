const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const { JSDOM } = require("jsdom");

function setup({ findings = true, search = false, closed = false, url = "https://example.test/report/src/nested/file.rs.html" } = {}) {
    const finding = id => `<section class="finding" id="finding-${id}"><details class="toggle finding-toggle" ${closed ? "" : "open"}><summary><section class="finding-summary"><h2>warning clippy::copy</h2><p class="finding-message">message ${id}</p><p class="finding-location"><a href="#L1">src/lib.rs:1:1</a></p></section></summary><div class="finding-body"><a class="body-link" href="#L1">location</a><pre>snippet ${id}</pre><p>note</p><details><summary>Targets</summary><div id="nested-${id}">target</div></details><details class="suggestion" open><summary>Suggestion</summary><pre>diff</pre></details></div></details></section>`;
    const dom = new JSDOM(`<!doctype html><body data-report-root="../../"><nav><details id="tree" open><summary>Files</summary>tree</details></nav><main><section id="main-content" class="content"><div class="main-heading"><h1>Source</h1><rustdoc-toolbar class="report-toolbar"><span id="search-button"><a href="?search="><span class="label">Search</span></a></span><div class="settings-menu"><a href="../../settings.html">Settings</a></div><div class="help-menu"><a href="../../help.html">Help</a></div></rustdoc-toolbar></div><pre class="source-code" id="L1">unchanged source</pre><details id="log" open><summary>Log</summary>log</details>${findings ? finding("one") + finding("two") : ""}</section></main></body>`, { url, runScripts: "outside-only" });
    const w = dom.window, doc = w.document, flags = new Map();
    w.scrollTo = () => {};
    w.ClippyHtmlSettings = { get: name => flags.get(name) || "false" };
    if (search) {
        w.CLIPPY_HTML_SEARCH_DATA = [{ kind: "finding", name: "two", context: "", description: "", href: "src/nested/file.rs.html#finding-two" }];
        for (const name of ["search-engine", "search"]) w.eval(fs.readFileSync(`assets/${name}.js`, "utf8"));
    }
    const script = fs.readFileSync("assets/summary.js", "utf8"); w.eval(script);
    const key = (target, value, extra = {}) => {
        const event = new w.KeyboardEvent("keydown", { key: value, bubbles: true, cancelable: true, ...extra }); target.dispatchEvent(event); return event;
    };
    return { dom, w, doc, flags, key, script, button: doc.getElementById("toggle-all-docs"), toggles: [...doc.querySelectorAll(".finding-toggle")] };
}
test("Summary and Show all control findings and nested details, leaving source/sidebar/log untouched", () => {
    const s = setup(); assert.ok(s.button);
    assert.equal(s.button.querySelector(".label").textContent, "Summary");
    assert.equal(s.button.parentElement.lastElementChild, s.button);
    s.button.click();
    assert.ok(s.toggles.every(t => !t.open));
    assert.ok([...s.doc.querySelectorAll(".finding details")].every(t => !t.open));
    assert.equal(s.button.querySelector(".label").textContent, "Show all");
    assert.ok(s.button.classList.contains("will-expand"));
    assert.equal(s.doc.querySelector("#tree").open, true);
    assert.equal(s.doc.querySelector("#log").open, true);
    assert.equal(s.doc.querySelector(".source-code").textContent, "unchanged source");
    s.button.click();
    assert.ok([...s.doc.querySelectorAll(".finding details")].every(t => t.open));
    assert.equal(s.button.querySelector(".label").textContent, "Summary");
    s.w.eval(s.script); assert.equal(s.doc.querySelectorAll("#toggle-all-docs").length, 1);
    s.dom.window.close();
});
test("manual and mixed finding state updates the aggregate action", () => {
    const s = setup();
    s.toggles[0].open = false; s.toggles[0].dispatchEvent(new s.w.Event("toggle"));
    assert.equal(s.button.querySelector(".label").textContent, "Summary");
    s.toggles[1].open = false; s.toggles[1].dispatchEvent(new s.w.Event("toggle"));
    assert.equal(s.button.querySelector(".label").textContent, "Show all");
    s.toggles[1].open = true; s.toggles[1].dispatchEvent(new s.w.Event("toggle"));
    assert.equal(s.button.querySelector(".label").textContent, "Summary");
    s.dom.window.close();
});
test("keyboard commands respect editing, composition, modifiers, handled events and disabled shortcuts", () => {
    const s = setup();
    for (const extra of [{ ctrlKey: true }, { altKey: true }, { metaKey: true }, { isComposing: true }, { keyCode: 229 }]) assert.equal(s.key(s.doc.body, "-", extra).defaultPrevented, false);
    for (const tag of ["input", "textarea", "select", "div"]) {
        const field = s.doc.createElement(tag); if (tag === "div") field.setAttribute("contenteditable", "true"); s.doc.body.append(field);
        assert.equal(s.key(field, "-").defaultPrevented, false);
    }
    s.doc.body.dispatchEvent(new s.w.CompositionEvent("compositionstart", { bubbles: true }));
    assert.equal(s.key(s.doc.body, "-").defaultPrevented, false);
    s.doc.body.dispatchEvent(new s.w.CompositionEvent("compositionend", { bubbles: true }));
    const event = new s.w.KeyboardEvent("keydown", { key: "-", bubbles: true, cancelable: true }); event.preventDefault(); s.doc.body.dispatchEvent(event);
    assert.ok(s.toggles.every(t => t.open));
    s.flags.set("disable-shortcuts", "true"); assert.equal(s.key(s.doc.body, "-").defaultPrevented, false);
    s.button.click(); assert.ok(s.toggles.every(t => !t.open));
    s.flags.set("disable-shortcuts", "false");
    assert.equal(s.key(s.doc.body, "=").defaultPrevented, true); assert.ok(s.toggles.every(t => t.open));
    s.key(s.doc.body, "-"); s.key(s.doc.body, "+"); assert.ok(s.toggles.every(t => t.open));
    assert.equal(s.key(s.doc.body, "_").defaultPrevented, false);
    s.dom.window.close();
});
test("closing a body with focused content moves focus to its visible summary", () => {
    const s = setup(); const link = s.doc.querySelector(".body-link"); link.focus();
    s.key(link, "-");
    assert.equal(s.doc.activeElement, s.toggles[0].querySelector(":scope > summary"));
    assert.equal(s.toggles[0].open, false); s.dom.window.close();
});
test("initial and history hashes reveal only the target finding and its nested detail ancestors", () => {
    const s = setup({ closed: true, url: "https://example.test/report/src/nested/file.rs.html#nested-two" });
    assert.equal(s.toggles[0].open, false); assert.equal(s.toggles[1].open, true);
    assert.equal(s.doc.querySelector("#nested-two").closest("details").open, true);
    s.button.click(); s.w.history.replaceState(null, "", "#finding-one"); s.w.dispatchEvent(new s.w.PopStateEvent("popstate"));
    assert.equal(s.toggles[0].open, true); assert.equal(s.toggles[1].open, false);
    s.dom.window.close();
});
test("same-hash clicks reveal their target without rewriting links or affecting modified/external/source links", () => {
    const s = setup({ closed: true, url: "https://example.test/report/src/nested/file.rs.html#finding-two" });
    const link = s.doc.createElement("a"); link.href = "#finding-two"; s.doc.body.append(link);
    // Preserve the app's default-navigation decision, then suppress jsdom navigation.
    let native;
    s.doc.addEventListener("click", event => { native = !event.defaultPrevented; event.preventDefault(); });
    s.button.click();
    for (const extra of [{ ctrlKey: true }, { shiftKey: true }, { metaKey: true }]) {
        link.dispatchEvent(new s.w.MouseEvent("click", { bubbles: true, cancelable: true, ...extra })); assert.equal(s.toggles[1].open, false);
    }
    link.click(); assert.equal(s.toggles[1].open, true); assert.equal(s.toggles[0].open, false); assert.equal(native, true);
    assert.equal(link.getAttribute("href"), "#finding-two");
    s.button.click(); link.href = "other.html#finding-two"; link.click(); assert.ok(s.toggles.every(t => !t.open));
    link.href = "#L1"; link.click(); assert.ok(s.toggles.every(t => !t.open));
    s.dom.window.close();
});
test("search disables controls even for an initial query, then close restores them without collapsing findings", () => {
    const s = setup({ search: true, url: "https://example.test/report/src/nested/file.rs.html?search=" });
    assert.equal(s.button.disabled, true); assert.ok(!s.button.parentElement.classList.contains("has-summary"));
    assert.equal(s.key(s.doc.body, "-").defaultPrevented, false); s.button.click(); assert.ok(s.toggles.every(t => t.open));
    s.key(s.doc.querySelector(".search-input"), "Escape"); assert.equal(s.button.disabled, false);
    assert.ok(s.button.parentElement.classList.contains("has-summary"));
    s.key(s.doc.body, "/"); assert.equal(s.button.disabled, true);
    s.doc.querySelector("#search-button a").click(); assert.equal(s.button.disabled, false);
    s.dom.window.close();
});
test("pages without findings expose no inactive control and intercept no section commands", () => {
    const s = setup({ findings: false }); assert.equal(s.button, null);
    assert.equal(s.key(s.doc.body, "-").defaultPrevented, false);
    s.dom.window.close();
});
test("same-document search results restore controls and reveal only their finding", () => {
    const s = setup({ search: true }); s.button.click();
    s.key(s.doc.body, "/");
    const input = s.doc.querySelector(".search-input"); input.value = "two";
    input.form.dispatchEvent(new s.w.Event("submit", { bubbles: true, cancelable: true }));
    s.doc.addEventListener("click", event => { event.preventDefault(); });
    s.doc.querySelector("#results .search-results.active > a").click();
    assert.equal(s.button.disabled, false);
    assert.ok(!s.doc.querySelector("#main-content").classList.contains("hidden"));
    assert.equal(s.toggles[0].open, false); assert.equal(s.toggles[1].open, true);
    assert.equal(s.button.querySelector(".label").textContent, "Summary");
    s.dom.window.close();
});
test("expanding or revealing a focused compact location keeps focus on the visible summary", () => {
    const s = setup({ closed: true });
    const link = s.toggles[0].querySelector(".finding-location a"); link.focus();
    s.key(link, "+");
    assert.equal(s.doc.activeElement, s.toggles[0].querySelector(":scope > summary"));
    s.button.click(); link.href = "#finding-one"; link.focus();
    s.doc.addEventListener("click", event => { event.preventDefault(); });
    link.click();
    assert.equal(s.doc.activeElement, s.toggles[0].querySelector(":scope > summary"));
    assert.equal(s.toggles[0].open, true); assert.equal(s.toggles[1].open, false);
    s.dom.window.close();
});
test("source findings reserve a chevron gutter even when main has no left padding", () => {
    const s = setup(); s.doc.body.className = "rustdoc src";
    const style = s.doc.createElement("style"); style.textContent = fs.readFileSync("assets/clippy-html.css", "utf8"); s.doc.head.append(style);
    const gutter = Number.parseFloat(s.w.getComputedStyle(s.toggles[0]).paddingLeft) || 0;
    assert.ok(gutter >= 16, "finding needs room for the native 16px chevron");
    s.dom.window.close();
});
