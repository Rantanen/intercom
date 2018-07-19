project (runpath)

# C++14 support
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Define source files.
set(PROJECT_SRCS
${PROJECT_SOURCE_DIR}/init.c
)

add_library(${PROJECT_NAME} SHARED ${PROJECT_SRCS})
