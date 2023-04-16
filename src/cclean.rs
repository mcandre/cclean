//! CLI unmake tool

extern crate die;
extern crate getopts;

use die::{die, Die};
use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

/// clean removes common cmake internal artifacts:
///
/// * cmake global clean task
/// * cached conan packages
/// * .ninja_log
fn clean(build_dir: &str) {
    _ = process::Command::new("cmake")
        .args(["--build", build_dir, "--target", "clean"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status();

    _ = fs::remove_file(path::Path::new(build_dir).join(".ninja_log"));

    _ = process::Command::new("conan")
        .args(["remove", "-f", "*"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status();

    if let Ok(cwd) = env::current_dir() {
        let build_dir_abs_result: Result<path::PathBuf, io::Error> =
            path::Path::new(build_dir).canonicalize();

        let cwd_abs_result: Result<path::PathBuf, io::Error> = cwd.canonicalize();

        if let (Ok(build_dir_abs), Ok(cwd_abs)) = (build_dir_abs_result, cwd_abs_result) {
            if build_dir_abs != cwd_abs {
                _ = fs::remove_dir(build_dir);
            }
        }
    }
}

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

    clean(&build_dir);
}
