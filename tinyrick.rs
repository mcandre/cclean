//! Build configuration

extern crate tinyrick;
extern crate tinyrick_extras;

use std::path;
use std::process;

/// Generate documentation
fn doc() {
    tinyrick_extras::build();
}

/// Security audit
fn audit() {
    tinyrick::exec!("cargo", &["audit"]);
}

/// Run clippy
fn clippy() {
    tinyrick_extras::clippy();
}

/// Run rustfmt
fn rustfmt() {
    tinyrick_extras::rustfmt();
}

/// Run unmake
fn unmake() {
    tinyrick::exec!("unmake", &["install.mk"]);
}

/// Validate documentation and run linters
fn lint() {
    tinyrick::deps(doc);
    tinyrick::deps(clippy);
    tinyrick::deps(rustfmt);
    tinyrick::deps(unmake);
}

/// Lint, and then install artifacts
fn install() {
    tinyrick::deps(lint);
    tinyrick::exec!("cargo", &["install", "--force", "--path", "."]);
}

/// Uninstall artifacts
fn uninstall() {
    tinyrick::exec!("cargo", &["uninstall"]);
}

/// Run tests
fn test() {
    let build_dir: &str = "build";

    assert!(
        tinyrick::exec_mut_with_arguments!("cmake", &["-B", &build_dir, "-G", "Ninja"])
            .current_dir("example")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .status()
            .unwrap()
            .success()
    );

    assert!(tinyrick::exec_mut_with_arguments!(
        "cmake",
        &["--build", &build_dir, "--target", "hello"]
    )
    .current_dir("example")
    .stdout(process::Stdio::piped())
    .stderr(process::Stdio::piped())
    .status()
    .unwrap()
    .success());

    assert!(
        tinyrick::exec_mut_with_arguments!("cclean", &["-B", &build_dir])
            .current_dir("example")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .status()
            .unwrap()
            .success()
    );

    assert!(!path::Path::new("example/.ninja_log").exists());
    assert!(!path::Path::new("example/build.ninja").exists());
    assert!(!path::Path::new("example/Makefile").exists());
    assert!(!path::Path::new("example/build").exists());
}

/// Build: Doc, lint, test, and compile
fn build() {
    tinyrick::deps(lint);
    tinyrick::deps(test);
    tinyrick_extras::build();
}

/// banner generates artifact labels.
fn banner() -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

/// Compress binaries.
fn archive() {
    let b: &str = &banner();

    let archive_basename: &str = &format!("{}.zip", b);
    let archive_path: &path::Path = path::Path::new(archive_basename);

    let archive_str: &str = &archive_path.display().to_string();

    let binary_artifacts_dir_str: &str =
        &path::Path::new(".crit").join("bin").display().to_string();

    assert!(tinyrick::exec_mut!("zip", &["-r", archive_str, b])
        .current_dir(binary_artifacts_dir_str)
        .status()
        .unwrap()
        .success());
}

/// Prepare cross-platform release media.
fn port() {
    tinyrick_extras::crit(vec!["-b".to_string(), banner()]);
    tinyrick::deps(archive);
}

/// Publish to crate repository
fn publish() {
    tinyrick_extras::publish();
}

/// Clean example
fn clean_example() {
    assert!(tinyrick::exec_mut!("cclean")
        .current_dir("example")
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .status()
        .unwrap()
        .success());
}

/// Clean ports
fn clean_ports() {
    assert!(tinyrick::exec_mut!("crit", &["-c"])
        .status()
        .unwrap()
        .success());
}

/// Clean workspaces
fn clean() {
    tinyrick::deps(clean_example);
    tinyrick_extras::clean_cargo();
    tinyrick::deps(clean_ports);
}

/// CLI entrypoint
fn main() {
    tinyrick::phony!(clean);

    tinyrick::wubba_lubba_dub_dub!(
        build;
        doc,
        install,
        uninstall,
        audit,
        clippy,
        rustfmt,
        unmake,
        lint,
        test,
        archive,
        port,
        publish,
        clean_ports,
        clean
    );
}
