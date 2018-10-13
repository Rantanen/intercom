
#include <functional>
#include <cstdint>
#include <string>
using std::char_traits;

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
    IAllocator_Automation* pAllocator = nullptr;
    intercom::HRESULT hr = CreateInstance(
        CLSID_Allocator,
        IID_IAllocator_Automation,
        &pAllocator );
    REQUIRE( hr == intercom::SC_OK );

    // Construct string test interface.
    IStringTests_Automation* pStringTestsAutomation = nullptr;
    hr = CreateInstance(
        CLSID_StringTests,
        IID_IStringTests_Automation,
        &pStringTestsAutomation );
    REQUIRE( hr == intercom::SC_OK );

	IStringTests_Raw* pStringTestsRaw = nullptr;
	hr = pStringTestsAutomation->QueryInterface(
			IID_IStringTests_Raw,
			OUT reinterpret_cast< void** >( &pStringTestsRaw ) );
	REQUIRE( hr == intercom::SC_OK );

	const int DESC_IDX = 0;
	const int UTF16_IDX = 1;
	const int UTF8_IDX = 2;
	std::tuple< const char*, const char16_t*, const char* > test_data[] = {
		std::make_tuple(
				"<empty>",
				nullptr,
				u8""
		),
		std::make_tuple( 
				"\"Test\"",
				u"Test",
				u8"Test"
		),
		std::make_tuple( 
				"Multibyte UTF-8",  // Scandinavian letters: öäå
				u"\u00f6\u00e4\u00e5",
				u8"\u00f6\u00e4\u00e5"
		),
		std::make_tuple( 
				"Multibyte UTF-16",  // Crab: 🦀
				u"\U0001F980",
				u8"\U0001F980"
		)
	};

    SECTION( "String parameters work." )
    {
        for( uint32_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
        {
            auto& tuple = test_data[ i ];
			const char* desc = std::get< DESC_IDX >( tuple );

			const char16_t* utf16_text = std::get< UTF16_IDX >( tuple );
			const size_t utf16_len = utf16_text == nullptr ? 0 : char_traits<char16_t>::length( utf16_text );
			const uint32_t utf16_len32 = static_cast< uint32_t >( utf16_len );

            SECTION( std::string( "Passing BSTR: " ) + desc )
            {
                intercom::BSTR test_text = pAllocator->AllocBstr(
                        const_cast< uint16_t* >(
                            reinterpret_cast< const uint16_t* >(
								utf16_text ) ),
						utf16_len32 );

				// Check the allocation succeeded.
                REQUIRE( test_text != nullptr );
                REQUIRE( *reinterpret_cast< uint32_t* >( test_text - 2 ) == utf16_len32 * 2 );

                uint32_t result;
                intercom::HRESULT hr = pStringTestsAutomation->StringToIndex( test_text, OUT &result );
                REQUIRE( hr == intercom::SC_OK );

                pAllocator->FreeBstr( test_text );

                REQUIRE( result == i );
            }

            SECTION( std::string( "Receiving BSTR: " ) + desc )
            {
                intercom::BSTR test_text = nullptr;
                intercom::HRESULT hr = pStringTestsAutomation->IndexToString( i, OUT &test_text );
                REQUIRE( hr == intercom::SC_OK );

                check_equal( utf16_len32, utf16_text, test_text );

                pAllocator->FreeBstr( test_text );
            }

			const char* utf8_text = std::get< UTF8_IDX >( tuple );
			const size_t utf8_len = utf8_text == nullptr ? 0 : char_traits<char>::length( utf8_text );
			const uint32_t utf8_len32 = static_cast< uint32_t >( utf8_len );

            SECTION( std::string( "Passing C-string: " ) + desc )
            {
                uint32_t result;
                intercom::HRESULT hr = pStringTestsRaw->StringToIndex(
						const_cast< char* >( utf8_text ),
						OUT &result );
                REQUIRE( hr == intercom::SC_OK );

                REQUIRE( result == i );
            }

            SECTION( std::string( "Receiving C-string: " ) + desc )
            {
                char* test_text = nullptr;
                intercom::HRESULT hr = pStringTestsRaw->IndexToString( i, OUT &test_text );
                REQUIRE( hr == intercom::SC_OK );

				REQUIRE( strcmp( test_text, utf8_text ) == 0 );

                pAllocator->Free( test_text );
            }

        }

        SECTION( "BSTR into &intercom::BStr is not re-allocated" ) {

            intercom::BSTR test_text = pAllocator->AllocBstr(
                    const_cast< uint16_t* >(
                        reinterpret_cast< const uint16_t* >( u"Test string" ) ),
                    11 );

            intercom::HRESULT hr = pStringTestsAutomation->BstrParameter(
                    test_text, reinterpret_cast< uintptr_t >( test_text ) );

            pAllocator->FreeBstr( test_text );

            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "BString return value is not re-allocated" ) {

            intercom::BSTR test_text = nullptr;
            uintptr_t test_ptr = 0;

            intercom::HRESULT hr = pStringTestsAutomation->BstrReturnValue(
                    OUT &test_text,
                    OUT &test_ptr );
            REQUIRE( hr == intercom::SC_OK );

            REQUIRE( test_text != nullptr );
            REQUIRE( reinterpret_cast< uintptr_t >( test_text ) == test_ptr );

            pAllocator->FreeBstr( test_text );
        }

        SECTION( "char* into &intercom::CStr is not re-allocated" ) {

            char* test_text = u8"Test string";

            intercom::HRESULT hr = pStringTestsRaw->CstrParameter(
                    test_text, reinterpret_cast< uintptr_t >( test_text ) );

            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "BString return value is not re-allocated" ) {

            char* test_text = nullptr;
            uintptr_t test_ptr = 0;

            intercom::HRESULT hr = pStringTestsRaw->CstrReturnValue(
                    OUT &test_text,
                    OUT &test_ptr );
            REQUIRE( hr == intercom::SC_OK );

            REQUIRE( test_text != nullptr );
            REQUIRE( reinterpret_cast< uintptr_t >( test_text ) == test_ptr );

            pAllocator->Free( test_text );
        }
    }

    SECTION( "Invalid UTF-16 results in E_INVALIDARG" )
    {
        // Low surrogate followed by high surrogate, ie. reversed order.
        uint16_t data[] = { 0xdfff, 0xd800 };
        intercom::BSTR test_text = pAllocator->AllocBstr( data, 2 );

        intercom::HRESULT hr = pStringTestsAutomation->InvalidString( test_text );
        REQUIRE( hr == intercom::EC_INVALIDARG );

        pAllocator->FreeBstr( test_text );
    }

    REQUIRE( pAllocator->Release() == 0 );
    REQUIRE( pStringTestsRaw->Release() == 1 );
    REQUIRE( pStringTestsAutomation->Release() == 0 );
}
