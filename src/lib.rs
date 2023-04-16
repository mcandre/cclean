//! cclean provides predicates for conveniently cleaning up cmake artifacts.

extern crate lazy_static;

use std::env;
use std::fs;
use std::path;
use std::process;

lazy_static::lazy_static! {
    /// CLEAN_TASKS collects the assorted sub-cleaning tasks.
    pub static ref CLEAN_TASKS: Vec<CleanTask> = vec![
        run_cmake_global_clean_target,
        remove_ninja_log,
        remove_conan_packages,
        remove_build_dir,
    ];
}

/// run_cmake_global_clean_target executes the cmake global clean target.
///
/// The cmake global clean target performs best when the project's specific artifacts
/// are configured according to the cmake variable ADDITIONAL_CLEAN_FILES.
pub fn run_cmake_global_clean_target(build_dir: &str) -> Result<(), String> {
    process::Command::new("cmake")
        .args(["--build", build_dir, "--target", "clean"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status()
        .map_err(|_| "unable to launch cmake global clean target".to_string())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err("unable to complete cmake global clean target".to_string())
            }
        })
}

/// remove_ninja_log deletes any .ninja_log files in the build directory.
pub fn remove_ninja_log(build_dir: &str) -> Result<(), String> {
    fs::remove_file(path::Path::new(build_dir).join(".ninja_log"))
        .map_err(|_| "unable to remove .ninja_log from the build directory".to_string())
}

/// remove_conan_packages attempts to remove any cached Conan packages.
///
/// Note that Conan may be incompatible with custom / out of source build directories.
pub fn remove_conan_packages(_: &str) -> Result<(), String> {
    process::Command::new("conan")
        .args(["remove", "-f", "*"])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status()
        .map_err(|_| "unable to launch conan for package removal".to_string())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err("unable to complete conan package removal".to_string())
            }
        })
}

/// remove_build_dir tentatively deletes the build directory.
///
/// If the directory is logically distinct from the current working directory (CWD)
/// and the build directory is empty,
/// then the directory is removed.
///
/// Warning: Behavior may vary when build_dir and CWD refer to different file systems.
pub fn remove_build_dir(build_dir: &str) -> Result<(), String> {
    env::current_dir()
        .map_err(|_| "unable to access current working directory".to_string())
        .and_then(|cwd| {
            path::Path::new(build_dir)
                .canonicalize()
                .map_err(|_| "unable to canonicalize build directory".to_string())
                .and_then(|build_dir_abs| {
                    cwd.canonicalize()
                        .map_err(|_| "unable to canonicalize current working directory".to_string())
                        .and_then(|cwd_abs| {
                            if build_dir_abs == cwd_abs {
                                Ok(())
                            } else {
                                fs::remove_dir(build_dir)
                                    .map_err(|_| "unable to remove build directory".to_string())
                            }
                        })
                })
        })
}

/// CleanTask models a cleanup task suitable for running in series with other cleanup tasks.
type CleanTask = fn(build_dir: &str) -> Result<(), String>;

/// clean runs [CLEAN_TASKS].
///
/// On success, returns an empty vector.
/// Otherwise, returns a vector of soft assertion errors.
///
pub fn clean(build_dir: &str) -> Vec<String> {
    let mut errs: Vec<String> = Vec::new();

    for clean_task in CLEAN_TASKS.iter() {
        _ = clean_task(build_dir).map_err(|s| errs.push(s));
    }

    errs
}
