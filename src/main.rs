mod cli;
fn main() {
    if let Err(error) = cli::parse_args(std::env::args_os().collect()) {
        error.exit();
    }
}
