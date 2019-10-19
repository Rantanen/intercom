project (rust-libs)
include(ExternalProject)

set(LIB_NAMES "test_lib" "multi_lib" )

set(WORKSPACE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/..")
set(INTERCOM_ROOT "${WORKSPACE_ROOT}/..")

if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
    set(BUILD_DIR "${WORKSPACE_ROOT}/target/release")
    set(TARGET_DIR "${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/release")
    set(BUILD_ARGUMENTS "--release")
else()
    set(BUILD_DIR "${WORKSPACE_ROOT}/target/debug")
    set(TARGET_DIR "${CMAKE_LIBRARY_OUTPUT_DIRECTORY}/debug")
    set(BUILD_ARGUMENTS "")
endif()


# Turn the lib names into binary names

set(LIB_FILE_NAMES "")
set(LIB_CPP_PATHS "")
foreach(l ${LIB_NAMES})
    LIST(APPEND LIB_FILE_NAMES "${CMAKE_SHARED_LIBRARY_PREFIX}${l}${CMAKE_SHARED_LIBRARY_SUFFIX}")
    LIST(APPEND LIB_CPP_PATHS "${BUILD_DIR}/${f}.cpp")
endforeach()

set(LIB_BUILD_PATHS "")
set(LIB_TARGET_PATHS "")
foreach(f ${LIB_FILE_NAMES})
    LIST(APPEND LIB_BUILD_PATHS "${BUILD_DIR}/${f}")
    LIST(APPEND LIB_TARGET_PATHS "${TARGET_DIR}/${f}")
endforeach()

# Compile the binaries.
add_custom_target(rust-libs-build
    COMMAND cargo build ${BUILD_ARGUMENTS}
    WORKING_DIRECTORY ${WORKSPACE_ROOT}
)

foreach(p ${LIB_BUILD_PATHS})
    add_custom_command(
        TARGET rust-libs-build POST_BUILD
        COMMAND cargo run ${BUILD_ARGUMENTS} embed-typelib ${p}
        WORKING_DIRECTORY ${INTERCOM_ROOT}/intercom-cli)
endforeach()

add_custom_command(
    TARGET rust-libs-build POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
        ${LIB_BUILD_PATHS}
        ${TARGET_DIR}
    DEPENDS ${LIB_BUILD_PATHS}
)

if( WIN32 )

    # Create the .lib file paths.
    set(IMPORT_BUILD_PATHS "")
    set(IMPORT_TARGET_PATHS "")
    foreach(f ${LIB_FILE_NAMES})
        LIST(APPEND IMPORT_BUILD_PATHS "${BUILD_DIR}/${f}.lib")
        LIST(APPEND IMPORT_TARGET_PATHS "${TARGET_DIR}/${f}.lib")
    endforeach()

    # Copy the .libs to the output directory.
    add_custom_command(
        TARGET rust-libs-build POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
            ${IMPORT_BUILD_PATHS}
            ${TARGET_DIR}
        DEPENDS ${LIB_BUILD_PATHS} )
endif()


add_library(multi_lib SHARED IMPORTED GLOBAL)
add_dependencies(multi_lib rust-libs-build)
set_target_properties(multi_lib PROPERTIES IMPORTED_LOCATION "${TARGET_DIR}/${CMAKE_LSHARED_LIBRARY_PREFIX}multi_lib${CMAKE_SHARED_LIBRARY_SUFFIX}")
set_target_properties(multi_lib PROPERTIES IMPORTED_IMPLIB "${TARGET_DIR}/${CMAKE_LSHARED_LIBRARY_PREFIX}multi_lib${CMAKE_SHARED_LIBRARY_SUFFIX}.lib")

add_library(test_lib SHARED IMPORTED GLOBAL)
add_dependencies(test_lib rust-libs-build)
set_target_properties(test_lib PROPERTIES IMPORTED_LOCATION "${TARGET_DIR}/${CMAKE_LSHARED_LIBRARY_PREFIX}test_lib${CMAKE_SHARED_LIBRARY_SUFFIX}")
set_target_properties(test_lib PROPERTIES IMPORTED_IMPLIB "${TARGET_DIR}/${CMAKE_LSHARED_LIBRARY_PREFIX}test_lib${CMAKE_SHARED_LIBRARY_SUFFIX}.lib")
