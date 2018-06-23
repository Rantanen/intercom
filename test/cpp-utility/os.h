
// Interface definitions.
#ifdef _MSC_VER
#include <Windows.h>
#else

// Include declarations on non-Windows platforms.
#include <intercom.h>
#include "../../intercom-cpp/src/msdef.h"

#endif

// Platform specific runtime initialization.
void InitializeRuntime();

// Platform specific runtime uninitialization.
void UninitializeRuntime();

// Create Intercom object instance.
intercom::HRESULT CreateInstance( const intercom::CLSID& clsid, const intercom::IID& iid, void** pout );

// Create Intercom object instance.
template <class TInterface>
intercom::HRESULT CreateInstance(
	intercom::REFCLSID clsid,
	intercom::REFIID iid, TInterface** pout )
{
	return CreateInstance( clsid, iid, reinterpret_cast< void** >( pout ) );
}
