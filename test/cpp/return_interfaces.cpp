
#include "catch.hpp"
#include "TestLib_h.h"

TEST_CASE( "Methods accept and return COM objects" )
{
	// Initialize COM.
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

	// Get the IPrimitiveOperations interface.
	IClassCreator* pOps = nullptr;
	HRESULT hr = CoCreateInstance(
		CLSID_ClassCreator,
		nullptr,
		CLSCTX_INPROC_SERVER,
		IID_IClassCreator,
		reinterpret_cast< void** >( &pOps ) );

	REQUIRE( hr == S_OK );
	REQUIRE( pOps != nullptr );

	SECTION( "Return new object" )
	{
		ICreatedClass* pParent = nullptr;
		hr = pOps->CreateRoot( 10, OUT &pParent );

		REQUIRE( hr == S_OK );
		REQUIRE( pParent != nullptr );

		int32_t id;
		hr = pParent->GetId( OUT &id );

		REQUIRE( hr == S_OK );
		REQUIRE( id == 10 );
	}

	CoUninitialize();
}
