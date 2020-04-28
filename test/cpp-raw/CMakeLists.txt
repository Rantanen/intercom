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
${PROJECT_SOURCE_DIR}/type_system_callbacks.cpp
${PROJECT_SOURCE_DIR}/variant.cpp
${PROJECT_SOURCE_DIR}/nullable_parameters.cpp
${PROJECT_SOURCE_DIR}/output_memory.cpp
)

include_directories("${PROJECT_BINARY_DIR}")
include_directories("${PROJECT_INCLUDE_DIR}")

# Ensure "intercom.hpp" is visible.
include_directories("${PROJECT_INCLUDE_DIR}/../../intercom-cpp")

add_executable(${PROJECT_NAME} ${PROJECT_SRCS})

# Link to static dependencies
target_link_libraries(${PROJECT_NAME} PRIVATE  cpp-utility)

# Link to test libraries.
target_link_libraries(${PROJECT_NAME} PRIVATE test_lib)
