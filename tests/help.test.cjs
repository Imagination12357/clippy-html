const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const { JSDOM } = require("jsdom");

function setup({ search = true, standalone = false, denied = false, url } = {}) {
    const content = fs.readFileSync("templates/help.html", "utf8").replaceAll("{{ generator_version }}", "test-version");
    const toolbar = '<rustdoc-toolbar class="report-toolbar"><span id="search-button" class="hidden"><a href="?search="><span class="label">Search</span></a></span><div class="help-menu" tabindex="-1"><a href="../../help.html"><span class="label">Help</span></a></div><div class="settings-menu hidden" tabindex="-1"><a href="../../settings.html"><span class="label">Settings</span></a></div></rustdoc-toolbar>';
    const dom = new JSDOM(`<!doctype html><title>Original</title><body data-report-root="${standalone ? "" : "../../"}"><main><section id="main-content" class="content"><div class="main-heading"><h1>Report</h1>${standalone ? '<a id="back" href="index.html">Back</a>' : toolbar}</div>${standalone ? content : "original"}</section></main>${standalone ? "" : `<template id="help-content">${content}</template>`}</body>`, { url: url || (standalone ? "file:///report/help.html" : "https://example.test/report/src/nested/file.rs.html"), runScripts: "outside-only" });
    const w = dom.window, doc = w.document;
    w.matchMedia = () => ({ matches: false, addEventListener() {} });
    w.scrollTo = () => {};
    if (denied) Object.defineProperty(w, "localStorage", { get() { throw new w.DOMException("Denied", "SecurityError"); } });
    if (search) w.CLIPPY_HTML_SEARCH_DATA = [{ kind: "lint", name: "copy", href: "lint/clippy/copy.html", context: "src/lib.rs", description: "copy warning" }];
    for (const name of ["settings-storage", "search-engine", "search", "settings", "help"]) {
        w.eval(fs.readFileSync(`assets/${name}.js`, "utf8"));
    }
    const key = (target, value, extra = {}) => {
        const event = new w.KeyboardEvent("keydown", { key: value, bubbles: true, cancelable: true, ...extra });
        target.dispatchEvent(event); return event;
    };
    const help = () => doc.querySelector(".help-menu > a");
    const settings = () => doc.querySelector(".settings-menu > a");
    return { dom, w, doc, key, help, settings };
}
test("native Help content and ? work independently of missing search data and denied storage", () => {
    const s = setup({ search: false, denied: true });
    const event = s.key(s.doc.body, "?");
    assert.equal(event.defaultPrevented, true);
    assert.ok(s.doc.querySelector(".help-menu #help.popover > .content > .side-by-side > .shortcuts dl kbd"));
    assert.ok(s.doc.querySelector("#help .infos"));
    assert.equal(s.help().getAttribute("aria-expanded"), "true");
    assert.equal(s.doc.activeElement, s.help());
    const text = s.doc.querySelector("#help").textContent;
    assert.ok(text.includes("Keyboard Shortcuts") && text.includes("Search Tricks"));
    assert.deepEqual([...s.doc.querySelectorAll("#help kbd")].map(k => k.textContent), ["?", "S", "/", "↑", "↓", "←", "→", "⏎"]);
    assert.equal(s.doc.querySelector("#help .bottom code").textContent, "cargo-clippy-html test-version");
    assert.ok(!text.includes("Expand all") && !text.includes("fn:") && !text.includes("rustdoc book"));
    s.dom.window.close();
});
test("Escape consumes Help or Settings before search and restores trigger focus", async () => {
    const s = setup();
    s.key(s.doc.body, "/");
    s.help().click();
    s.key(s.help(), "Escape");
    assert.equal(s.doc.querySelector(".help-menu #help"), null);
    assert.ok(s.doc.querySelector("#main-content").classList.contains("hidden"));
    assert.equal(s.doc.activeElement, s.help());
    s.settings().click();
    s.doc.body.dispatchEvent(new s.w.CompositionEvent("compositionstart", { bubbles: true }));
    s.key(s.doc.querySelector("#settings input"), "Escape");
    assert.equal(s.settings().getAttribute("aria-expanded"), "true");
    s.doc.body.dispatchEvent(new s.w.CompositionEvent("compositionend", { bubbles: true }));
    s.key(s.doc.querySelector("#settings input"), "Escape");
    assert.equal(s.settings().getAttribute("aria-expanded"), "false");
    assert.ok(s.doc.querySelector("#main-content").classList.contains("hidden"));
    assert.equal(s.doc.activeElement, s.settings());
    s.key(s.settings(), "Escape");
    assert.ok(!s.doc.querySelector("#main-content").classList.contains("hidden"));
    await Promise.resolve();
    s.dom.window.close();
});
test("Help and Settings exclude each other; opening search closes Help", () => {
    const s = setup();
    s.settings().click(); s.help().click();
    assert.equal(s.settings().getAttribute("aria-expanded"), "false");
    assert.ok(s.doc.querySelector(".help-menu #help"));
    s.settings().click();
    assert.equal(s.doc.querySelector(".help-menu #help"), null);
    assert.equal(s.settings().getAttribute("aria-expanded"), "true");
    s.help().click(); s.key(s.help(), "/");
    assert.equal(s.doc.querySelector(".help-menu #help"), null);
    assert.equal(s.doc.activeElement, s.doc.querySelector(".search-input"));
    s.dom.window.close();
});
test("modified links stay native and popup internal focus does not dismiss it", async () => {
    const s = setup();
    const native = [];
    // Observe the app's decision, then suppress jsdom's unimplemented navigation.
    s.help().addEventListener("click", event => {
        if (event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) {
            native.push(!event.defaultPrevented); event.preventDefault();
        }
    });
    for (const modifier of ["ctrlKey", "altKey", "metaKey", "shiftKey"]) {
        const event = new s.w.MouseEvent("click", { bubbles: true, cancelable: true, [modifier]: true });
        s.help().dispatchEvent(event);
        assert.equal(native.at(-1), true);
        assert.equal(s.doc.querySelector(".help-menu #help"), null);
    }
    s.help().click();
    const panel = s.doc.querySelector(".help-menu #help");
    const link = s.doc.createElement("a"); link.href = "#inside"; link.textContent = "inside"; panel.append(link);
    s.help().dispatchEvent(new s.w.FocusEvent("focusout", { bubbles: true, relatedTarget: link }));
    link.focus(); await Promise.resolve();
    assert.ok(s.doc.querySelector(".help-menu #help"));
    s.doc.body.dispatchEvent(new s.w.MouseEvent("click", { bubbles: true }));
    assert.equal(s.doc.querySelector(".help-menu #help"), null);
    s.dom.window.close();
});
test("editable content, composition, modifiers and handled events retain their keys", () => {
    const s = setup();
    for (const tag of ["input", "textarea", "select", "div"]) {
        const field = s.doc.createElement(tag);
        if (tag === "div") field.contentEditable = "true", field.setAttribute("contenteditable", "true");
        s.doc.body.append(field); field.focus();
        for (const key of ["?", "s", "/"]) assert.equal(s.key(field, key).defaultPrevented, false);
    }
    for (const extra of [{ ctrlKey: true }, { altKey: true }, { metaKey: true }, { isComposing: true }, { keyCode: 229 }]) {
        assert.equal(s.key(s.doc.body, "?", extra).defaultPrevented, false);
    }
    s.doc.body.dispatchEvent(new s.w.CompositionEvent("compositionstart", { bubbles: true }));
    for (const value of ["?", "s", "/"]) assert.equal(s.key(s.doc.body, value).defaultPrevented, false);
    assert.equal(s.doc.querySelector(".help-menu #help"), null);
    s.doc.body.dispatchEvent(new s.w.CompositionEvent("compositionend", { bubbles: true }));
    const event = new s.w.KeyboardEvent("keydown", { key: "?", bubbles: true, cancelable: true }); event.preventDefault(); s.doc.body.dispatchEvent(event);
    assert.equal(s.doc.querySelector(".help-menu #help"), null);
    s.dom.window.close();
});
test("disable-shortcuts suppresses custom global and result keys but keeps controls usable", () => {
    const s = setup();
    s.key(s.doc.body, "/");
    s.w.ClippyHtmlSettings.set("disable-shortcuts", "true");
    assert.equal(s.key(s.doc.querySelector(".search-input"), "ArrowDown").defaultPrevented, false);
    for (const key of ["?", "s", "/", "Escape"]) assert.equal(s.key(s.doc.body, key).defaultPrevented, false);
    assert.ok(s.doc.querySelector("#main-content").classList.contains("hidden"));
    s.help().click(); assert.ok(s.doc.querySelector(".help-menu #help"));
    s.help().click(); assert.equal(s.doc.querySelector(".help-menu #help"), null);
    s.dom.window.close();
});
test("standalone Help is already readable without JS and initializes without duplication", () => {
    const s = setup({ standalone: true, search: false });
    assert.equal(s.doc.querySelectorAll("#help").length, 1);
    assert.ok(s.doc.querySelector("#help dl").textContent.includes("⏎"));
    assert.equal(s.doc.querySelector("#back").getAttribute("href"), "index.html");
    assert.equal(s.doc.querySelector(".help-menu"), null);
    s.dom.window.close();
});
test("a source page named help.html still opens the Help popover", () => {
    const s = setup({ url: "https://example.test/report/src/help.html" });
    s.key(s.doc.body, "?");
    assert.ok(s.doc.querySelector(".help-menu #help.popover"));
    s.dom.window.close();
});
