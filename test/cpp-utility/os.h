#pragma once

#include <intercom.h>

// Interface definitions.
#ifdef _MSC_VER
#include <Windows.h>
#else

// Include declarations on non-Windows platforms.
#include "../../intercom-cpp/src/msdef.h"

#endif

// Platform specific runtime initialization.
void InitializeRuntime();

// Platform specific runtime uninitialization.
void UninitializeRuntime();

// Create Intercom object instance.
template <class TInterface>
intercom::HRESULT CreateInstance(
	const intercom::CLSID& clsid,
	const intercom::IID& iid,
	TInterface** pout
)
{
    return intercom::create_instance( clsid, iid, reinterpret_cast< void** >( pout ) );
}
