#include <wrapper.h>

void hello(rust::String name) {
  return hello(std::string(name.c_str()));
}