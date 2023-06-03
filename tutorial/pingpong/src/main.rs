#[cxx::bridge(namespace = "pingpong")]
mod ffi {
    extern "Rust" {
        fn pong(times:i32);
    }

    unsafe extern "C++" {
        include!("pingpong/include/ping.h");
        fn ping(times:i32);
    }
}

pub fn pong(times:i32) {
    if times > 0 {
        println!("rust :: pong({times})");
        ffi::ping(times - 1);
    }
}

fn main() {
    ffi::ping(3);
}
