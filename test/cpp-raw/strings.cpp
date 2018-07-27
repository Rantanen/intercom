
#include <functional>
#include <cstdint>

#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

void check_equal( uint32_t len, const char16_t* text, intercom::BSTR right )
{
    if( len == 0 ) {
        REQUIRE( right == nullptr );
        return;
    }

    if( len != 0 )
        REQUIRE( right != nullptr );

    uint32_t right_len = 0;
    std::memcpy(
            reinterpret_cast< char* >( &right_len ),
            reinterpret_cast< char* >( right ) - 4,
            4 );

    REQUIRE( len * 2 == right_len );

    uint16_t right_termination = 0xffff;
    std::memcpy(
            reinterpret_cast< char* >( &right_termination ),
            reinterpret_cast< char* >( right ) + right_len,
            2 );

    REQUIRE( right_termination == 0 );

    for( uint32_t i = 0; i < len; i++ ) {
        REQUIRE( text[ i ] == right[ i ] );
    }
}

TEST_CASE( "Using BSTR in interface works" )
{
    // Initialize COM.
    InitializeRuntime();

    // Construct allocator.
    IAllocator* pAllocator = nullptr;
    intercom::HRESULT hr = CreateInstance(
        CLSID_Allocator,
        IID_IAllocator,
        &pAllocator );
    REQUIRE( hr == intercom::SC_OK );

    // Construct string test interface.
    IStringTests* pStringTests = nullptr;
    hr = CreateInstance(
        CLSID_StringTests,
        IID_IStringTests,
        &pStringTests );
    REQUIRE( hr == intercom::SC_OK );

    SECTION( "String parameters work." )
    {
        uint32_t multibyte_codepoint = 0x131d4;
        uint32_t multibyte_high = 0xdc00 + ( 0x031d4 >> 10 );
        uint32_t multibyte_low = 0xdc00 + ( 0x031d4 && 0x3ff );
        

        std::tuple< uint32_t, const char16_t*, const char* > test_data[] = {
            std::make_tuple( 0, nullptr, "<empty>" ),
            std::make_tuple( 4, u"Test", "\"Test\"" ),
            std::make_tuple( 3, u"\u00f6\u00e4\u00e5", "Multibyte UTF-8" ),  // Scandinavian letters: öäå
            std::make_tuple( 2, u"\U0001F980", "Multibyte UTF-16" ),  // Crab: 🦀
        };

        for( uint32_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
        {
            auto& pair = test_data[ i ];
            SECTION( std::string( "Passing BSTR: " ) + std::get<2>( pair ) )
            {
                intercom::BSTR test_text = pAllocator->AllocBstr(
                        const_cast< uint16_t* >(
                            reinterpret_cast< const uint16_t* >(
                                std::get<1>( pair ) ) ),
                        std::get<0>( pair ) );

                REQUIRE( test_text != nullptr );
                REQUIRE( *reinterpret_cast< uint32_t* >( test_text - 2 ) == std::get<0>( pair ) * 2 );

                uint32_t result;
                intercom::HRESULT hr = pStringTests->StringToIndex( test_text, OUT &result );
                REQUIRE( hr == intercom::SC_OK );

                pAllocator->FreeBstr( test_text );

                REQUIRE( result == i );
            }

            SECTION( std::string( "Receiving BSTR: " ) + std::get<2>( pair ) )
            {
                intercom::BSTR test_text = nullptr;
                intercom::HRESULT hr = pStringTests->IndexToString( i, OUT &test_text );
                REQUIRE( hr == intercom::SC_OK );

                check_equal( std::get<0>( pair ), std::get<1>( pair ), test_text );

                pAllocator->FreeBstr( test_text );
            }
        }

        SECTION( "BSTR into &intercom::BStr is not re-allocated" ) {

            intercom::BSTR test_text = pAllocator->AllocBstr(
                    const_cast< uint16_t* >(
                        reinterpret_cast< const uint16_t* >( u"Test string" ) ),
                    11 );

            intercom::HRESULT hr = pStringTests->BstrParameter(
                    test_text, reinterpret_cast< uintptr_t >( test_text ) );

            pAllocator->FreeBstr( test_text );

            REQUIRE( hr == S_OK );
        }

        SECTION( "BString return value is not re-allocated" ) {

            intercom::BSTR test_text = nullptr;
            uintptr_t test_ptr = 0;

            intercom::HRESULT hr = pStringTests->BstrReturnValue(
                    OUT &test_text,
                    OUT &test_ptr );
            REQUIRE( hr == S_OK );

            REQUIRE( test_text != nullptr );
            REQUIRE( reinterpret_cast< uintptr_t >( test_text ) == test_ptr );

            pAllocator->FreeBstr( test_text );
        }
    }

    SECTION( "Invalid UTF-16 results in E_INVALIDARG" )
    {
        // Low surrogate followed by high surrogate, ie. reversed order.
        uint16_t data[] = { 0xdfff, 0xd800 };
        intercom::BSTR test_text = pAllocator->AllocBstr( data, 2 );

        intercom::HRESULT hr = pStringTests->InvalidString( test_text );
        REQUIRE( hr == intercom::EC_INVALIDARG );

        pAllocator->FreeBstr( test_text );
    }

    REQUIRE( pStringTests->Release() == 0 );
}
