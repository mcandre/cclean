# cclean: a cmake sanitizer

# SUMMARY

cclean cleans up cmake's messiest build artifacts.

# EXAMPLE

```console
$ cd example

$ cmake .
$ cmake --build . --target hello
$ tree
.
├── CMakeCache.txt
├── CMakeFiles
...
├── CMakeLists.txt
├── Makefile
├── bin
│   └── hello
├── cmake_install.cmake
└── hello.c

$ cclean
$ tree
.
├── CMakeLists.txt
└── hello.c
```

See `cclean -h` for more options.

# ABOUT

cclean assists cmake with removing common internal artifacts. This is helpful for freeing up disk space.

cmake often enters a corrupt state, requiring manual intervention. cclean automates the process of resetting cmake back to a clean state.

cclean is particularly helpful for cross-platform workflows. For example, when building the same host directory once in WSL, and then again in PowerShell.

# NOTABLE FEATURES

* cclean runs independently of cmake, enabling it to remove sticky cmake artifacts like `.ninja_log`
* cclean removes custom `-B` build directories
* cclean wraps the cmake global `clean` target
* cclean removes cached `conan` packages

Like cmake, cclean encourages portable build steps. This enables cmake projects to build more reliably on more environments.

# WARNING

Configure the cmake global `clean` target (esp. `ADDITIONAL_CLEAN_FILES`) carefully, in order to avoid accidents.

# CRATE

https://crates.io/crates/cclean

# API DOCUMENTATION

https://docs.rs/cclean/latest/cclean/

# INSTALL FROM SOURCE

```console
$ cargo install --force --path .
```

# RUNTIME REQUIREMENTS

* [cmake](https://cmake.org/) 3.4+

# CONTRIBUTING

For more details on developing cclean itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# LICENSE

FreeBSD

# SEE ALSO

* [git](https://git-scm.com/), a distributed version control system
* [gitignore.io](https://www.toptal.com/developers/gitignore), community maintained gitignore templates
* [make](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/make.html), the classic, plain build system
* [ninja](https://ninja-build.org/), the fast, plain build system
* [unmake](https://github.com/mcandre/unmake), a linter for manually written makefiles
