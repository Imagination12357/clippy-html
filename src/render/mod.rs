//! Generate static documents from normalized data, without running Cargo.
mod assets;
mod highlight;
mod pages;
pub mod paths;
pub use pages::{render_report, validate_report};
mod navigation;
mod search;
