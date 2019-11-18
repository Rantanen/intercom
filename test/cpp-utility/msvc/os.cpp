
#include <unordered_map>
#include <mutex>
#include <Objbase.h>

#include "../../dependencies/catch.hpp"
#include "../os.hpp"
#include "../generated/test_lib.hpp"
#include "../../runpath/init.h"
#include "../../../intercom-cpp/src/cominterop.hpp"
#include "../../../intercom-cpp/src/activator.hpp"

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

