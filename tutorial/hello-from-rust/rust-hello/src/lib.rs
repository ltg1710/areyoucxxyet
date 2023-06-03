#[cxx::bridge]
mod ffi {
    #[namespace = "rust_hello"]
    extern "Rust" {
        fn hello(name: String);
    }
}

fn hello(name:String) {
    println!("Hello, {}!", name);
}
