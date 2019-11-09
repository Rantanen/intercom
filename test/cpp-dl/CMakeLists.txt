project (cpp-dl)

set (PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR})
set (PROJECT_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set (INTERCOM_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../..)
set (TESTLIB_TARGET_DIR ${INTERCOM_ROOT}/test/target/${CMAKE_BUILD_TYPE})

if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    set (TESTLIB_TARGET_DIR ${INTERCOM_ROOT}/test/target/release)
else()
    set (TESTLIB_TARGET_DIR ${INTERCOM_ROOT}/test/target/debug)
endif()

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define source files.
set(PROJECT_SRCS
${PROJECT_SOURCE_DIR}/main.cpp
${PROJECT_SOURCE_DIR}/generated/multi_lib.cpp
${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
${PROJECT_SOURCE_DIR}/shared_functions.cpp
)

# Specify additional platform specific paths.
if( WIN32 )
    set (TESTLIB_LIB ${TESTLIB_TARGET_DIR}/test_lib.dll)
    set (MULTILIB_LIB ${TESTLIB_TARGET_DIR}/multi_lib.dll)
else()
    set (TESTLIB_LIB ${TESTLIB_TARGET_DIR}/libtest_lib.so)
    set (MULTILIB_LIB ${TESTLIB_TARGET_DIR}/libmulti_lib.so)
endif()

# Create the target dirs for the generated output.
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated
    COMMAND ${CMAKE_COMMAND} -E make_directory ${PROJECT_SOURCE_DIR}/generated)

# Use the intercom-cli as code generator for multi_lib.h/multi_lib.cpp
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated/multi_lib.hpp ${PROJECT_SOURCE_DIR}/generated/multi_lib.cpp
    COMMAND cargo run cpp ${MULTILIB_LIB} --header ${PROJECT_SOURCE_DIR}/generated/multi_lib.hpp --source ${PROJECT_SOURCE_DIR}/generated/multi_lib.cpp
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${MULTILIB_LIB} ${PROJECT_SOURCE_DIR}/generated)

# Use the intercom-cli as code generator for test_lib.h/test_lib.cpp
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated/test_lib.hpp ${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
    COMMAND cargo run cpp ${TESTLIB_LIB} --header ${PROJECT_SOURCE_DIR}/generated/test_lib.hpp --source ${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${TESTLIB_LIB} ${PROJECT_SOURCE_DIR}/generated)

include_directories("${PROJECT_BINARY_DIR}")
include_directories("${PROJECT_INCLUDE_DIR}")

# Ensure "intercom.h" is visible.
include_directories("${PROJECT_INCLUDE_DIR}/../../intercom-cpp")

# Compile the test executable.
add_executable(${PROJECT_NAME} ${PROJECT_SRCS} ${PROJECT_COMPILER_SPECIFIC_SRC})

# Link to static dependencies
target_link_libraries(${PROJECT_NAME} PRIVATE cpp-utility)

# Link to test libraries.
target_link_libraries(${PROJECT_NAME} PRIVATE test_lib PRIVATE multi_lib )
