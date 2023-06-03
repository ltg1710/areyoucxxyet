fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/ping.cc")
        .flag_if_supported("-std=c++14")
        .compile("pingpong");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/ping.cc");
    println!("cargo:rerun-if-changed=include/ping.h");
    println!("cargo:rerun-if-changed=build.rs");
}