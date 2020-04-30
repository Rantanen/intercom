
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "parameter_order" )
{
    // Initialize COM.
    InitializeRuntime();

    IParameterOrderTests_Automation* pTests = nullptr;
    intercom::HRESULT hr = CreateInstance(
            CLSID_ParameterOrderTests,
            IID_IParameterOrderTests_Automation,
            &pTests );
    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pTests != nullptr );

    SECTION( "normal_order" )
    {
        double result;
        pTests->Reciprocal(123, &result);
        REQUIRE( result == 1.0/123 );
    }
    SECTION( "reversed" )
    {
        double result;
        pTests->ReciprocalReversed(&result, 123);
        REQUIRE( result == 1.0/123 );
    }
    SECTION( "interleaved" )
    {
        double result1, result2;
        pTests->ReciprocalTwo(123, &result1, -100, &result2);
        REQUIRE( result1 == 1.0/123 );
        REQUIRE( result2 == -1.0/100 );
    }

    REQUIRE( pTests->Release() == 0 );

    UninitializeRuntime();
}

