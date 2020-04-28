
#include <functional>
#include <cstdint>
#include <string>
using std::char_traits;

#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

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

intercom::BSTR AllocBstr(
    IAllocator_Automation* pAllocator,
    const char16_t* str
)
{
    return pAllocator->AllocBstr(
            const_cast< uint16_t* >(
                reinterpret_cast< const uint16_t* >( str ) ),
			static_cast< uint32_t >(
                char_traits<char16_t>::length( str ) ) );
}


void check_equal( const char16_t* text, intercom::BSTR right )
{
    const size_t len_size_t = text == nullptr ? 0 : char_traits<char16_t>::length( text );
    const uint32_t len = static_cast< uint32_t >( len_size_t );

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

TEST_CASE( "strings" )
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

                check_equal( utf16_text, test_text );

                pAllocator->FreeBstr( test_text );
            }

			const char* utf8_text = std::get< UTF8_IDX >( tuple );
			const size_t utf8_len = utf8_text == nullptr ? 0 : char_traits<char>::length( utf8_text );

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

        intercom::BSTR test_bstr_input = AllocBstr( pAllocator, u"\U0001F600" );

        SECTION( "BSTR to BStr" ) {

            intercom::HRESULT hr = pStringTestsAutomation->BstrParameter(
                    test_bstr_input, reinterpret_cast< uintptr_t >( test_bstr_input ) );
            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "BSTR to BString" ) {

            intercom::HRESULT hr = pStringTestsAutomation->BstringParameter( test_bstr_input );
            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "BSTR to CStr" ) {

            intercom::HRESULT hr = pStringTestsAutomation->CstrParameter(
                    test_bstr_input, reinterpret_cast< uintptr_t >( test_bstr_input ) );

            // The text validation should succeed (ie. no E_FAIL), but
            // pointer validation won't.
            REQUIRE( hr == intercom::EC_POINTER );
        }

        SECTION( "BSTR to CString" ) {

            intercom::HRESULT hr = pStringTestsAutomation->CstringParameter( test_bstr_input );
            REQUIRE( hr == intercom::SC_OK );
        }

        pAllocator->FreeBstr( test_bstr_input );

        intercom::BSTR test_bstr_output = nullptr;
        uintptr_t test_ptr = 0;

        SECTION( "BString into BSTR return value" ) {

            intercom::HRESULT hr = pStringTestsAutomation->BstringReturnValue(
                    OUT &test_bstr_output,
                    OUT &test_ptr );
            REQUIRE( hr == intercom::SC_OK );

            check_equal( u"\U0001F600", test_bstr_output );
            REQUIRE( reinterpret_cast< uintptr_t >( test_bstr_output ) == test_ptr );
        }

        SECTION( "CString into BSTR return value" ) {

            intercom::HRESULT hr = pStringTestsAutomation->CstringReturnValue(
                    OUT &test_bstr_output,
                    OUT &test_ptr );
            REQUIRE( hr == intercom::SC_OK );

            check_equal( u"\U0001F600", test_bstr_output );

            // CString into BSTR gets reallocated so the pointer should differ here.
            REQUIRE( reinterpret_cast< uintptr_t >( test_bstr_output ) != test_ptr );
        }


        pAllocator->FreeBstr( test_bstr_output );

        // We are passing this to generated functions that do not define 'const'.
        char* test_cstr_input = const_cast< char* >( u8"\U0001F600" );

        SECTION( "char* to CStr" ) {

            intercom::HRESULT hr = pStringTestsRaw->CstrParameter(
                    test_cstr_input, reinterpret_cast< uintptr_t >( test_cstr_input ) );

            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "char* to CString" ) {

            intercom::HRESULT hr = pStringTestsRaw->CstringParameter( test_cstr_input );

            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "char* to BStr" ) {

            intercom::HRESULT hr = pStringTestsRaw->BstrParameter(
                    test_cstr_input, reinterpret_cast< uintptr_t >( test_cstr_input ) );

            // The text validation should succeed (ie. no E_FAIL), but
            // pointer validation won't.
            REQUIRE( hr == intercom::EC_POINTER );
        }

        SECTION( "char* to BString" ) {

            intercom::HRESULT hr = pStringTestsRaw->BstringParameter( test_cstr_input );

            REQUIRE( hr == intercom::SC_OK );
        }

        char* test_cstr_output = nullptr;

        SECTION( "CString into char* return value" ) {

            intercom::HRESULT hr = pStringTestsRaw->CstringReturnValue(
                    OUT &test_cstr_output,
                    OUT &test_ptr );
            REQUIRE( hr == intercom::SC_OK );

            REQUIRE( test_cstr_output != nullptr );
            REQUIRE( strcmp( test_cstr_output, u8"\U0001F600" ) == 0 );
            REQUIRE( reinterpret_cast< uintptr_t >( test_cstr_output ) == test_ptr );
        }

        SECTION( "BString into char* return value" ) {

            intercom::HRESULT hr = pStringTestsRaw->BstringReturnValue(
                    OUT &test_cstr_output,
                    OUT &test_ptr );
            REQUIRE( hr == intercom::SC_OK );

            REQUIRE( test_cstr_output != nullptr );
            REQUIRE( strcmp( test_cstr_output, u8"\U0001F600" ) == 0 );

            // BString into char* gets reallocated so the pointer should differ here.
            REQUIRE( reinterpret_cast< uintptr_t >( test_cstr_output ) != test_ptr );
        }

        pAllocator->Free( test_cstr_output );
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
