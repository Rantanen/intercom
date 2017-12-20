
#include <unordered_map>
#include <mutex>

#include "../os.h"
#include "TestLib_h.h"
#include "../../../intercom-cpp/src/cominterop.h"
#include "../../../intercom-cpp/src/activator.h"

using intercom::cpp::Activator;

// Storage for libraries.
namespace
{
	static std::mutex m_libraryLock;
	static std::unordered_map<std::string, dlopen_wrapper > g_libraries;
}



void InitializeRuntime()
{
}

void UninitializeRuntime()
{
}

HRESULT CreateInstance( REFCLSID clsid, REFIID iid, void** pout )
{
	Activator activate( clsid );
	activate.create( iid, pout );

	return S_OK;
}
