// Report-owned matching; no rustdoc API/type-search engine is included.
"use strict";
(() => {
    const normalize = text => text.normalize("NFC").toLowerCase();
    function createIndex(entries) {
        return entries.map((entry, order) => {
            const name = normalize(entry.name);
            return { entry, order, name, text: normalize(`${entry.name}\n${entry.context}\n${entry.description}`) };
        });
    }
    function query(index, text) {
        const result = { lint: [], file: [], finding: [], category: [] };
        const needle = normalize(text.trim());
        if (!needle) return result;
        const terms = needle.split(/\s+/u);
        const matches = index.filter(item => terms.every(term => item.text.includes(term)));
        const rank = item => item.name === needle ? 0 : item.name.startsWith(needle) ? 1 : item.name.includes(needle) ? 2 : 3;
        matches.sort((a, b) => rank(a) - rank(b) || a.order - b.order);
        for (const item of matches) result[item.entry.kind].push(item.entry);
        return result;
    }
    globalThis.ClippyHtmlSearch = { createIndex, query };
})();
