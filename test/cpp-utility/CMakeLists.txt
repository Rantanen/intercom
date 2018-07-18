project (cpp-utility)

set (PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR})
set (PROJECT_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set (INTERCOM_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../..)
set (TESTLIB_DIR ${INTERCOM_ROOT}/test/testlib)

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define source files.
set(PROJECT_SRCS
${PROJECT_SOURCE_DIR}/dummy_interface.cpp
)

# Specify additional compiler specific helpers.
set(PROJECT_COMPILER_SPECIFIC_SRC)
if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")

set(PROJECT_COMPILER_SPECIFIC_SRC
    ${PROJECT_SOURCE_DIR}/gcc/os.cpp
    ${PROJECT_SOURCE_DIR}/gcc/test_lib.cpp
)
endif()

# Use the intercom-cli as code generator for test_lib.h/test_lib.cpp
file(GLOB testlib_sources ${TESTLIB_DIR}/src)
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/gcc/test_lib.h ${PROJECT_SOURCE_DIR}/gcc/test_lib.cpp
    COMMAND cargo run cpp ${TESTLIB_DIR} ${PROJECT_SOURCE_DIR}/gcc
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${testlib_sources})

include_directories("${PROJECT_BINARY_DIR}")
include_directories("${PROJECT_INCLUDE_DIR}")

# Ensure "intercom.hpp" is visible.
include_directories("${PROJECT_INCLUDE_DIR}/../../intercom-cpp")

add_library(${PROJECT_NAME} STATIC ${PROJECT_SRCS} ${PROJECT_COMPILER_SPECIFIC_SRC})

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
