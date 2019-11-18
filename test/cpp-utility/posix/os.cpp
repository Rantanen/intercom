
#include <unordered_map>
#include <mutex>

#include "../../dependencies/catch.hpp"
#include "../os.hpp"
#include "../generated/test_lib.hpp"
#include "../../runpath/init.h"
#include "../../../intercom-cpp/src/cominterop.hpp"
#include "../../../intercom-cpp/src/activator.hpp"

using intercom::Activator;


void InitializeRuntime()
{
    // Ensure the library "runpath" won't get optimized away.
    init_runpath();

    REQUIRE( test_lib::Descriptor::is_available() );
}

void UninitializeRuntime()
{
}
