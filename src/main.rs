mod cargo;
mod cli;
mod diagnostic;
mod render;
mod source;
fn main() {
    if let Err(error) = cli::parse_args(std::env::args_os().collect()) {
        error.exit();
    }
}
