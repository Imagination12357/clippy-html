// Settings form/popup adapter from pinned rustdoc settings.js (MIT option).
// Rust Project Contributors; see licenses/rustdoc-MIT.txt and licenses/SETTINGS.md.
"use strict";
(() => {
    const api = globalThis.ClippyHtmlSettings;
    if (!api) return;
    const standalone = window.location.pathname.endsWith("/settings.html");
    const main = document.getElementById("main-content");
    const menu = document.querySelector(".settings-menu");
    if (!main || (!standalone && !menu)) return;
    const themes = ["light", "dark", "ayu"];
    const settings = [
        ["theme", "Theme", [...themes, "system preference"]],
        ["preferred-light-theme", "Preferred light theme", themes],
        ["preferred-dark-theme", "Preferred dark theme", themes],
        ["go-to-only-result", "Directly go to item in search if there is only one result"],
        ["hide-sidebar", "Hide persistent navigation bar"],
        ["hide-toc", "Hide table of contents"],
        ["disable-shortcuts", "Disable keyboard shortcuts"],
        ["sans-serif-fonts", "Use sans serif fonts"],
        ["word-wrap-source-code", "Word wrap source code"],
    ];
    const element = (tag, className, text) => {
        const node = document.createElement(tag);
        if (className) node.className = className;
        if (text !== undefined) node.textContent = text;
        return node;
    };
    const panel = element(standalone ? "section" : "div", standalone ? "" : "popover");
    panel.id = "settings";
    const form = element("div", "settings");
    for (const [name, label, options] of settings) {
        const row = element("div", "setting-line");
        if (options) {
            row.id = name; row.append(element("div", "setting-radio-name", label));
            const choices = element("div", "setting-radio-choices");
            for (const value of options) {
                const choice = element("label", "setting-radio");
                const input = element("input"); input.type = "radio"; input.name = name; input.value = value;
                input.id = `${name}-${value.replace(/ /g, "-")}`; choice.htmlFor = input.id;
                choice.append(input, element("span", "", value)); choices.append(choice);
            }
            row.append(choices);
        } else {
            const choice = element("label", "setting-check"), input = element("input");
            input.type = "checkbox"; input.id = name;
            choice.append(input, element("span", "", label)); row.append(choice);
        }
        form.append(row);
    }
    panel.append(form);
    function sync() {
        const system = api.get("use-system-theme") !== "false";
        for (const row of panel.querySelectorAll("#preferred-light-theme, #preferred-dark-theme")) row.classList.toggle("hidden", !system);
        for (const input of panel.querySelectorAll("input")) {
            input.checked = input.type === "checkbox" ? api.get(input.id) === "true"
                : input.value === (input.name === "theme" && system ? "system preference" : api.get(input.name));
        }
    }
    panel.addEventListener("change", event => {
        const input = event.target;
        if (!(input instanceof HTMLInputElement)) return;
        if (input.type === "checkbox") api.set(input.id, String(input.checked));
        else if (input.checked) api.set(input.name, input.value);
    });
    document.addEventListener("clippy-html-settings-change", sync); sync();
    if (standalone) {
        main.append(panel);
        const back = document.getElementById("back");
        back?.addEventListener("click", event => {
            if (event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey || !document.referrer) return;
            const root = new URL(document.body.dataset.reportRoot || "./", document.baseURI);
            const previous = new URL(document.referrer);
            if (previous.protocol === root.protocol && previous.host === root.host && previous.pathname.startsWith(root.pathname) && previous.pathname !== window.location.pathname && window.history.length > 1) {
                event.preventDefault(); window.history.back();
            }
        });
        return;
    }
    const button = menu.querySelector("a");
    panel.tabIndex = -1; panel.style.display = "none"; menu.append(panel);
    button.setAttribute("aria-label", "Settings"); button.setAttribute("aria-controls", "settings");
    button.setAttribute("aria-expanded", "false");
    let opened = false;
    function close(focus = false) {
        if (!opened) return;
        opened = false; panel.style.display = "none"; button.setAttribute("aria-expanded", "false");
        if (focus) button.focus({ preventScroll: true });
    }
    button.addEventListener("click", event => {
        if (event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        event.preventDefault();
        if (opened) close();
        else { sync(); opened = true; panel.style.display = ""; button.setAttribute("aria-expanded", "true"); }
    });
    document.addEventListener("click", event => { if (!menu.contains(event.target)) close(); });
    menu.addEventListener("focusout", () => { queueMicrotask(() => { if (!menu.contains(document.activeElement)) close(); }); });
    document.addEventListener("keydown", event => {
        if (event.key === "Escape" && !event.isComposing && !event.ctrlKey && !event.altKey && !event.metaKey && api.get("disable-shortcuts") !== "true" && opened) {
            event.preventDefault(); close(true);
        }
    });
    document.addEventListener("clippy-html-search-open", () => close(menu.contains(document.activeElement)));
    menu.classList.remove("hidden"); menu.closest("rustdoc-toolbar")?.classList.remove("hidden");
})();
