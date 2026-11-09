// Theme/state adapter derived from pinned rustdoc storage.js (MIT option).
// Rust Project Contributors; see licenses/rustdoc-MIT.txt and licenses/SETTINGS.md.
"use strict";
(() => {
    const themes = ["light", "dark", "ayu"];
    const defaults = { theme: "light", "use-system-theme": "true", "preferred-light-theme": "light", "preferred-dark-theme": "dark",
        "go-to-only-result": "false", "hide-sidebar": "false", "hide-toc": "false", "disable-shortcuts": "false",
        "sans-serif-fonts": "false", "word-wrap-source-code": "false" };
    const classes = ["hide-sidebar", "hide-toc", "sans-serif-fonts", "word-wrap-source-code"];
    const volatile = new Map();
    function read(name) {
        if (volatile.has(name)) return volatile.get(name);
        try { return window.localStorage.getItem(`clippy-html-${name}`); } catch (_) { return null; }
    }
    function write(name, value) {
        try { window.localStorage.setItem(`clippy-html-${name}`, value); volatile.delete(name); }
        catch (_) { volatile.set(name, value); }
    }
    function valid(name, value) {
        if (!Object.hasOwn(defaults, name)) return false;
        return ["theme", "preferred-light-theme", "preferred-dark-theme"].includes(name)
            ? themes.includes(value) : value === "true" || value === "false";
    }
    function get(name) {
        if (!Object.hasOwn(defaults, name)) return null;
        const value = read(name);
        return valid(name, value) ? value : defaults[name] ?? null;
    }
    const media = window.matchMedia ? window.matchMedia("(prefers-color-scheme: dark)") : null;
    function apply() {
        let theme = get("theme");
        if (get("use-system-theme") !== "false") {
            theme = get(media?.matches ? "preferred-dark-theme" : "preferred-light-theme");
            write("use-system-theme", "true"); write("theme", theme);
        }
        document.documentElement.dataset.theme = theme;
        for (const name of classes) document.documentElement.classList.toggle(name, get(name) === "true");
        document.dispatchEvent(new CustomEvent("clippy-html-settings-change"));
    }
    function set(name, value) {
        if (name === "theme" && value === "system preference") {
            write("use-system-theme", "true");
        } else {
            if (!valid(name, value)) return false;
            if (name === "theme") write("use-system-theme", "false");
            write(name, value);
        }
        apply(); return true;
    }
    globalThis.ClippyHtmlSettings = { get, set, apply };
    media?.addEventListener("change", apply);
    window.addEventListener("pageshow", event => { if (event.persisted) apply(); });
    apply();
})();
