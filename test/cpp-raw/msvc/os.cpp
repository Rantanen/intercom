
#include "../os.h"

#pragma comment(linker, "\"/manifestdependency:name='test_lib' type='win32' version='1.0.0.0'\"" )

void InitializeRuntime()
{
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );
}

void UninitializeRuntime()
{
	CoUninitialize();
}

HRESULT CreateInstance( REFCLSID clsid, REFIID iid, void** pout )
{
	return CoCreateInstance(
			clsid,
			nullptr,
			CLSCTX_INPROC_SERVER,
			iid,
			OUT pout );
}
