//! Emit pinned text/binary assets and check CSS font resources before publication.
use std::path::Path;

macro_rules! font {
    ($name:literal) => {
        (
            concat!("static.files/", $name),
            include_bytes!(concat!("../../assets/vendor/rustdoc/fonts/", $name)),
        )
    };
}
macro_rules! font_license {
    ($name:literal) => {
        (
            concat!("licenses/", $name),
            include_bytes!(concat!("../../assets/vendor/rustdoc/fonts/", $name)),
        )
    };
}
const FILES: &[(&str, &[u8])] = &[
    (
        "licenses/SUGGESTIONS.md",
        include_bytes!("../../assets/vendor/rustdoc/SUGGESTIONS.md"),
    ),
    (
        "static.files/suggestions.js",
        include_bytes!("../../assets/suggestions.js"),
    ),
    (
        "static.files/settings-tabs.css",
        include_bytes!("../../assets/vendor/rustdoc/settings-tabs.css"),
    ),
    (
        "licenses/SETTINGS.md",
        include_bytes!("../../assets/vendor/rustdoc/SETTINGS.md"),
    ),
    (
        "static.files/settings.js",
        include_bytes!("../../assets/settings.js"),
    ),
    (
        "static.files/settings-storage.js",
        include_bytes!("../../assets/settings-storage.js"),
    ),
    (
        "static.files/search.js",
        include_bytes!("../../assets/search.js"),
    ),
    (
        "static.files/search-engine.js",
        include_bytes!("../../assets/search-engine.js"),
    ),
    (
        "licenses/CLIPPY-CATEGORIES.md",
        include_bytes!("../../assets/vendor/clippy/README.md"),
    ),
    (
        "licenses/clippy-MIT.txt",
        include_bytes!("../../assets/vendor/clippy/LICENSE-MIT"),
    ),
    (
        "static.files/favicon.svg",
        include_bytes!("../../assets/favicon/favicon.svg"),
    ),
    (
        "static.files/favicon.ico",
        include_bytes!("../../assets/favicon/favicon.ico"),
    ),
    (
        "licenses/FAVICON.md",
        include_bytes!("../../assets/favicon/README.md"),
    ),
    (
        "licenses/favicon-CC-BY-4.0.txt",
        include_bytes!("../../assets/favicon/CC-BY-4.0.txt"),
    ),
    (
        "static.files/rustdoc.css",
        include_bytes!("../../assets/vendor/rustdoc/rustdoc.css"),
    ),
    (
        "static.files/normalize.css",
        include_bytes!("../../assets/vendor/rustdoc/normalize.css"),
    ),
    (
        "static.files/fonts.css",
        include_bytes!("../../assets/vendor/rustdoc/fonts.css"),
    ),
    (
        "static.files/clippy-html.css",
        include_bytes!("../../assets/clippy-html.css"),
    ),
    (
        "licenses/rustdoc-MIT.txt",
        include_bytes!("../../assets/vendor/rustdoc/LICENSE-MIT"),
    ),
    (
        "licenses/rustdoc-COPYRIGHT.txt",
        include_bytes!("../../assets/vendor/rustdoc/COPYRIGHT.txt"),
    ),
    (
        "licenses/normalize-MIT.txt",
        include_bytes!("../../assets/vendor/rustdoc/normalize-LICENSE-MIT"),
    ),
    (
        "licenses/UPSTREAM.md",
        include_bytes!("../../assets/vendor/rustdoc/UPSTREAM.md"),
    ),
    (
        "licenses/FONTS.md",
        include_bytes!("../../assets/vendor/rustdoc/FONTS.md"),
    ),
    (
        "licenses/HIGHLIGHTING.md",
        include_bytes!("../../assets/vendor/rustdoc/HIGHLIGHTING.md"),
    ),
    (
        "licenses/SEARCH.md",
        include_bytes!("../../assets/vendor/rustdoc/SEARCH.md"),
    ),
    (
        "licenses/THIRD-PARTY-NOTICES.md",
        include_bytes!("../../THIRD-PARTY-NOTICES.md"),
    ),
    (
        "licenses/generator-MIT.txt",
        include_bytes!("../../LICENSE-MIT"),
    ),
    (
        "licenses/generator-APACHE.txt",
        include_bytes!("../../LICENSE-APACHE"),
    ),
    font!("FiraSans-Regular-0fe48ade.woff2"),
    font!("FiraSans-Italic-81dc35de.woff2"),
    font!("FiraSans-Medium-e1aa3f0a.woff2"),
    font!("FiraSans-MediumItalic-ccf7e434.woff2"),
    font!("SourceSerif4-Regular-6b053e98.ttf.woff2"),
    font!("SourceSerif4-It-ca3b17ed.ttf.woff2"),
    font!("SourceSerif4-Semibold-457a13ac.ttf.woff2"),
    font!("SourceSerif4-Bold-6d4fd4c0.ttf.woff2"),
    font!("SourceCodePro-Regular-8badfe75.ttf.woff2"),
    font!("SourceCodePro-It-fc8b9304.ttf.woff2"),
    font!("SourceCodePro-Semibold-aa29a496.ttf.woff2"),
    font!("NanumBarunGothic-13b3dcba.ttf.woff2"),
    font_license!("Fira-LICENSE.txt"),
    font_license!("SourceCodePro-LICENSE.txt"),
    font_license!("SourceSerif4-LICENSE.md"),
    font_license!("NanumBarunGothic-LICENSE.txt"),
];

pub(super) fn write_assets(destination: &Path) -> Result<(), String> {
    for directory in ["static.files", "licenses"] {
        std::fs::create_dir_all(destination.join(directory)).map_err(|e| e.to_string())?;
    }
    for (name, bytes) in FILES {
        std::fs::write(destination.join(name), bytes).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub(super) fn validate_font_resources(root: &Path) -> Result<(), String> {
    let path = root.join("static.files/fonts.css");
    let css = match std::fs::read_to_string(&path) {
        Ok(css) => css,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.to_string()),
    };
    let base = url::Url::from_file_path(&path).map_err(|_| "Invalid font CSS URL")?;
    // This is the fixed, generator-owned stylesheet, not an arbitrary CSS parser.
    for reference in css.split("url(").skip(1) {
        let (value, _) = reference.split_once(')').ok_or("Unclosed font URL")?;
        let value = value.trim().trim_matches(['\'', '"']);
        let linked = base.join(value).map_err(|e| e.to_string())?;
        let target = linked
            .to_file_path()
            .map_err(|_| "Nonlocal font resource")?
            .canonicalize()
            .map_err(|e| format!("Missing font resource {value}: {e}"))?;
        if !target.starts_with(root)
            || !target.is_file()
            || !target.extension().is_some_and(|ext| ext == "woff2")
        {
            return Err(format!("Invalid font resource: {value}"));
        }
    }
    Ok(())
}
