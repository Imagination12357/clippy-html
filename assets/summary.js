// Finding toggle structure and toolbar action adapted from rustdoc 1.97.1 (MIT).
// Rust Project Contributors; see licenses/rustdoc-MIT.txt and licenses/SUMMARY.md.
"use strict";
(() => {
    const main = document.getElementById("main-content");
    const toolbar = document.querySelector("rustdoc-toolbar.report-toolbar");
    const toggles = main ? [...main.querySelectorAll(".finding > details.finding-toggle")] : [];
    if (!toolbar || !toggles.length || toolbar.querySelector("#toggle-all-docs")) return;
    const controlled = toggles.flatMap(toggle => [toggle, ...toggle.querySelectorAll("details")]);
    const button = document.createElement("button");
    button.id = "toggle-all-docs"; button.type = "button";
    const label = document.createElement("span"); label.className = "label"; button.append(label); toolbar.append(button);
    let composing = false;
    function setSearchState(hidden) {
        button.disabled = hidden;
        toolbar.classList.toggle("has-summary", !hidden);
    }
    function sync() {
        const open = toggles.some(toggle => toggle.open);
        button.classList.toggle("will-expand", !open);
        label.textContent = open ? "Summary" : "Show all";
        button.title = open ? "Collapse diagnostic details" : "Expand diagnostic details";
        setSearchState(main.classList.contains("hidden"));
    }
    function preserveVisibleFocus(toggle, opening) {
        const summary = toggle.querySelector(":scope > summary");
        const active = document.activeElement;
        if (opening ? summary.querySelector(".finding-location")?.contains(active)
            : toggle.contains(active) && !summary.contains(active)) {
            summary.focus({ preventScroll: true });
        }
    }
    function setExpanded(open) {
        if (button.disabled) return;
        for (const toggle of toggles) preserveVisibleFocus(toggle, open);
        for (const detail of controlled) detail.open = open;
        sync();
    }
    button.addEventListener("click", () => setExpanded(!toggles.some(toggle => toggle.open)));
    for (const toggle of toggles) toggle.addEventListener("toggle", sync);
    function reveal(target) {
        const finding = target?.closest(".finding");
        const toggle = finding?.querySelector(":scope > details.finding-toggle");
        if (!toggles.includes(toggle)) return;
        preserveVisibleFocus(toggle, true);
        toggle.open = true;
        for (let node = target; node && finding.contains(node); node = node.parentElement) {
            if (node instanceof HTMLDetailsElement) node.open = true;
        }
        sync();
    }
    function revealHash() {
        try { reveal(document.getElementById(decodeURIComponent(window.location.hash.slice(1)))); }
        catch (_) { /* Invalid fragments leave native navigation untouched. */ }
    }
    document.addEventListener("click", event => {
        if (event.defaultPrevented || event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        const link = event.target instanceof Element ? event.target.closest("a[href]") : null;
        if (!link || link.hasAttribute("download") || (link.target && link.target !== "_self")) return;
        const destination = new URL(link.href), current = new URL(window.location.href);
        if (destination.protocol !== current.protocol || destination.host !== current.host || destination.pathname !== current.pathname || !destination.hash) return;
        try { reveal(document.getElementById(decodeURIComponent(destination.hash.slice(1)))); }
        catch (_) { /* Keep native link behavior even for malformed fragments. */ }
    });
    for (const name of ["hashchange", "popstate", "pageshow"]) window.addEventListener(name, revealHash);
    document.addEventListener("compositionstart", () => { composing = true; });
    document.addEventListener("compositionend", () => { composing = false; });
    document.addEventListener("keydown", event => {
        if (event.defaultPrevented || button.disabled || event.isComposing || composing || event.keyCode === 229 || event.ctrlKey || event.altKey || event.metaKey || globalThis.ClippyHtmlSettings?.get("disable-shortcuts") === "true") return;
        const editable = event.target instanceof Element ? event.target.closest('input, textarea, select, [contenteditable]:not([contenteditable="false"])') : null;
        if (editable && !(editable instanceof HTMLInputElement && ["checkbox", "radio"].includes(editable.type))) return;
        if (["-", "+", "="].includes(event.key)) { event.preventDefault(); setExpanded(event.key !== "-"); }
    });
    document.addEventListener("clippy-html-search-open", () => setSearchState(true));
    document.addEventListener("clippy-html-search-close", sync);
    sync(); revealHash();
})();
