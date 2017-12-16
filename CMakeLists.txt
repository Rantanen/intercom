cmake_minimum_required(VERSION 3.8)
project(InterCom)

# The version number.
set (InterCom_VERSION_MAJOR 0)
set (InterCOm_VERSION_MINOR 1)
set (InterCOm_VERSION_PATCH 1)

set(CMAKE_BINARY_DIR ${CMAKE_SOURCE_DIR}/build)

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(test/cpp-raw)