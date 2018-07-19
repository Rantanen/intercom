
#include <unordered_map>
#include <mutex>

#include "catch.hpp"
#include "../os.h"
#include "../generated/test_lib.h"
#include "../../runpath/init.h"
#include "../../../intercom-cpp/src/cominterop.h"
#include "../../../intercom-cpp/src/activator.h"

using intercom::Activator;

#pragma comment(linker, "\"/manifestdependency:name='test_lib' type='win32' version='1.0.0.0'\"" )


void InitializeRuntime()
{
    CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

    // Ensure the library "runpath" won't get optimized away.
    init_runpath();

    REQUIRE( test_lib::Descriptor::is_available() );
}

void UninitializeRuntime()
{
    CoUninitialize();
}

