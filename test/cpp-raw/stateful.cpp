
#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "Objects maintain their state" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the IResultOperations interface.
    IStatefulOperations* pOps = nullptr;
    intercom::HRESULT hr = CreateInstance(
            CLSID_StatefulOperations,
            IID_IStatefulOperations,
            &pOps );

    REQUIRE( hr == intercom::SC_OK );
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

    REQUIRE( pOps->Release() == 0 );

    UninitializeRuntime();
}
