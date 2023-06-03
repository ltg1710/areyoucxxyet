cmake_minimum_required(VERSION 3.2)

set(SOURCE_FILES hello.cc ../include/hello.h)

ADD_LIBRARY(hello STATIC ${SOURCE_FILES})

install(TARGETS hello DESTINATION ${HELLOLIB_INSTALL_LIB_DIR})