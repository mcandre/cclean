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

* cclean assumes the current working directory for the cmake build directory, as in `cmake [-B] .`
* cclean uses portable commands to remove artifacts
* cclean runs independently of cmake, enabling it to remove sticky cmake artifacts like `.ninja_log`
* cclean integrates with the cmake global `clean` target, such as cmake `ADDITIONAL_CLEAN_FILES` file paths
* cclean removes artifacts for the standard cmake *cached* directory variables `CMAKE_RUNTIME_OUTPUT_DIRECTORY` and `EXECUTABLE_OUTPUT_PATH`
* cclean removes the common Doxygen artifact directories `html` and `latex`
* cclean removes common artifacts for `make`, `ninja`, and `MSVC`
* cclean removes `ctest` artifacts
* cclean removes `conan` artifacts, including packages installed via conan

# WARNING

```text
/!\ Use cclean at your own risk. /!\
```

cclean deletes files.

This is not a dry run.

Take appropriate precautions before running cclean, including:

* Read cclean documentation thoroughly.
* Maintain regular, reversible backups of your work on a separate host.
* Inspect staged version control changes before comitting them.
* Confirm working directory paths.
* Rename any manually written build configurations `Makefile`, `build.ninja`, etc. to a substantially different filename. Minor casing variations such as `makefile`, `Build.ninja`, etc. are NOT substantially different.

# CRATE

https://crates.io/crates/cclean

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
