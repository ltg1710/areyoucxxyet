#include "pingpong/include/ping.h"
#include "pingpong/src/main.rs.h"
#include <iostream>

using namespace pingpong;

void pingpong::ping(int times) {
    if (times > 0) {
        std::cout << "cxx  :: ping(" << times << ")" << std::endl;
        pong(times);
    }
}