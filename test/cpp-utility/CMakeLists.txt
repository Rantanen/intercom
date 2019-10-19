project (cpp-utility)

set (PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR})
set (PROJECT_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set (INTERCOM_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../..)
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
    ${PROJECT_SOURCE_DIR}/dummy_interface.cpp
    ${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
)

# Specify additional compiler specific helpers.
if( WIN32 )
    set(PROJECT_COMPILER_SPECIFIC_SRC
        ${PROJECT_SOURCE_DIR}/msvc/os.cpp
    )
    set (TESTLIB_LIB ${TESTLIB_TARGET_DIR}/test_lib.dll)
else()
    set(PROJECT_COMPILER_SPECIFIC_SRC
        ${PROJECT_SOURCE_DIR}/posix/os.cpp
    )
    set (TESTLIB_LIB ${TESTLIB_TARGET_DIR}/libtest_lib.so)
endif()

# Use the intercom-cli as code generator for test_lib.h/test_lib.cpp
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated
    COMMAND ${CMAKE_COMMAND} -E make_directory ${PROJECT_SOURCE_DIR}/generated)
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated/test_lib.hpp ${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
    COMMAND cargo run --release cpp ${TESTLIB_LIB} --all --header ${PROJECT_SOURCE_DIR}/generated/test_lib.hpp --source ${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${TESTLIB_LIB} ${PROJECT_SOURCE_DIR}/generated)

include_directories("${PROJECT_BINARY_DIR}")
include_directories("${PROJECT_INCLUDE_DIR}")

# Ensure "intercom.hpp" is visible.
include_directories("${PROJECT_INCLUDE_DIR}/../../intercom-cpp")

add_library(${PROJECT_NAME} STATIC ${PROJECT_SRCS} ${PROJECT_COMPILER_SPECIFIC_SRC})

# Testlib required to generate the test_lib.h and test_lib.cpp
add_dependencies(${PROJECT_NAME} testlib)

# Linking to "runpath" library ensures that RUNPATH is set for ELF executables.
# All the test libraries built with Rust are copied into this location as well.
# Forcing the RUNPATH with the help of a dummy library ensures we can
# dynamically load the libraries with their name alone.
target_link_libraries(${PROJECT_NAME} PUBLIC runpath)

# Compile specific linking.
if( NOT WIN32 )

# Link to dl, support for loading dynamic libraries.
find_library(DL_LOCATION dl)
target_link_libraries(${PROJECT_NAME} PUBLIC ${DL_LOCATION})

# Link to "pthread", otherwise std::call_once used by ClassFactory does not work.
# See the comments around "__gthread_active" in gthr-default.h
find_library(PTHREAD_LOCATION pthread)
target_link_libraries(${PROJECT_NAME} PUBLIC ${PTHREAD_LOCATION})

endif()
