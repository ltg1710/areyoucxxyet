if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()

set(RUST_HELLO_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/rust_hello.lib")

set(RUST_HELLO_CXX "${CMAKE_CURRENT_BINARY_DIR}/rust_hello.cpp")
add_library(rust_hello STATIC ${RUST_HELLO_CXX})

add_custom_command(
    OUTPUT ${RUST_HELLO_CXX}
    COMMAND set CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}
    COMMAND set RUSTFLAGS="${RUST_FLAGS}"
    COMMAND ${CARGO_CMD}
    COMMAND copy ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust_hello/src/lib.rs.cc ${RUST_HELLO_CXX}
    COMMAND copy ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust_hello/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/rust_hello.h
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

target_link_libraries(rust_hello pthread dl ${RUST_HELLO_LIB})
