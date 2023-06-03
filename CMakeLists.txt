CMAKE_MINIMUM_REQUIRED(VERSION 3.10)
# set the project name
PROJECT(HelloCxx)

# include the project's directory
include_directories(${PROJECT_SOURCE_DIR})

# add the sub directory
add_subdirectory(tutorial/hello-from-rust)
