//! Cross-platform subprocess fixture for pipe saturation and early failures.
use std::io::{self, Write};
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("flood") => {
            let stderr = std::thread::spawn(|| {
                let chunk = vec![b'x'; 8192];
                for _ in 0..520 {
                    io::stderr().write_all(&chunk).unwrap();
                }
            });
            let line = format!("{}\n", "x".repeat(8191));
            for _ in 0..520 {
                print!("{line}");
            }
            stderr.join().unwrap();
            println!("{{\"reason\":\"build-finished\",\"success\":true}}");
        }
        Some("malformed") => {
            println!("{{\"reason\":");
            println!("{{\"reason\":\"build-finished\",\"success\":true}}");
        }
        Some("early-failure") => {
            eprintln!("Clippy unavailable");
            std::process::exit(101);
        }
        Some("no-finish") => println!("stopped"),
        Some("false-success") => println!("{{\"reason\":\"build-finished\",\"success\":false}}"),
        _ => println!("{{\"reason\":\"build-finished\",\"success\":true}}"),
    }
}
