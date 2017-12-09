
#include "os.h"
#include "catch.hpp"

TEST_CASE( "Methods accept and return COM objects" )
{
	// Initialize COM.
	InitializeRuntime();

	// Get the IPrimitiveOperations interface.
	IClassCreator* pOps = nullptr;
	HRESULT hr = CreateInstance(
			CLSID_ClassCreator,
			IID_IClassCreator,
			&pOps );

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

		pParent->Release();
	}

	pOps->Release();

	UninitializeRuntime();
}
