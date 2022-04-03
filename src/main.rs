use log::{info, warn};
use std::process;

use minigrep::Config;

fn main() {
    env_logger::init();
    info!("starting up");
    let config = Config::new();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    warn!("end");
}
