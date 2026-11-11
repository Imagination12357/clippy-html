/* Report-specific whole-suggestion copy, using rustdoc's code-block structure. */
"use strict";
(() => {
    function fallbackCopy(text) {
        const active = document.activeElement;
        const selection = window.getSelection();
        const ranges = [];
        if (selection) for (let i = 0; i < selection.rangeCount; i++) ranges.push(selection.getRangeAt(i).cloneRange());
        const field = document.createElement("textarea");
        field.value = text;
        field.readOnly = true;
        field.style.position = "fixed";
        field.style.left = "-9999px";
        document.body.append(field);
        let handled = false;
        const onCopy = event => {
            event.preventDefault();
            if (event.clipboardData) {
                try {
                    event.clipboardData.setData("text/plain", text);
                    handled = true;
                } catch (_) { /* Leave failure visible if the event cannot write. */ }
            }
        };
        document.addEventListener("copy", onCopy, true);
        try {
            field.focus(); field.select();
            return typeof document.execCommand === "function" && document.execCommand("copy") && handled;
        } catch (_) {
            return false;
        } finally {
            document.removeEventListener("copy", onCopy, true);
            field.remove();
            if (active && active.isConnected) active.focus({ preventScroll: true });
            if (selection) {
                selection.removeAllRanges();
                for (const range of ranges) selection.addRange(range);
            }
        }
    }
    for (const wrap of document.querySelectorAll(".suggestion-diff[data-patch]")) {
        if (wrap.querySelector(".patch-copy")) continue;
        let patch;
        try { patch = JSON.parse(wrap.dataset.patch); } catch (_) { continue; }
        if (typeof patch !== "string" || !patch) continue;
        const holder = document.createElement("div"); holder.className = "button-holder keep-visible";
        const button = document.createElement("button");
        button.type = "button"; button.className = "copy-button patch-copy";
        button.textContent = "Copy patch"; button.setAttribute("aria-live", "polite");
        let reset;
        let busy = false;
        button.addEventListener("click", async () => {
            if (busy) return;
            busy = true;
            window.clearTimeout(reset);
            button.setAttribute("aria-busy", "true");
            let copied = false;
            try {
                if (navigator.clipboard && typeof navigator.clipboard.writeText === "function") {
                    await navigator.clipboard.writeText(patch);
                    copied = true;
                }
            } catch (_) { /* Try the selected-text fallback used by rustdoc. */ }
            if (!copied) copied = fallbackCopy(patch);
            button.textContent = copied ? "Copied" : "Copy failed";
            busy = false;
            button.removeAttribute("aria-busy");
            reset = window.setTimeout(() => { button.textContent = "Copy patch"; }, 2000);
        });
        holder.append(button); wrap.append(holder);
    }
})();
