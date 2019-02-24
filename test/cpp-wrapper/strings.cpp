
#include <functional>
#include <string>

#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "Manipulating BSTR succeeds" )
{
    SECTION( "Allocationg BSTR succeeds" )
    {
        intercom::BSTR allocated = intercom::allocate_bstr( 10 );
        REQUIRE( allocated != nullptr );

        REQUIRE( *( reinterpret_cast< uint32_t* >( allocated ) - 1 ) == 10 * 2 );
        REQUIRE( *( allocated + 10 ) == 0 );

        intercom::free_bstr( allocated );
    }

    std::tuple< const char16_t*, const char*, const char* > test_data[] = {
        std::make_tuple( nullptr, nullptr, "<empty>" ),
        std::make_tuple( u"Test", u8"Test", "\"Test\"" ),
        std::make_tuple( u"\u00f6\u00e4\u00e5", u8"\u00f6\u00e4\u00e5", "Multibyte UTF-8" ),  // Scandinavian letters: öäå
        std::make_tuple( u"\U0001F980", u8"\U0001F980", "Multibyte UTF-16" ),  // Crab: 🦀
    };

    for( const auto& tuple : test_data )
    {
        const char16_t* utf16_text = nullptr;
        const char* utf8_text = nullptr;
        const char* description = nullptr;
        std::tie( utf16_text, utf8_text, description ) = tuple;

        size_t utf8_len = utf8_text == nullptr
            ? 0
            : strlen( utf8_text );
        size_t utf16_len = utf16_text == nullptr
            ? 0
            : std::char_traits<char16_t>::length( utf16_text );

        SECTION( std::string( "Converting UTF-8 to BSTR works: " ) + description )
        {
            intercom::BSTR converted;
            intercom::utf8_to_bstr( utf8_text, &converted );

            if( utf16_text != nullptr )
                REQUIRE( converted != nullptr );
            else
                REQUIRE( converted == nullptr );

            REQUIRE( intercom::get_characters_in_bstr( converted ) == utf16_len );

            REQUIRE( memcmp( converted, utf16_text, utf16_len * 2 ) == 0 );
        }

        SECTION( std::string( "Converting BSTR to UTF-8 works: " ) + description )
        {
            intercom::BSTR bstr = intercom::allocate_bstr( utf16_len );
            memcpy( bstr, utf16_text, utf16_len * 2 );

            char* converted = nullptr;
            intercom::bstr_to_utf8( bstr, OUT &converted );

            // UTF-8 strings are always non-null.
            REQUIRE( converted != nullptr );
            REQUIRE( strlen( converted ) == utf8_len );

            // Zero length UTF-8 string is still allocated.
            const char* expected = utf8_text;
            if( utf8_len == 0 )
                expected = "";

            REQUIRE( memcmp( converted, expected, utf8_len ) == 0 );
        }
    }

    SECTION( "Attempting to free null BSTR succeeds" )
    {
        intercom::free_bstr( nullptr );
    }
}
