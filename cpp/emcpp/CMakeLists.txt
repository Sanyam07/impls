cmake_minimum_required(VERSION 3.10)
project(emcpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_FLAGS "-stdlib=libc++")
set(CMAKE_C_FLAGS "-stdlib=libc++")

find_package(Boost COMPONENTS REQUIRED)
include_directories(${Boost_INCLUDE_DIR})
add_subdirectory(impls)