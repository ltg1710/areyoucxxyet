fn main() {
    let dst = cmake::Config::new("hellolib")
        .define("BUILD_LIBRARY_TYPE", "Static")
        .define("INSTALL_DIR_LIB", "lib")
        .define("INSTALL_DIR_INCLUDE", "include")
        .build();

    if cfg!(debug_assertions) {
        println!("cargo:rustc-link-search=native={}", dst.join("build/src/Debug").display());
    } else {
        println!("cargo:rustc-link-search=native={}", dst.join("build/src/Release").display());
    }
    let mut build = cxx_build::bridge("src/lib.rs");

    build
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .include(format!("{}", dst.join("include").display()))
        .include("include")
        .file("cpp/wrapper.cc")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=hello");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=cpp/wrapper.cc");
    println!("cargo:rerun-if-changed=build.rs");
}