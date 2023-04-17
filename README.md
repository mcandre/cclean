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

Like cmake, cclean encourages portable build steps. This enables cmake projects to build more reliably on difference machines.

# WARNING

Configure the cmake global `clean` target (esp. `ADDITIONAL_CLEAN_FILES`) carefully, in order to avoid accidents.

# Windows support

Windows users may enjoy cclean with either WSL or host-naive COMSPEC Windows environments (Command Prompt and PowerShell).

However, cmake fails to process `ADDITIONAL_CLEAN_FILES` when using the MSVC toolchain directly with cmake.

Fortunately, LLVM for Windows provides operational workarounds.

1. Install the Clang, such as with the [LLVM](https://community.chocolatey.org/packages/llvm) Chocolatey package.
2. Install the [GNU make](https://community.chocolatey.org/packages/make) and/or [ninja](https://community.chocolatey.org/packages/ninja) Chocolatey packages.
2. Manually clear your cmake project of any existing cmake MSVC artifacts.
3. Update any compiler-specific sections of your `CMakeLists.txt` build configuration, to target clang / clang++, rather than MSVC.
4. Initialize your cmake project with either the make generator or the Ninja generator:

* `cmake -G "Unix Makefiles" <build directory>`
* `cmake -G Ninja <build directory>`

Both of these cmake generator implementations will generally respect `ADDITIONAL_CLEAN_FILES`. Except for the `.ninja_log` file, which cclean knows to automatically remove.

## Further research

We are interested to hear reports of how cmake and cclean behave when using clang-cl or other generators.

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
