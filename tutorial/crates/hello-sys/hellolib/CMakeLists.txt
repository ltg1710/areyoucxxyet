CMAKE_MINIMUM_REQUIRED(VERSION 3.10)
# set the project name
PROJECT(HelloLib)

# include the project's directory
include_directories(${PROJECT_SOURCE_DIR})

set(CMAKE_INSTALL_PREFIX ${PROJECT_SOURCE_DIR})
set(HELLOLIB_INSTALL_LIB_DIR ${CMAKE_INSTALL_PREFIX}/lib)

# add the src directory
add_subdirectory(src)