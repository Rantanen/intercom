cmake_minimum_required(VERSION 3.8)
project (cpp-raw)

set (PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR})
set (PROJECT_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})

# Define build directory.
set(CMAKE_BINARY_DIR ${CMAKE_SOURCE_DIR}/build)

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define source files.
set(PROJECT1_SRCS
${PROJECT_SOURCE_DIR}/catch.hpp
${PROJECT_SOURCE_DIR}/error_info.cpp
${PROJECT_SOURCE_DIR}/interface_params.cpp
${PROJECT_SOURCE_DIR}/iunknown.cpp
${PROJECT_SOURCE_DIR}/main.cpp
${PROJECT_SOURCE_DIR}/primitive_tests.cpp
${PROJECT_SOURCE_DIR}/result.cpp
# ${PROJECT_SOURCE_DIR}/return_interfaces.cpp
${PROJECT_SOURCE_DIR}/stateful.cpp
)


# Specify additional compiler specific helpers.
set(PROJECT1_COMPILER_SPECIFIC_SRC)
if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")

set(PROJECT1_COMPILER_SPECIFIC_SRC
${PROJECT_SOURCE_DIR}/gcc/os.cpp
${PROJECT_SOURCE_DIR}/gcc/TestLib_i.cpp
)
endif()

include_directories("${PROJECT_BINARY_DIR}")
include_directories("${PROJECT_INCLUDE_DIR}")

add_executable(${PROJECT_NAME} ${PROJECT1_SRCS} ${PROJECT1_COMPILER_SPECIFIC_SRC})

# Link to testlib
find_library(TESTLIB_LOCATION libtest_lib.so PATHS "${PROJECT_SOURCE_DIR}/../testlib/target/debug/")
target_link_libraries(${PROJECT_NAME} ${TESTLIB_LOCATION})