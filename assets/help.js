// Help DOM and popover behavior adapted from rustdoc 1.97.1 under the MIT option.
// Rust Project Contributors; see licenses/rustdoc-MIT.txt and licenses/HELP.md.
"use strict";
(() => {
    if (document.querySelector("#main-content > #help")) {
        document.getElementById("back")?.addEventListener("click", event => {
            if (event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey || !document.referrer) return;
            const root = new URL(document.body.dataset.reportRoot || "./", document.baseURI);
            const previous = new URL(document.referrer);
            if (previous.protocol === root.protocol && previous.host === root.host && previous.pathname.startsWith(root.pathname) && previous.pathname !== window.location.pathname && window.history.length > 1) {
                event.preventDefault(); window.history.back();
            }
        });
        return;
    }
    const menu = document.querySelector(".help-menu");
    const button = menu?.querySelector("a");
    const template = document.getElementById("help-content");
    if (!menu || !button || !(template instanceof HTMLTemplateElement)) return;
    let panel = null, composing = false;
    const disabled = () => globalThis.ClippyHtmlSettings?.get("disable-shortcuts") === "true";
    button.setAttribute("aria-controls", "help"); button.setAttribute("aria-expanded", "false");
    function close(focus = false) {
        if (!panel) return;
        panel.remove(); panel = null; button.setAttribute("aria-expanded", "false");
        if (focus) button.focus({ preventScroll: true });
    }
    function open() {
        if (panel) { button.focus({ preventScroll: true }); return; }
        document.dispatchEvent(new Event("clippy-html-help-open"));
        panel = template.content.firstElementChild.cloneNode(true);
        panel.classList.add("popover"); menu.append(panel);
        button.setAttribute("aria-expanded", "true"); button.focus({ preventScroll: true });
    }
    button.addEventListener("click", event => {
        if (event.defaultPrevented || event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        event.preventDefault(); if (panel) close(); else open();
    });
    document.addEventListener("click", event => { if (!menu.contains(event.target)) close(); });
    menu.addEventListener("focusout", event => {
        if (menu.contains(event.relatedTarget)) return;
        queueMicrotask(() => { if (!menu.contains(document.activeElement)) close(); });
    });
    document.addEventListener("compositionstart", () => { composing = true; });
    document.addEventListener("compositionend", () => { composing = false; });
    document.addEventListener("keydown", event => {
        if (event.defaultPrevented || event.isComposing || composing || event.keyCode === 229 || event.ctrlKey || event.altKey || event.metaKey || disabled()) return;
        if (event.key === "Escape" && panel) { event.preventDefault(); close(true); return; }
        const editable = event.target instanceof Element ? event.target.closest('input, textarea, select, [contenteditable]:not([contenteditable="false"])') : null;
        if (editable && !(editable instanceof HTMLInputElement && ["checkbox", "radio"].includes(editable.type))) return;
        if (event.key === "?") { event.preventDefault(); open(); }
    }, true);
    document.addEventListener("clippy-html-search-open", () => close(menu.contains(document.activeElement)));
    document.addEventListener("clippy-html-settings-open", () => close());
    menu.closest("rustdoc-toolbar")?.classList.remove("hidden");
})();
