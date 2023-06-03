#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("hello-sys/include/wrapper.h");
        pub fn hello(name: String);
    }
}
