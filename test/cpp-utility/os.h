
// Interface definitions.
#ifdef _MSC_VER
#include <Windows.h>
#else

// Include declarations on non-Windows platforms.
#define INTERCOM_FLATTEN_DECLARATIONS
#include <intercom.h>
#include "../../intercom-cpp/src/msdef.h"

#endif

// Platform specific runtime initialization.
void InitializeRuntime();

// Platform specific runtime uninitialization.
void UninitializeRuntime();

// Create Intercom object instance.
HRESULT CreateInstance( const CLSID& clsid, const IID& iid, void** pout );

// Create Intercom object instance.
template <class TInterface>
HRESULT CreateInstance( REFCLSID clsid, REFIID iid, TInterface** pout )
{
    return CreateInstance( clsid, iid, reinterpret_cast< void** >( pout ) );
}
