// UI adapted from pinned rustdoc main.js/search.js under the MIT option.
// Rust Project Contributors; see licenses/rustdoc-MIT.txt and licenses/SEARCH.md.
"use strict";
(() => {
    const data = globalThis.CLIPPY_HTML_SEARCH_DATA;
    const engine = globalThis.ClippyHtmlSearch;
    const kinds = ["lint", "file", "finding", "category"];
    const labels = ["In Lints", "In Files", "In Findings", "In Categories"];
    const types = ["Lint", "File", "Finding", "Category"];
    const main = document.getElementById("main-content");
    const toolbar = document.querySelector("rustdoc-toolbar.report-toolbar");
    if (!Array.isArray(data) || !engine || !main || !toolbar) return;
    const root = new URL(document.body.dataset.reportRoot || "./", document.baseURI);
    const localHref = href => {
        const url = new URL(href, root);
        if (url.protocol !== root.protocol || url.host !== root.host || !url.pathname.startsWith(root.pathname)) {
            throw new Error("Nonlocal search destination");
        }
        return url.href;
    };
    // Do not expose a partially initialized control if the index cannot be read.
    let index;
    try {
        if (!data.every(entry => kinds.includes(entry.kind) && ["name", "context", "description", "href"].every(key => typeof entry[key] === "string"))) return;
        for (const entry of data) localHref(entry.href);
        index = engine.createIndex(data);
    } catch (_) { return; }
    const element = (tag, className, text) => {
        const node = document.createElement(tag);
        if (className) node.className = className;
        if (text !== undefined) node.textContent = text;
        return node;
    };
    const originalHeading = toolbar.parentElement;
    const originalTitle = document.title;
    const button = toolbar.querySelector("#search-button a");
    const label = button.querySelector(".label");
    const alternative = element("section", "content hidden");
    alternative.id = "alternative-display";
    const search = element("section"); search.id = "search";
    const heading = element("div", "main-heading search-results-main-heading");
    const nav = element("nav", "sub");
    const form = element("form", "search-form");
    form.append(element("span")); // rustdoc's Safari input-layout workaround.
    const input = element("input", "search-input");
    input.type = "search"; input.name = "search"; input.autocomplete = "off"; input.spellcheck = false;
    input.setAttribute("aria-label", "Search lint names, files, findings and categories");
    input.placeholder = "Type ‘S’ or ‘/’ to search…";
    form.append(input); nav.append(form); heading.append(nav, element("div", "search-switcher", "Search results"));
    const output = element("div", "search-out");
    const tabs = element("div"); tabs.id = "search-tabs"; tabs.setAttribute("role", "tablist");
    tabs.setAttribute("aria-label", "Search result kinds");
    const results = element("div"); results.id = "results";
    const buttons = [], panels = [];
    for (let i = 0; i < kinds.length; i++) {
        const tab = element("button", "", labels[i]); tab.type = "button"; tab.id = `search-tab-${kinds[i]}`;
        tab.setAttribute("role", "tab"); tab.setAttribute("aria-controls", `search-results-${kinds[i]}`);
        tab.append(element("span", "count", " (0)"));
        const panel = element("div", "search-results"); panel.id = `search-results-${kinds[i]}`;
        panel.setAttribute("role", "tabpanel"); panel.setAttribute("aria-labelledby", tab.id);
        tab.addEventListener("click", () => selectTab(i));
        tabs.append(tab); results.append(panel); buttons.push(tab); panels.push(panel);
    }
    output.append(tabs, results); search.append(heading, output); alternative.append(search);
    main.after(alternative);
    let displayed = false, currentTab = 0, timeout = null, composing = false;
    let previousFocus = null, scroll = [0, 0];
    const skip = document.querySelector(".skip-main-content");
    const clearTimeoutInput = () => { window.clearTimeout(timeout); timeout = null; };
    function selectTab(i) {
        currentTab = i;
        for (let n = 0; n < kinds.length; n++) {
            buttons[n].classList.toggle("selected", n === i);
            buttons[n].setAttribute("aria-selected", String(n === i));
            panels[n].classList.toggle("active", n === i);
            panels[n].setAttribute("aria-hidden", String(n !== i));
        }
    }
    function searchUrl() {
        const url = new URL(window.location.href); url.searchParams.set("search", input.value);
        return url;
    }
    function historyUpdate(method, url) {
        try { window.history[method](null, "", url.href); } catch (_) { /* file:// history may be unavailable. */ }
    }
    const examples = ["lint", "file", "category"].map(kind => data.find(entry => entry.kind === kind)?.name).filter(Boolean);
    if (!examples.length) examples.push("clippy::clone_on_copy", "src/", "complexity");
    function emptyState(panel) {
        if (input.value.trim()) panel.append(document.createTextNode("No results :("), element("br"), element("br"));
        panel.append(document.createTextNode("Example searches:"));
        const list = element("ul");
        for (const example of examples) {
            const item = element("li"), link = element("a", "", example);
            const url = new URL(window.location.href); url.searchParams.set("search", example); link.href = url.href;
            link.addEventListener("click", event => {
                if (event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
                event.preventDefault(); input.value = example; render(); input.focus();
            });
            item.append(link); list.append(item);
        }
        panel.append(list);
    }
    function render(updateHistory = true) {
        clearTimeoutInput();
        const matches = engine.query(index, input.value);
        for (let i = 0; i < kinds.length; i++) {
            const entries = matches[kinds[i]], panel = panels[i]; panel.replaceChildren();
            panel.className = entries.length ? "search-results" : "search-failed";
            buttons[i].querySelector(".count").textContent = ` (${entries.length})`;
            for (const entry of entries.slice(0, 100)) {
                const link = element("a"); link.href = localHref(entry.href);
                const name = element("span", "result-name");
                name.append(element("span", "typename", `${types[i]} `), element("span", "path", entry.name));
                const desc = element("div", "desc", entry.description); desc.title = entry.description;
                link.append(name, desc); panel.append(link);
            }
            if (!entries.length) emptyState(panel);
            if (entries.length > 100) panel.append(element("p", "", `Showing 100 of ${entries.length} results. Refine your search to see fewer results.`));
        }
        if (!matches[kinds[currentTab]].length) {
            const first = kinds.findIndex(kind => matches[kind].length);
            if (first !== -1) currentTab = first;
        }
        selectTab(currentTab);
        document.title = input.value.trim() ? `Results for ${input.value} — Clippy HTML` : `Search — Clippy HTML`;
        button.href = searchUrl().href;
        if (updateHistory) historyUpdate("replaceState", searchUrl());
        if (input.value.trim() && globalThis.ClippyHtmlSettings?.get("go-to-only-result") === "true"
            && kinds.reduce((total, kind) => total + matches[kind].length, 0) === 1) {
            const url = new URL(window.location.href); url.searchParams.delete("search");
            historyUpdate("replaceState", url);
            panels[currentTab].querySelector("a")?.click();
        }
    }
    function open(updateHistory = true, focus = true) {
        document.dispatchEvent(new Event("clippy-html-search-open"));
        if (!displayed) {
            previousFocus = document.activeElement; scroll = [window.scrollX, window.scrollY];
            displayed = true; main.classList.add("hidden"); alternative.classList.remove("hidden");
            heading.append(toolbar); label.textContent = "Exit"; button.setAttribute("aria-expanded", "true");
            if (skip) skip.href = "#alternative-display";
            if (updateHistory && !new URL(window.location.href).searchParams.has("search")) historyUpdate("pushState", searchUrl());
            window.scrollTo(0, 0);
        }
        render(updateHistory);
        if (focus && displayed) input.focus();
    }
    function close(updateHistory = true, restoreFocus = true) {
        clearTimeoutInput();
        if (!displayed) return;
        displayed = false; alternative.classList.add("hidden"); main.classList.remove("hidden");
        originalHeading.append(toolbar); label.textContent = "Search"; button.setAttribute("aria-expanded", "false");
        if (skip) skip.href = "#main-content";
        document.title = originalTitle;
        if (updateHistory) {
            const url = new URL(window.location.href); url.searchParams.delete("search"); historyUpdate("replaceState", url);
        }
        if (restoreFocus) {
            const target = previousFocus?.isConnected && previousFocus !== document.body ? previousFocus : button;
            target.focus({ preventScroll: true });
        }
        window.scrollTo(...scroll);
    }
    button.addEventListener("click", event => {
        if (event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        event.preventDefault(); if (displayed) close(); else open();
    });
    document.addEventListener("click", event => {
        if (!displayed || event.defaultPrevented || event.button !== 0 || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        const link = event.target instanceof Element ? event.target.closest("a[href]") : null;
        if (!link || link.hasAttribute("download") || (link.target && link.target !== "_self")) return;
        const destination = new URL(link.href), current = new URL(window.location.href);
        if (destination.protocol !== current.protocol || destination.host !== current.host || destination.pathname !== current.pathname || !destination.hash) return;
        let target;
        try { target = document.getElementById(decodeURIComponent(destination.hash.slice(1))); } catch (_) { return; }
        if (!target || !main.contains(target)) return;
        close(true, false);
        destination.searchParams.delete("search"); link.href = destination.href;
        target.scrollIntoView?.({ block: "start" });
        // Keep native anchor navigation/focus, including a reload if history is
        // denied and removing an existing search query requires navigation.
    });
    form.addEventListener("submit", event => { event.preventDefault(); if (!composing) render(); });
    function schedule() {
        clearTimeoutInput(); if (!composing) timeout = window.setTimeout(() => render(), 500);
    }
    input.addEventListener("input", schedule);
    input.addEventListener("compositionstart", () => { composing = true; clearTimeoutInput(); });
    input.addEventListener("compositionend", () => { composing = false; schedule(); });
    input.addEventListener("keydown", event => {
        if (event.isComposing || composing || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        if (event.key === "ArrowDown") {
            event.preventDefault(); if (timeout !== null) render(); panels[currentTab].querySelector("a")?.focus();
        }
    });
    output.addEventListener("keydown", event => {
        if (event.isComposing || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
            event.preventDefault(); const tabFocused = buttons.includes(document.activeElement);
            selectTab((currentTab + (event.key === "ArrowLeft" ? kinds.length - 1 : 1)) % kinds.length);
            if (tabFocused) buttons[currentTab].focus(); else (panels[currentTab].querySelector("a") || input).focus();
        } else if (event.key === "ArrowUp" || event.key === "ArrowDown") {
            const links = [...panels[currentTab].querySelectorAll("a")];
            const position = links.indexOf(document.activeElement); if (position === -1) return;
            event.preventDefault();
            const next = event.key === "ArrowUp" ? links[position - 1] || input : links[position + 1] || links[position];
            next.focus(); next.scrollIntoView?.({ block: "nearest" });
        }
    });
    document.addEventListener("keydown", event => {
        if (event.isComposing || composing || event.ctrlKey || event.altKey || event.metaKey || globalThis.ClippyHtmlSettings?.get("disable-shortcuts") === "true") return;
        if (event.key === "Escape" && displayed) { event.preventDefault(); close(); return; }
        const editable = event.target instanceof Element ? event.target.closest('input, textarea, select, [contenteditable]:not([contenteditable="false"])') : null;
        if (editable && !(editable instanceof HTMLInputElement && ["checkbox", "radio"].includes(editable.type))) return;
        if (event.key === "s" || event.key === "S" || event.key === "/") { event.preventDefault(); open(); }
    });
    function restoreUrl() {
        const url = new URL(window.location.href);
        if (url.searchParams.has("search")) { input.value = url.searchParams.get("search"); open(false, false); }
        else close(false, false);
    }
    window.addEventListener("popstate", restoreUrl);
    window.addEventListener("pageshow", restoreUrl);
    button.setAttribute("aria-controls", "alternative-display"); button.setAttribute("aria-expanded", "false");
    button.parentElement.classList.remove("hidden"); toolbar.classList.remove("hidden"); restoreUrl();
})();
