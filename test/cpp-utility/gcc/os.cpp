
#include <unordered_map>
#include <mutex>

#include "../os.hpp"
#include "test_lib.hpp"
#include "../../../intercom-cpp/src/cominterop.hpp"
#include "../../../intercom-cpp/src/activator.hpp"

using intercom::Activator;


void InitializeRuntime()
{
}

void UninitializeRuntime()
{
}

intercom::HRESULT CreateInstance( intercom::REFCLSID clsid, intercom::REFIID iid, void** pout )
{
    Activator activate( test_lib::Descriptor::NAME, clsid );
    activate.create( iid, pout );

    return S_OK;
}
