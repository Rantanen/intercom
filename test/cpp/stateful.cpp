
#include "catch.hpp"
#include "TestLib_h.h"

TEST_CASE( "Objects maintain their state" )
{
	// Initialize COM.
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

	// Get the IResultOperations interface.
	IStatefulOperations* pOps = nullptr;
	HRESULT hr = CoCreateInstance(
			CLSID_StatefulOperations,
			nullptr,
			CLSCTX_INPROC_SERVER,
			IID_IStatefulOperations,
			reinterpret_cast< void** >( &pOps ) );

	REQUIRE( hr == S_OK );
	REQUIRE( pOps != nullptr );

	SECTION( "State is stored" )
	{
		pOps->PutValue( 10 );
		REQUIRE( pOps->GetValue() == 10 );
		pOps->PutValue( -100 );
		REQUIRE( pOps->GetValue() == -100 );
		pOps->PutValue( 55555 );
		REQUIRE( pOps->GetValue() == 55555 );
	}

	CoUninitialize();
}
