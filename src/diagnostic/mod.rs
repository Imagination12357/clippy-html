//! Structured diagnostics, independent of HTML and process execution.
pub mod dedup;
pub mod model;
pub mod normalize;
pub mod parse;

#[cfg(test)]
pub fn sample() -> serde_json::Value {
    serde_json::json!({"reason":"compiler-message", "package_id":"path+file:///demo#demo@0.1.0", "manifest_path":"/demo/Cargo.toml", "target":{"kind":["lib"],"crate_types":["lib"],"name":"demo","src_path":"/demo/src/lib.rs","edition":"2024","doc":true,"doctest":true,"test":true},"message":{"message":"unnecessary clone", "code":{"code":"clippy::clone_on_copy","explanation":null},"level":"warning","spans":[{"file_name":"src/lib.rs","byte_start":0,"byte_end":3,"line_start":1,"line_end":1,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"abc","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: unnecessary clone"}})
}
