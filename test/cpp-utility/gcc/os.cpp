
#include <unordered_map>
#include <mutex>

#include "../os.h"
#include "test_lib.h"
#include "../../../intercom-cpp/src/cominterop.h"
#include "../../../intercom-cpp/src/activator.h"

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
