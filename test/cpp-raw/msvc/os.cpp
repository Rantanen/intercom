
#include "../os.h"

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
