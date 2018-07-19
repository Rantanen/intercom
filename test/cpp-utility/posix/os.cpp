
#include <unordered_map>
#include <mutex>

#include "catch.hpp"
#include "../os.h"
#include "../generated/test_lib.h"
#include "../../runpath/init.h"
#include "../../../intercom-cpp/src/cominterop.h"
#include "../../../intercom-cpp/src/activator.h"

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
