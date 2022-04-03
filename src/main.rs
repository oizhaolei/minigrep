use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
