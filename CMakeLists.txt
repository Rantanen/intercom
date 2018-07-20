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

# Enable warnings.
# -Wall is not enabled on Windows platform due to warnings from Windows SDK headers.
if(MSVC)
    SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -W3")
else()
    SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
endif()

# Define output directory.
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin/${CMAKE_SYSTEM_PROCESSOR})

if( WIN32 )
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY})
else()
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/lib)
    set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/lib)
endif()


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
