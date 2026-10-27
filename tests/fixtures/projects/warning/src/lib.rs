#![warn(clippy::clone_on_copy)]
/// <script>alert("this is source text")</script>
pub fn copy_number(value: i32) -> i32 { value.clone() }
