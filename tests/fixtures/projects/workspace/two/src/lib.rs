#![warn(clippy::clone_on_copy)]
pub fn first(value: i32) -> i32 { value.clone() }
pub fn second(value: i32) -> i32 { value.clone() }
