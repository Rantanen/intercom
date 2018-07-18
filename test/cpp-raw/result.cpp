#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "Results can be returned" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the IResultOperations interface.
    IResultOperations* pOps = nullptr;
    HRESULT hr = CreateInstance(
            CLSID_ResultOperations,
            IID_IResultOperations,
            &pOps );

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
            HRESULT error = pOps->Sqrt( -1.0, OUT &out );
            REQUIRE( error == E_INVALIDARG );
            REQUIRE( out == 0 );
        }

        SECTION( "Tuples are converted to multiple OUT parameters" )
        {
            uint16_t left;
            uint16_t right;

            hr = pOps->Tuple( 0xABBA0CD0, OUT &left, OUT &right );

            REQUIRE( hr == S_OK );
            REQUIRE( left == 0xABBA );
            REQUIRE( right == 0x0CD0 );
        }
    }

    REQUIRE( pOps->Release() == 0 );

    UninitializeRuntime();
}