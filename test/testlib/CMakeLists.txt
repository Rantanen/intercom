cmake_minimum_required(VERSION 3.8)
project (test_lib)
include(ExternalProject)

# Determine the parameters based on the build type.
if( "${CMAKE_BUILD_TYPE}" STREQUAL "Release" )
    set(TESTLIB_BINARY_BUILD_PATH "${CMAKE_CURRENT_SOURCE_DIR}/target/release/libtest_lib.so" )
    set(TESTLIB_BINARY_FINAL_PATH "${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/libtest_lib.so" )
    set(TESTLIB_BUILD_ARGUMENT_RELEASE "--release")
else()
    set(TESTLIB_BINARY_BUILD_PATH "${CMAKE_CURRENT_SOURCE_DIR}/target/debug/libtest_lib.so" )
    set(TESTLIB_BINARY_FINAL_PATH "${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/libtest_lib.so" )
    set(TESTLIB_BUILD_ARGUMENT_RELEASE "")
endif()

# Build instructions for the library.
add_custom_command(
        OUTPUT ${TESTLIB_BINARY_BUILD_PATH}
        COMMAND "cargo" ARGS "build" ${TESTLIB_BUILD_ARGUMENT_RELEASE}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} )
add_custom_command(
        OUTPUT ${TESTLIB_BINARY_FINAL_PATH}
        COMMAND ${CMAKE_COMMAND} -E copy
                ${TESTLIB_BINARY_BUILD_PATH}
                ${TESTLIB_BINARY_FINAL_PATH}
        DEPENDS ${TESTLIB_BINARY_BUILD_PATH} )
add_custom_target( testlib DEPENDS ${TESTLIB_BINARY_FINAL_PATH} )

# Publish as a library.
add_library(test_lib SHARED IMPORTED GLOBAL)
add_dependencies(test_lib testlib)
set_target_properties(test_lib PROPERTIES IMPORTED_LOCATION "${TESTLIB_BINARY_FINAL_PATH}" )
