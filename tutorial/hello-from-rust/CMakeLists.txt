project(hello_from_rust)
cmake_minimum_required(VERSION 3.10)

include_directories(${CMAKE_BINARY_DIR}/rust-hello)
include_directories(include)

add_subdirectory(rust-hello)

add_executable(hello_from_rust src/main.cc)
target_link_libraries(hello_from_rust rust_hello)
