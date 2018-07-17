project (cpp-raw)

set (PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR})
set (PROJECT_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set (INTERCOM_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../..)
set (TESTLIB_DIR ${INTERCOM_ROOT}/test/testlib)

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define source files.
set(PROJECT_SRCS
${PROJECT_SOURCE_DIR}/error_info.cpp
${PROJECT_SOURCE_DIR}/interface_params.cpp
${PROJECT_SOURCE_DIR}/iunknown.cpp
${PROJECT_SOURCE_DIR}/main.cpp
${PROJECT_SOURCE_DIR}/primitive_tests.cpp
${PROJECT_SOURCE_DIR}/result.cpp
${PROJECT_SOURCE_DIR}/return_interfaces.cpp
${PROJECT_SOURCE_DIR}/stateful.cpp
${PROJECT_SOURCE_DIR}/strings.cpp
)

# Use the intercom-cli as code generator for test_lib.h/test_lib.cpp
file(GLOB testlib_sources ${TESTLIB_DIR}/src)
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/gcc/test_lib.h ${PROJECT_SOURCE_DIR}/gcc/test_lib.cpp
    COMMAND cargo run cpp ${TESTLIB_DIR} ${PROJECT_SOURCE_DIR}/gcc
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${testlib_sources})

include_directories("${PROJECT_BINARY_DIR}")
include_directories("${PROJECT_INCLUDE_DIR}")

# Ensure "intercom.h" is visible.
include_directories("${PROJECT_INCLUDE_DIR}/../../intercom-cpp")

add_executable(${PROJECT_NAME} ${PROJECT_SRCS})

# Linking to "runpath" library ensures that RUNPATH is set for ELF executables.
# All the test libraries built with Rust are copied into this location as well.
# Forcing the RUNPATH with the help of a dummy library ensures we can
# dynamically load the libraries with their name alone.
target_link_libraries(${PROJECT_NAME} runpath)

# Link to static dependencies
target_link_libraries(${PROJECT_NAME} cpp-utility)

# Link to test libraries.
add_dependencies(${PROJECT_NAME} test_lib)
target_link_libraries(${PROJECT_NAME} test_lib)

# Compile specific linking.
if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")

# Link to dl, support for loading dynamic libraries.
find_library(DL_LOCATION dl)
target_link_libraries(${PROJECT_NAME} ${DL_LOCATION})

# Link to "pthread", otherwise std::call_once used by ClassFactory does not work.
# See the comments around "__gthread_active" in gthr-default.h
find_library(PTHREAD_LOCATION pthread)
target_link_libraries(${PROJECT_NAME} ${PTHREAD_LOCATION})

endif()
