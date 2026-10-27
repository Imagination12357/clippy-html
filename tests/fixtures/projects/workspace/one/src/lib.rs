#![warn(clippy::clone_on_copy)]
#[path = "한글.rs"]
pub mod unicode;
pub fn first(value: i32) -> i32 { value.clone() }
pub fn second(value: i32) -> i32 { value.clone() }
