
#include "catch.hpp"
#include "TestLib_h.h"

TEST_CASE( "Basic IUnknown implementation works" )
{
	// Initialize COM.
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

	SECTION( "create instance succeeds" ) {

		IUnknown* pUnknown = nullptr;
		HRESULT hr = CoCreateInstance(
				CLSID_PrimitiveOperations,
				nullptr,
				CLSCTX_INPROC_SERVER,
				IID_IUnknown,
				reinterpret_cast<void**>( &pUnknown ));

		REQUIRE( hr == S_OK );
		REQUIRE( pUnknown != nullptr );

		SECTION( "create instance produces one reference." ) {

			REQUIRE( pUnknown->Release() == 0 );
		}
		SECTION( "AddRef increments reference count" ) {

			REQUIRE( pUnknown->AddRef() == 2 );
			REQUIRE( pUnknown->AddRef() == 3 );
			REQUIRE( pUnknown->AddRef() == 4 );

			SECTION( "Release decrements reference count" ) {
				REQUIRE( pUnknown->Release() == 3 );
				REQUIRE( pUnknown->Release() == 2 );
				REQUIRE( pUnknown->Release() == 1 );
				REQUIRE( pUnknown->Release() == 0 );
			}
		}
		SECTION( "QueryInterface produces a new interface" ) {

			IUnknown* pUnknownCopy = nullptr;
			hr = pUnknown->QueryInterface(
					IID_IPrimitiveOperations,
					reinterpret_cast< void** >( &pUnknownCopy ) );

			REQUIRE( hr == S_OK );
			REQUIRE( pUnknownCopy != nullptr );

			SECTION( "reference count was incremented" ) {
				REQUIRE( pUnknownCopy->Release() == 1 );
			}
			SECTION( "reference count is shared between interfaces" ) {
				REQUIRE( pUnknownCopy->AddRef() == 3 );
				REQUIRE( pUnknown->AddRef() == 4 );
				REQUIRE( pUnknownCopy->AddRef() == 5 );
				REQUIRE( pUnknown->AddRef() == 6 );
				REQUIRE( pUnknownCopy->Release() == 5 );
				REQUIRE( pUnknown->Release() == 4 );
				REQUIRE( pUnknownCopy->Release() == 3 );
				REQUIRE( pUnknown->Release() == 2 );
				REQUIRE( pUnknownCopy->Release() == 1 );
				REQUIRE( pUnknown->Release() == 0 );
			}
		}
	}

	// Uninitialize COM.
	CoUninitialize();
}
