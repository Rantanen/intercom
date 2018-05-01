
// #ifdef __GNUC__

#include <functional>

#include "os.h"
#include "catch.hpp"

#include "testlib.h"
#include "utility/dummy_interface.h"

TEST_CASE( "Manipulating BSTR succeeds" )
{
    SECTION( "Allocationg BSTR succeeds" )
    {
        intercom::BSTR allocated = intercom::allocate_bstr( 10 );
        REQUIRE( allocated != nullptr );

        intercom::free_bstr( allocated );
    }

    SECTION( "Converting UTF-8 string to BSTR and back succeeds" )
    {
        // NOTE: 𓇔 is from "Supplementary Multilingual Plane"
        // and requires 4-bytes in UTF-16 representation.
        intercom::BSTR converted;
        intercom::utf8_to_bstr( u8"Test: 𓇔", &converted );
        REQUIRE( converted != nullptr );
        REQUIRE( intercom::get_characters_in_bstr( converted ) == 8 );

        char* utf8_back;
        intercom::bstr_to_utf8( converted, &utf8_back );
        REQUIRE( u8"Test: 𓇔" == std::string( utf8_back ) );

        std::free( utf8_back );
        intercom::free_bstr( converted );
    }

    SECTION( "Attempting to free null BSTR succeeds" )
    {
        intercom::free_bstr( nullptr );
    }
}

TEST_CASE( "Using BSTR in interface works" )
{
    // Initialize COM.
    InitializeRuntime();

    // Construct string storage.
    IStringTests* pStringTests = nullptr;
    HRESULT hr = CreateInstance(
        CLSID_StringTests,
        IID_IStringTests,
        &pStringTests );
    REQUIRE( hr == S_OK );

    SECTION( "Default value is nullptr" )
    {
        intercom::BSTR test_value_get;
        intercom::HRESULT get = pStringTests->GetValue( &test_value_get );
        REQUIRE( get == intercom::SC_OK );
        REQUIRE( test_value_get == nullptr );
    }

    SECTION( "Manipulating a value succeeds" )
    {
        intercom::BSTR test_value_put;
        intercom::utf8_to_bstr( u8"Test", &test_value_put );

        pStringTests->PutValue( test_value_put );
        intercom::free_bstr( test_value_put );

        SECTION( "Reading the value succeeds" )
        {
            intercom::BSTR test_value_get;
            intercom::HRESULT get = pStringTests->GetValue( &test_value_get );
            REQUIRE( get == intercom::SC_OK );

            char* test_value;
            intercom::bstr_to_utf8( test_value_get, &test_value );
            REQUIRE( u8"Test" == std::string( test_value ) );
            intercom::free_bstr( test_value_get );
            std::free( test_value );
        }

        SECTION( "Clearing the value with a nullptr succeeds" )
        {
            pStringTests->PutValue( nullptr );

            intercom::BSTR test_value_get;
            intercom::HRESULT get = pStringTests->GetValue( &test_value_get );
            REQUIRE( get == intercom::SC_OK );
            REQUIRE( test_value_get == nullptr );
        }
    }

    REQUIRE( pStringTests->Release() == 0 );
}

// #endif
