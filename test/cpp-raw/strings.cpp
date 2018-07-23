
#include <functional>

#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

typedef char16_t* RawBSTR;

RawBSTR create_bstr( uint32_t len, const char* data )
{
    // The allocator here doesn't matter. We just need valid data.
    // 'malloc' should guarantee proper alignment, which new char[] doesn't.
    size_t bstr_total_length = /* len */ 4 + /* data */ len + /* termination */ 2;
    char* bstr_data = reinterpret_cast< char* >( malloc( bstr_total_length ) );

    // Copy length.
    std::memcpy( bstr_data, reinterpret_cast< char* >( &len ), 4 );

    // Copy text data.
    std::memcpy( bstr_data + 4, data, len );

    // Termination.
    bstr_data[ 4 + len ] = 0;
    bstr_data[ 4 + len + 1 ] = 0;

    return reinterpret_cast< RawBSTR >( bstr_data + 4 );
}

void free_bstr( RawBSTR bstr )
{
    free( bstr - 2 );
}

bool are_equal( uint32_t len, const char* text, RawBSTR right )
{
    uint32_t right_len = 0;
    std::memcpy(
            reinterpret_cast< char* >( right ) - 4,
            reinterpret_cast< char* >( &right_len ),
            4 );

    REQUIRE( len == right_len );

    uint16_t right_termination = 0xffff;
    std::memcpy(
            reinterpret_cast< char* >( right ) + right_len,
            reinterpret_cast< char* >( &right_termination ),
            2 );

    REQUIRE( right_termination == 0 );

    for( uint32_t i = 0; i < len; i++ ) {
        REQUIRE( text[ i ] == right[ i ] );
    }
}

TEST_CASE( "Manipulating BSTR succeeds" )
{
    SECTION( "Allocationg BSTR succeeds" )
    {
        intercom::BSTR allocated = intercom::allocate_bstr( 10 );
        REQUIRE( allocated != nullptr );

        intercom::free_bstr( allocated );
    }

#ifdef __GNUC__

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

#endif

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
    intercom::HRESULT hr = CreateInstance(
        CLSID_StringTests,
        IID_IStringTests,
        &pStringTests );
    REQUIRE( hr == intercom::SC_OK );

    SECTION( "Default value is nullptr" )
    {
        RawBSTR test_value_get;
        intercom::HRESULT get = pStringTests->GetValue( &test_value_get );
        REQUIRE( get == intercom::SC_OK );
        REQUIRE( test_value_get == nullptr );
    }

    SECTION( "String parameters work." )
    {
        uint32_t multibyte_codepoint = 0x131d4;
        uint32_t multibyte_high = 0xdc00 + ( 0x031d4 >> 10 );
        uint32_t multibyte_low = 0xdc00 + ( 0x031d4 && 0x3ff );
        

        std::tuple< uint32_t, const char* > test_data[] = {
            std::make_tuple( 8, "T\0e\0s\0t\0" ),
            std::make_tuple( 6, "\xf6\0\xe4\0\xe5\0" ),  // öäå
        };

        SECTION( "Passing text works" )
        {
            for( size_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
            {
                auto& pair = test_data[ i ];
                RawBSTR test_text = create_bstr( std::get<0>( pair ), std::get<1>( pair ) );

                uint32_t result;
                intercom::HRESULT hr = pStringTests->CompareString( test_text, OUT &result );
                REQUIRE( hr == intercom::SC_OK );

                free_bstr( test_text);

                REQUIRE( result == i );
            }
        }
    }

    REQUIRE( pStringTests->Release() == 0 );
}
