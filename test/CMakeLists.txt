project (test)

# Helper library for ensuring RUNPATH gets set for ELF binaries.
add_subdirectory(runpath)

# Collection of available test libraries
add_subdirectory(testlib)

# Collection of available tests
add_subdirectory(cpp-raw)
