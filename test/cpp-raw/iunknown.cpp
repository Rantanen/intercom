
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "iunknown" )
{
    // Initialize COM.
    InitializeRuntime();

    SECTION( "create instance succeeds" )
    {

        IRefCountOperations_Automation* pRefCount = nullptr;
        intercom::HRESULT hr = CreateInstance(
                CLSID_RefCountOperations,
                IID_IRefCountOperations_Automation,
                &pRefCount );

        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( pRefCount != nullptr );

        SECTION( "create instance produces one reference." )
        {

            REQUIRE( pRefCount->GetRefCount() == 1 );
        }
        SECTION( "AddRef increments reference count" )
        {

            REQUIRE( pRefCount->AddRef() == 2 );
            REQUIRE( pRefCount->GetRefCount() == 2 );
            REQUIRE( pRefCount->AddRef() == 3 );
            REQUIRE( pRefCount->GetRefCount() == 3 );
            REQUIRE( pRefCount->AddRef() == 4 );
            REQUIRE( pRefCount->GetRefCount() == 4 );

            SECTION( "Release decrements reference count" )
            {
                REQUIRE( pRefCount->Release() == 3 );
                REQUIRE( pRefCount->GetRefCount() == 3 );
                REQUIRE( pRefCount->Release() == 2 );
                REQUIRE( pRefCount->GetRefCount() == 2 );
                REQUIRE( pRefCount->Release() == 1 );
                REQUIRE( pRefCount->GetRefCount() == 1 );
            }
        }
        SECTION( "QueryInterface produces a new interface" )
        {

            IUnknown* pUnknownCopy = nullptr;
            hr = pRefCount->QueryInterface(
                    IID_IUnknown,
                    reinterpret_cast< void** >( &pUnknownCopy ) );

            REQUIRE( hr == intercom::SC_OK );
            REQUIRE( pUnknownCopy != nullptr );

            SECTION( "reference count was incremented" )
            {
                REQUIRE( pUnknownCopy->Release() == 1 );
                REQUIRE( pRefCount->GetRefCount() == 1 );
            }
            SECTION( "reference count is shared between interfaces" )
            {
                REQUIRE( pUnknownCopy->AddRef() == 3 );
                REQUIRE( pRefCount->GetRefCount() == 3 );

                REQUIRE( pRefCount->AddRef() == 4 );

                REQUIRE( pUnknownCopy->AddRef() == 5 );
                REQUIRE( pRefCount->GetRefCount() == 5 );

                REQUIRE( pRefCount->AddRef() == 6 );

                REQUIRE( pUnknownCopy->Release() == 5 );
                REQUIRE( pRefCount->GetRefCount() == 5 );

                REQUIRE( pRefCount->Release() == 4 );

                REQUIRE( pUnknownCopy->Release() == 3 );
                REQUIRE( pRefCount->GetRefCount() == 3 );

                REQUIRE( pRefCount->Release() == 2 );

                REQUIRE( pUnknownCopy->Release() == 1 );
                REQUIRE( pRefCount->GetRefCount() == 1 );
            }
        }

        SECTION( "COM interface returned from function has proper ref count" )
        {
            IRefCountOperations_Automation* pAnotherRefCount = nullptr;
            REQUIRE( pRefCount->GetNew( OUT &pAnotherRefCount ) == intercom::SC_OK );

            REQUIRE( pAnotherRefCount->GetRefCount() == 1 );
            REQUIRE( pRefCount->GetRefCount() == 1 );

            SECTION( "Returned objects have their own ref count" )
            {
                pAnotherRefCount->AddRef();
                REQUIRE( pAnotherRefCount->GetRefCount() == 2 );
                REQUIRE( pRefCount->GetRefCount() == 1 );
                REQUIRE( pAnotherRefCount->Release() == 1 );
            }

            REQUIRE( pAnotherRefCount->Release() == 0 );
        }

        REQUIRE( pRefCount->Release() == 0 );
    }

    // Uninitialize COM.
    UninitializeRuntime();
}
