
#include "../cpp-utility/os.h"
#include "../cpp-utility/catch.hpp"

#include "testlib.h"

TEST_CASE( "Methods accept and return COM objects" )
{
	// Initialize COM.
	InitializeRuntime();

	// Get the IPrimitiveOperations interface.
	IClassCreator* pOps = nullptr;
	intercom::HRESULT hr = CreateInstance(
			CLSID_ClassCreator,
			IID_IClassCreator,
			&pOps );

	REQUIRE( hr == intercom::SC_OK );
	REQUIRE( pOps != nullptr );

	SECTION( "Return new object" )
	{
		ICreatedClass* pParent = nullptr;
		hr = pOps->CreateRoot( 10, OUT &pParent );

		REQUIRE( hr == intercom::SC_OK );
		REQUIRE( pParent != nullptr );

		int32_t id;
		hr = pParent->GetId( OUT &id );

		REQUIRE( hr == intercom::SC_OK );
		REQUIRE( id == 10 );

		SECTION( "New objects have correct reference count" )
		{
			IRefCount* pRefCount = nullptr;
			hr = pParent->QueryInterface( IID_IRefCount, reinterpret_cast< void** >( &pRefCount ) );

			// We have two references now: pParent and pRefCount.
			REQUIRE( pRefCount->GetRefCount() == 2 );

			pRefCount->Release();
		}

		SECTION( "Objects can be used as parameters" )
		{
			ICreatedClass* pChild = nullptr;
			IParent* pParentItf = nullptr;
			hr = pParent->QueryInterface( IID_IParent, reinterpret_cast< void** >( &pParentItf ) );
			hr = pOps->CreateChild( 20, pParentItf, &pChild );

			REQUIRE( hr == intercom::SC_OK );
			REQUIRE( pChild != nullptr );

			hr = pChild->GetId( &id );
			REQUIRE( hr == intercom::SC_OK );
			REQUIRE( id == 20 );

			hr = pChild->GetParentId( &id );
			REQUIRE( hr == intercom::SC_OK );
			REQUIRE( id == 10 );

			SECTION( "Parameter reference count stays same." )
			{
				IRefCount* pRefCountParent = nullptr;
				hr = pParent->QueryInterface( IID_IRefCount, reinterpret_cast< void** >( &pRefCountParent ) );

				// Three references:
				// - pParent
				// - pParentItf
				// - pRefCountParent
				REQUIRE( pRefCountParent->GetRefCount() == 3 );

				pRefCountParent->Release();
			}

			pParentItf->Release();
			pChild->Release();
		}

		REQUIRE( pParent->Release() == 0 );
	}

	REQUIRE( pOps->Release() == 0 );

	UninitializeRuntime();
}
