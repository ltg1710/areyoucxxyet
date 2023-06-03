# areyoucxxyet
are you cxx yet  
this project have test on `x86_64-pc-windows-msvc`

# Tutorial aboue cxx crates

## 1 Hello from Cpp
you can run it by `cargo run --bin hello-from-cpp`.

``` mermaid
flowchart LR
    subgraph cpp["fab:fa-c++ C++"]
        hellolib
    end
    subgraph rust["fab:fa-rust Rust"]
        hello-sys
        hello-from-cpp
    end
    hellolib --> hello-sys
    hello-sys --> hello-from-cpp
    click hellolib "https://github.com/ltg1710/areyoucxxyet/blob/main/tutorial\crates\hello-sys\hellolib" _blank
    click hello-sys "https://github.com/ltg1710/areyoucxxyet/blob/main/tutorial\crates\hello-sys" _blank
    click hello-from-cpp "https://github.com/ltg1710/areyoucxxyet/blob/main/tutorial\hello-from-cpp" _blank
``` 

## 2 PingPong
you can run it by `cargo run --bin pingpong`.

``` mermaid
flowchart LR
    subgraph cpp["fab:fa-c++"]
        ping
    end
    subgraph sys["fab:fa-rust rust"]
        pong
    end
    subgraph app["fas:fa-table-tennis rust"]
        pingpong
    end
    app --> ping
    ping --> pong
    pong --> ping
    click ping "https://github.com/ltg1710/areyoucxxyet/blob/main/tutorial/pingpong/src/ping.cc" _blank
    click pong "https://github.com/ltg1710/areyoucxxyet/blob/main/tutorial/pingpong/src/main.rs#L13-L18" _blank
    click pingpong "https://github.com/ltg1710/areyoucxxyet/blob/main/tutorial/pingpong/src/main.rs#L20-L22" _blank
```