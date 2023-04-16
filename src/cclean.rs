//! CLI unmake tool

extern crate die;
extern crate getopts;

use die::{die, Die};
use std::env;
use std::fs;
use std::process;

/// clean removes common cmake internal artifacts:
///
/// * cmake global clean task
/// * cached conan packages
/// * .ninja_log
fn clean() {
    _ = process::Command::new("cmake")
        .args(["--build", ".", "--target", "clean"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status();

    _ = fs::remove_file(".ninja_log");

    _ = process::Command::new("conan")
        .args(["remove", "-f", "*"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status();
}

/// CLI entrypoint
fn main() {
    let brief: String = format!("Usage: {} [OPTIONS]", env!("CARGO_PKG_NAME"));

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    clean();
}
