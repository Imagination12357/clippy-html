const test = require("node:test");
const assert = require("node:assert/strict");
const fs = require("node:fs");
const { JSDOM } = require("jsdom");

function setup({ clipboard, command } = {}) {
    const dom = new JSDOM('<!doctype html><button id="before">Before</button><details class="suggestion" open><div class="example-wrap suggestion-diff"><pre><code>visible diff</code></pre></div></details>', { url: "file:///report/lint/test.html", runScripts: "outside-only" });
    const { window: w } = dom;
    const patch = '--- "a/한 글.rs"\n+++ "b/한 글.rs"\n@@ -1 +1 @@\n-old\r\n+<script>new</script>\r\n';
    w.document.querySelector(".suggestion-diff").dataset.patch = JSON.stringify(patch);
    if (clipboard) Object.defineProperty(w.navigator, "clipboard", { value: clipboard });
    if (command) w.document.execCommand = command.bind(null, w);
    const script = fs.readFileSync("assets/suggestions.js", "utf8");
    w.eval(script);
    return { dom, w, patch, script, button: w.document.querySelector(".patch-copy") };
}
const tick = () => new Promise(resolve => setImmediate(resolve));

test("copies one exact complete patch through Clipboard API, not visible labels or markup", async () => {
    const copied = [];
    const { dom, w, patch, script, button } = setup({ clipboard: { writeText: async text => { copied.push(text); } } });
    assert.ok(button, "copy control initializes without search/settings");
    assert.equal(button.type, "button");
    w.eval(script);
    assert.equal(w.document.querySelectorAll(".patch-copy").length, 1);
    button.click(); await tick();
    assert.deepEqual(copied, [patch]);
    assert.equal(button.textContent, "Copied");
    assert.equal(w.document.querySelector("pre code").textContent, "visible diff");
    assert.equal(w.document.querySelectorAll("script").length, 0);
    dom.window.close();
});
test("denied Clipboard API uses the selected-text fallback and restores focus", async () => {
    let copied;
    const { dom, w, patch, button } = setup({ clipboard: { writeText: async () => { throw Error("denied"); } }, command: (window, name) => {
        assert.equal(name, "copy");
        const event = new window.Event("copy", { bubbles: true, cancelable: true });
        Object.defineProperty(event, "clipboardData", { value: { setData: (type, text) => { assert.equal(type, "text/plain"); copied = text; } } });
        window.document.activeElement.dispatchEvent(event);
        assert.equal(event.defaultPrevented, true);
        return true;
    } });
    button.focus(); button.click(); await tick();
    assert.equal(copied, patch);
    assert.equal(w.document.activeElement, button);
    assert.equal(button.textContent, "Copied");
    assert.equal(w.document.querySelectorAll("textarea").length, 0);
    dom.window.close();
});
test("failed or unavailable clipboard stays selectable and never reports success", async () => {
    for (const command of [undefined, () => false, () => { throw Error("blocked"); }]) {
        const { dom, w, button } = setup({ command });
        button.click(); await tick();
        assert.equal(button.textContent, "Copy failed");
        assert.equal(button.disabled, false);
        assert.equal(w.document.querySelectorAll("textarea").length, 0);
        assert.equal(w.document.querySelector("pre code").textContent, "visible diff");
        dom.window.close();
    }
});
test("malformed patch data gets no active copy control; fallback replacements are untouched", () => {
    const { dom, w, script } = setup();
    const wrap = w.document.createElement("div"); wrap.className = "example-wrap suggestion-diff";
    wrap.dataset.patch = "invalid JSON"; wrap.textContent = "original replacement";
    w.document.body.append(wrap); w.eval(script);
    assert.equal(wrap.querySelector("button"), null);
    assert.equal(wrap.textContent, "original replacement");
    dom.window.close();
});
