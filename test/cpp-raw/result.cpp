#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "result" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the IResultOperations interface.
    IResultOperations_Automation* pOps = nullptr;
    intercom::HRESULT hr = CreateInstance(
            CLSID_ResultOperations,
            IID_IResultOperations_Automation,
            &pOps );

    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pOps != nullptr );

    SECTION( "HRESULT is returned with no OUT parameters" )
    {
        REQUIRE( pOps->SOk() == intercom::SC_OK );
        REQUIRE( pOps->NotImpl() == intercom::EC_NOTIMPL );
    }

    SECTION( "Rust results are converted to [retval] and HRESULT" )
    {
        double out = -1;

        SECTION( "Success yields intercom::SC_OK and [retval]" )
        {
            // Success case. Returns intercom::SC_OK and value.
            REQUIRE( pOps->Sqrt( 16.0, OUT &out ) == intercom::SC_OK );
            REQUIRE( out == 4.0 );
        }

        SECTION( "Failure yields error value and resets [retval]" )
        {
            // Fail case. Returns error and sets value to 0.
            intercom::HRESULT error = pOps->Sqrt( -1.0, OUT &out );
            REQUIRE( error == intercom::EC_INVALIDARG );
            REQUIRE( out == 0 );
        }

        SECTION( "Tuples are converted to multiple OUT parameters" )
        {
            uint16_t left;
            uint16_t right;

            hr = pOps->Tuple( 0xABBA0CD0, OUT &left, OUT &right );

            REQUIRE( hr == intercom::SC_OK );
            REQUIRE( left == 0xABBA );
            REQUIRE( right == 0x0CD0 );
        }
    }

    REQUIRE( pOps->Release() == 0 );

    UninitializeRuntime();
}
