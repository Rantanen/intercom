project (test)

# Helper library for ensuring RUNPATH gets set for ELF binaries.
add_subdirectory(runpath)

# Collection of available test libraries
add_subdirectory(testlib)
add_subdirectory(cpp-utility)

# Collection of available tests
add_subdirectory(cpp-raw)
add_subdirectory(cpp-wrapper)
