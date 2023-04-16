//! CLI unmake tool

extern crate cclean;
extern crate die;
extern crate getopts;

use die::{die, Die};
use std::env;

/// CLI entrypoint
fn main() {
    let brief: String = format!("Usage: {} [OPTIONS]", env!("CARGO_PKG_NAME"));

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optopt("B", "", "custom build directory", "<path>");
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);
    let build_dir: String = optmatches.opt_str("B").unwrap_or(".".to_string());

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    cclean::clean(&build_dir);
}
