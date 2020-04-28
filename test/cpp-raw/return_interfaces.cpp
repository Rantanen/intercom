
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"
#include <iostream>
using namespace std;

#include "testlib.hpp"

TEST_CASE( "return_interfaces" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the IPrimitiveOperations interface.
    IClassCreator_Automation* pOps = nullptr;
    intercom::HRESULT hr = CreateInstance(
            CLSID_ClassCreator,
            IID_IClassCreator_Automation,
            &pOps );

    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pOps != nullptr );

    SECTION( "Return new object" )
    {
        ICreatedClass_Automation* pParent = nullptr;
        hr = pOps->CreateRoot( 10, OUT &pParent );

        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( pParent != nullptr );

        int32_t id;
        hr = pParent->GetId( OUT &id );

        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( id == 10 );

        SECTION( "New objects have correct reference count" )
        {
            IRefCount_Automation* pRefCount = nullptr;
            hr = pParent->QueryInterface( IID_IRefCount_Automation, reinterpret_cast< void** >( &pRefCount ) );

            // We have two references now: pParent and pRefCount.
            REQUIRE( pRefCount->GetRefCount() == 2 );

            pRefCount->Release();
        }

        SECTION( "Objects can be used as parameters" )
        {
            ICreatedClass_Automation* pChild = nullptr;
            IParent_Automation* pParentItf = nullptr;
            hr = pParent->QueryInterface( IID_IParent_Automation, reinterpret_cast< void** >( &pParentItf ) );
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
                IRefCount_Automation* pRefCountParent = nullptr;
                hr = pParent->QueryInterface( IID_IRefCount_Automation, reinterpret_cast< void** >( &pRefCountParent ) );

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

        SECTION( "Returned interface corresponds with the type system" )
        {
            SECTION( "Automation interface returns automation interface" )
            {
                // Create an object through the automation interface.
                ICreatedClass_Automation* pCreated = nullptr;
                hr = pOps->CreateRoot( 1, OUT &pCreated );
                REQUIRE( hr == intercom::SC_OK );

                // Query for the automation interface to be extra sure we get
                // an interface that represents that one.
                //
                // We are getting this into IUnknown pointer as that's the
                // only interface we need out of it in the end for release.
                IUnknown* pCreated_qi = nullptr;
                hr = pCreated->QueryInterface(
                        IID_ICreatedClass_Automation,
                        reinterpret_cast< void** >( &pCreated_qi ) );
                REQUIRE( hr == intercom::SC_OK );

                // Ensure the two interface pointers point to the same
                // interface.
                REQUIRE( pCreated == pCreated_qi );

                pCreated->Release();
                pCreated_qi->Release();
            }

            SECTION( "Raw interface returns raw interface" )
            {
                // Get a raw interface for the class creator.
                IClassCreator_Raw* pOpsRaw = nullptr;
                hr = pOps->QueryInterface(
                        IID_IClassCreator_Raw,
                        reinterpret_cast< void** >( &pOpsRaw ) );
                REQUIRE( hr == intercom::SC_OK );

                // Create an object through the automation interface.
                ICreatedClass_Raw* pCreated = nullptr;
                hr = pOpsRaw->CreateRoot( 1, OUT &pCreated );
                REQUIRE( hr == intercom::SC_OK );

                // Query for the automation interface to be extra sure we get
                // an interface that represents that one.
                //
                // We are getting this into IUnknown pointer as that's the
                // only interface we need out of it in the end for release.
                IUnknown* pCreated_qi = nullptr;
                hr = pCreated->QueryInterface(
                        IID_ICreatedClass_Raw,
                        reinterpret_cast< void** >( &pCreated_qi ) );
                REQUIRE( hr == intercom::SC_OK );

                // Ensure the two interface pointers point to the same
                // interface.
                REQUIRE( pCreated == pCreated_qi );

                pCreated->Release();
                pCreated_qi->Release();
                pOpsRaw->Release();
            }

            SECTION( "Sanity check the two interfaces are not the same" )
            {
                // Create an object through the automation interface.
                ICreatedClass_Automation* pCreated = nullptr;
                hr = pOps->CreateRoot( 1, OUT &pCreated );
                REQUIRE( hr == intercom::SC_OK );

                // Query for the automation interface to be extra sure we get
                // an interface that represents that one.
                //
                // We are getting this into IUnknown pointer as that's the
                // only interface we need out of it in the end for release.
                IUnknown* pCreated_qi = nullptr;
                hr = pCreated->QueryInterface(
                        IID_ICreatedClass_Raw,
                        reinterpret_cast< void** >( &pCreated_qi ) );
                REQUIRE( hr == intercom::SC_OK );

                // Ensure the two interface pointers point to the same
                // interface.
                REQUIRE( pCreated != pCreated_qi );

                pCreated->Release();
                pCreated_qi->Release();
            }
        }

        REQUIRE( pParent->Release() == 0 );
    }

    REQUIRE( pOps->Release() == 0 );

    UninitializeRuntime();
}
