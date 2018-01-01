if( ${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR} )
    message( FATAL_ERROR "In-source builds not allowed. Please make a new directory and run CMake from there. You may need to remove CMakeCache.txt." )
endif()

cmake_minimum_required(VERSION 3.5)
project(intercom)

# The version number.
set (intercom_VERSION_MAJOR 0)
set (intercom_VERSION_MINOR 1)
set (intercom_VERSION_PATCH 1)

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define build directory.
set(CMAKE_BINARY_DIR ${CMAKE_SOURCE_DIR}/build)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Use the "Debug" as the default build type.
if( "${CMAKE_BUILD_TYPE}" STREQUAL "" )
    message( "Build type not specified. Defaulting to \"Debug\".\n Use \"cmake -DCMAKE_BUILD_TYPE=Release ..\" to build a release version." )
    set( CMAKE_BUILD_TYPE Debug )
endif()

# Compile the Rust crates
if( "${CMAKE_BUILD_TYPE}" STREQUAL "Release" )
    add_custom_target( intercom ALL cargo build --release
            WORKING_DIRECTORY ${CMAKE_SOURCE_DIR} )
else()
    add_custom_target( intercom ALL cargo build
            WORKING_DIRECTORY ${CMAKE_SOURCE_DIR} )
endif()

# Compile tests
add_subdirectory(test)
