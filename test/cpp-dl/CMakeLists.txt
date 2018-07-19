project (cpp-dl)

set (PROJECT_INCLUDE_DIR ${PROJECT_SOURCE_DIR})
set (PROJECT_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set (INTERCOM_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/../..)
set (MULTILIB_DIR ${INTERCOM_ROOT}/test/multilib)
set (TESTLIB_DIR ${INTERCOM_ROOT}/test/testlib)

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
# Use the intercom-cli as code generator for multi_lib.h/multi_lib.cpp
file(GLOB multilib_sources ${MULTILIB_DIR}/src)
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated/multi_lib.h ${PROJECT_SOURCE_DIR}/generated/multi_lib.cpp
    COMMAND cargo run cpp ${MULTILIB_DIR} ${PROJECT_SOURCE_DIR}/generated
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${multilib_sources})

# Use the intercom-cli as code generator for test_lib.h/test_lib.cpp
file(GLOB testlib_sources ${TESTLIB_DIR}/src)
add_custom_command(
    OUTPUT ${PROJECT_SOURCE_DIR}/generated/test_lib.h ${PROJECT_SOURCE_DIR}/generated/test_lib.cpp
    COMMAND cargo run cpp ${TESTLIB_DIR} ${PROJECT_SOURCE_DIR}/generated
    WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli
    DEPENDS ${testlib_sources})

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
