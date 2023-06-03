use hello_sys;

fn hello(name:String) {
    hello_sys::ffi::hello(name)
}
fn main() {
    hello("world".to_string());
}