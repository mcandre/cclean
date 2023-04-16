//! CLI unmake tool

extern crate die;
extern crate getopts;
extern crate lazy_static;
extern crate regex;

use die::{die, Die};
use std::env;
use std::fs;
use std::path;
use std::process;

lazy_static::lazy_static! {
    /// CMAKE_CACHED_ARTIFACT_DIR_PATTERN matches cached cmake variable declarations
    /// for the main cmake artifact output directory path.
    pub static ref CMAKE_CACHED_ARTIFACT_DIR_PATTERN: regex::Regex = regex::Regex::new("^(CMAKE_RUNTIME_OUTPUT_DIRECTORY|EXECUTABLE_OUTPUT_PATH):STRING=(.*)$").unwrap();
}

/// clean removes common cmake internal artifacts:
///
/// * artifacts in the standard cmake cached directory variables CMAKE_RUNTIME_OUTPUT_DIRECTORY and EXECUTABLE_OUTPUT_PATH
/// * artifacts managed by the cmake global clean task, such as cmake ADDITIONAL_CLEAN_FILES paths
/// * common Doxygen artifacts
/// * common artifacts for make, ninja, and MSVC
/// * ctest artifacts
/// * conan artifacts, including packages installed with conan
fn clean() {
    let cmake_artifact_directories: Vec<String> = process::Command::new("cmake")
        .args(["-LA"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .map_err(|_| "unable to run cmake -LA".to_string())
        .and_then(|output| match output.status.success() {
            // work around cmake writing warning messages to stdout
            false => Err("error: unable to query cmake variable cache".to_string()),
            _ => String::from_utf8(output.stdout)
                .map_err(|_| "error: unable to decode cmake stdout stream".to_string()),
        })
        .map(|text| {
            text.lines()
                .filter(|line| CMAKE_CACHED_ARTIFACT_DIR_PATTERN.is_match(line))
                .map(|line| {
                    CMAKE_CACHED_ARTIFACT_DIR_PATTERN
                        .captures(line)
                        .and_then(|e| e.get(2))
                        .map(|e| e.as_str())
                        .unwrap()
                })
                .map(|e| e.to_string())
                .collect()
        })
        .unwrap_or(Vec::new());

    _ = process::Command::new("cmake")
        .args(["--build", ".", "--target", "clean"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status();

    let cmake_internal_directories: Vec<String> = [
        cmake_artifact_directories,
        vec![
            "html".to_string(),
            "latex".to_string(),
            "Testing".to_string(),
            "debug".to_string(),
            "x64".to_string(),
            "x86".to_string(),
            "CMakeFiles".to_string(),
        ],
    ]
    .concat();

    let cmake_internal_files: Vec<&str> = vec![
        "CTestTestfile.cmake",
        "debug.log",
        "install_manifest.txt",
        "Makefile",
        ".ninja_log",
        ".ninja_deps",
        "build.ninja",
        "cmake_install.cmake",
        "CMakeCache.txt",
        "conanbuildinfo.cmake",
        "conanbuildinfo.txt",
        "conaninfo.txt",
        "graph_info.json",
        "conan.lock",
    ];

    for cmake_internal_directory in cmake_internal_directories {
        _ = fs::remove_dir_all(path::Path::new(&cmake_internal_directory));
    }

    for cmake_internal_file in cmake_internal_files {
        let pth: &path::Path = path::Path::new(cmake_internal_file);

        if pth.exists() {
            _ = fs::remove_file(pth);
        }
    }

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
