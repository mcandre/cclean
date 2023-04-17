cmake_minimum_required(VERSION 3.4)
set(CMAKE_MESSAGE_LOG_LEVEL "WARNING")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "bin" CACHE STRING "main artifact directory" FORCE)
project(example)

set_property(
    DIRECTORY .
    APPEND PROPERTY
    ADDITIONAL_CLEAN_FILES
    "${CMAKE_RUNTIME_OUTPUT_DIRECTORY};install_manifest.txt;Makefile;build.ninja;rules.ninja;.ninja_deps;Testing;CTestTestfile.cmake;CMakeFiles;CMakeCache.txt;cmake_install.cmake;conanbuildinfo.cmake;conanbuildinfo.txt;conaninfo.txt;graph_info.json;conan.lock;latex;html"
)

add_executable(hello hello.c)
