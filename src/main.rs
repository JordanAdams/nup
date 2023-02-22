mod package;
mod version;

use clap::{Arg, Command};

fn main() {
    env_logger::init();

    let _args = Command::new("nup")
        .args([Arg::new("version").required(true)])
        .get_matches();
}
