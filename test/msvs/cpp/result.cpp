#include "catch.hpp"
#include "TestLib_h.h"

TEST_CASE( "Results can be returned" )
{
	// Initialize COM.
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

	// Get the IResultOperations interface.
	IResultOperations* pOps = nullptr;
	HRESULT hr = CoCreateInstance(
			CLSID_ResultOperations,
			nullptr,
			CLSCTX_INPROC_SERVER,
			IID_IResultOperations,
			reinterpret_cast< void** >( &pOps ) );

	REQUIRE( hr == S_OK );
	REQUIRE( pOps != nullptr );

	SECTION( "HRESULT is returned with no OUT parameters" )
	{
		REQUIRE( pOps->SOk() == S_OK );
		REQUIRE( pOps->NotImpl() == E_NOTIMPL );
	}

	SECTION( "Rust results are converted to [retval] and HRESULT" )
	{
		double out = -1;

		SECTION( "Success yields S_OK and [retval]" )
		{
			// Success case. Returns S_OK and value.
			REQUIRE( pOps->Sqrt( 16.0, OUT &out ) == S_OK );
			REQUIRE( out == 4.0 );
		}

		SECTION( "Failure yields error value and resets [retval]" )
		{
			// Fail case. Returns error and sets value to 0.
			REQUIRE( pOps->Sqrt( -1.0, OUT &out ) == E_INVALIDARG );
			REQUIRE( out == 0 );
		}
	}

	CoUninitialize();
}