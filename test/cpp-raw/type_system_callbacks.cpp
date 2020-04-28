
#include <string>
using std::char_traits;

#include <iostream>
using namespace std;

#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#define INTERCOM_FLATTEN_DECLARATIONS
#include "testlib.hpp"

#include <intercom.hpp>

namespace {

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

const char16_t* poop_utf16 = u"\U0001F4A9";
const char* poop_utf8 = u8"\U0001F4A9";

intercom::BSTR AllocBstr(
    IAllocator_Automation* pAllocator,
    const char16_t* str
)
{
    size_t len = str == nullptr ? 0 : char_traits<char16_t>::length( str );
    return pAllocator->AllocBstr(
            const_cast< uint16_t* >(
                reinterpret_cast< const uint16_t* >( str ) ),
			static_cast< uint32_t >( len ) );
}

}

class RawImplementation : public IStringTests_Raw
{
    virtual intercom::HRESULT INTERCOM_CC StringToIndex(
        char* input,
        OUT uint32_t* output
    )
    {
        size_t input_len = strlen( input );

        for( uint32_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
        {
            auto& tuple = test_data[ i ];

			const char* utf8_text = std::get< UTF8_IDX >( tuple );
			const size_t utf8_len = utf8_text == nullptr ? 0 : char_traits<char>::length( utf8_text );

            if( input_len != utf8_len )
                continue;

            if( char_traits<char>::compare(
                    input,
                    utf8_text,
                    input_len ) == 0 )
            {
                *output = i;
                return intercom::SC_OK;
            }
        }

        return intercom::EC_FAIL;
    }

    virtual intercom::HRESULT INTERCOM_CC IndexToString(
        uint32_t input,
        OUT char** output
    )
    {
        // Construct allocator.
        IAllocator_Raw* pAllocator = nullptr;
        intercom::HRESULT hr = CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Raw,
            &pAllocator );
        if( hr != intercom::SC_OK )
            return hr;

        auto& tuple = test_data[ input ];
        const char* utf8_text = std::get< UTF8_IDX >( tuple );
        const size_t utf8_len = utf8_text == nullptr ? 0 : char_traits<char>::length( utf8_text );

        *output = reinterpret_cast< char* >( pAllocator->Alloc( utf8_len + 1 ) );
        memcpy( *output, utf8_text, utf8_len + 1 );

        pAllocator->Release();
        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC BstrParameter(
        char* input,
        size_t pointer
    )
    {
        size_t input_len = strlen( input );

        if( char_traits<char>::compare(
                input, poop_utf8, input_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        if( reinterpret_cast<size_t>(input) != pointer )
            return intercom::EC_POINTER;

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC BstringParameter(
        char* input
    )
    {
        size_t input_len = strlen( input );

        if( char_traits<char>::compare(
                input, poop_utf8, input_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC BstringReturnValue(
        OUT char** result,
        OUT size_t* result_ptr
    )
    {
        // Construct allocator.
        IAllocator_Raw* pAllocator = nullptr;
        intercom::HRESULT hr = CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Raw,
            &pAllocator );
        if( hr != intercom::SC_OK )
            return hr;

        const size_t utf8_len = char_traits<char>::length( poop_utf8 );

        *result = reinterpret_cast< char* >( pAllocator->Alloc( utf8_len + 1 ) );
        memcpy( *result, poop_utf8, utf8_len + 1 );
        *result_ptr = reinterpret_cast<size_t>( *result );

        pAllocator->Release();
        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC CstrParameter(
        char* input,
        size_t pointer
    )
    {
        size_t input_len = strlen( input );

        if( char_traits<char>::compare(
                input, poop_utf8, input_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        if( reinterpret_cast<size_t>(input) != pointer )
            return intercom::EC_POINTER;

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC CstringParameter(
        char* input
    )
    {
        size_t input_len = strlen( input );

        if( char_traits<char>::compare(
                input, poop_utf8, input_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC CstringReturnValue(
        OUT char** result,
        OUT size_t* result_ptr
    )
    {
        // Construct allocator.
        IAllocator_Raw* pAllocator = nullptr;
        intercom::HRESULT hr = CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Raw,
            &pAllocator );
        if( hr != intercom::SC_OK )
            return hr;

        const size_t utf8_len = char_traits<char>::length( poop_utf8 );

        *result = reinterpret_cast< char* >( pAllocator->Alloc( utf8_len + 1 ) );
        memcpy( *result, poop_utf8, utf8_len + 1 );
        *result_ptr = reinterpret_cast<size_t>( *result );

        pAllocator->Release();
        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC InvalidString( char* _ ) { return intercom::EC_NOTIMPL; }

    // IUnknown implementation.

    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return 1; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return 1; }
};

class AutomationImplementation : public IStringTests_Automation
{
    virtual intercom::HRESULT INTERCOM_CC StringToIndex(
        intercom::BSTR input,
        OUT uint32_t* output
    )
    {
        uint32_t input_len = 0;
        if( input != nullptr )
        {
            std::memcpy(
                    reinterpret_cast< char* >( &input_len ),
                    reinterpret_cast< char* >( input ) - 4,
                    4 );
        }

        for( uint32_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
        {
            auto& tuple = test_data[ i ];

			const char16_t* utf16_text = std::get< UTF16_IDX >( tuple );
			const size_t utf16_len = utf16_text == nullptr ? 0 : char_traits<char16_t>::length( utf16_text );
			const uint32_t utf16_len32 = static_cast< uint32_t >( utf16_len );

            if( input_len != utf16_len32 * 2 )
                continue;

            if( char_traits<char16_t>::compare(
                    reinterpret_cast<char16_t*>(input),
                    utf16_text,
                    utf16_len32 ) == 0 )
            {
                *output = i;
                return intercom::SC_OK;
            }
        }

        return intercom::EC_FAIL;
    }

    virtual intercom::HRESULT INTERCOM_CC IndexToString(
        uint32_t input,
        OUT intercom::BSTR* output
    )
    {
        // Construct allocator.
        IAllocator_Automation* pAllocator = nullptr;
        intercom::HRESULT hr = CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Automation,
            &pAllocator );
        if( hr != intercom::SC_OK )
            return hr;

        auto& tuple = test_data[ input ];
        *output = AllocBstr( pAllocator, std::get< UTF16_IDX >( tuple ) );

        pAllocator->Release();
        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC BstrParameter(
        intercom::BSTR input,
        size_t pointer
    )
    {
        uint32_t input_len = 0;
        if( input != nullptr )
        {
            std::memcpy(
                    reinterpret_cast< char* >( &input_len ),
                    reinterpret_cast< char* >( input ) - 4,
                    4 );
        }

        size_t expected_len = char_traits<char16_t>::length( poop_utf16 );
        if( expected_len * 2 != input_len )
            return intercom::EC_FAIL;

        if( char_traits<char16_t>::compare(
                reinterpret_cast<char16_t*>(input),
                poop_utf16,
                expected_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        if( reinterpret_cast<size_t>(input) != pointer )
            return intercom::EC_POINTER;

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC BstringParameter(
        intercom::BSTR input
    )
    {
        uint32_t input_len = 0;
        if( input != nullptr )
        {
            std::memcpy(
                    reinterpret_cast< char* >( &input_len ),
                    reinterpret_cast< char* >( input ) - 4,
                    4 );
        }

        size_t expected_len = char_traits<char16_t>::length( poop_utf16 );
        if( expected_len * 2 != input_len )
            return intercom::EC_FAIL;

        if( char_traits<char16_t>::compare(
                reinterpret_cast<char16_t*>(input),
                poop_utf16,
                expected_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC BstringReturnValue(
        OUT intercom::BSTR* result,
        OUT size_t* result_ptr
    )
    {
        // Construct allocator.
        IAllocator_Automation* pAllocator = nullptr;
        intercom::HRESULT hr = CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Automation,
            &pAllocator );
        if( hr != intercom::SC_OK )
            return hr;

        *result = AllocBstr( pAllocator, poop_utf16 );
        *result_ptr = reinterpret_cast<size_t>( *result );

        pAllocator->Release();
        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC CstrParameter(
        intercom::BSTR input,
        size_t pointer
    )
    {
        uint32_t input_len = 0;
        if( input != nullptr )
        {
            std::memcpy(
                    reinterpret_cast< char* >( &input_len ),
                    reinterpret_cast< char* >( input ) - 4,
                    4 );
        }

        size_t expected_len = char_traits<char16_t>::length( poop_utf16 );
        if( expected_len * 2 != input_len )
            return intercom::EC_FAIL;

        if( char_traits<char16_t>::compare(
                reinterpret_cast<char16_t*>(input),
                poop_utf16,
                expected_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        if( reinterpret_cast<size_t>(input) != pointer )
            return intercom::EC_POINTER;

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC CstringParameter(
        intercom::BSTR input
    )
    {
        uint32_t input_len = 0;
        if( input != nullptr )
        {
            std::memcpy(
                    reinterpret_cast< char* >( &input_len ),
                    reinterpret_cast< char* >( input ) - 4,
                    4 );
        }

        size_t expected_len = char_traits<char16_t>::length( poop_utf16 );
        if( expected_len * 2 != input_len )
            return intercom::EC_FAIL;

        if( char_traits<char16_t>::compare(
                reinterpret_cast<char16_t*>(input),
                poop_utf16,
                expected_len ) != 0 )
        {
            return intercom::EC_FAIL;
        }

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC CstringReturnValue(
        OUT intercom::BSTR* result,
        OUT size_t* result_ptr
    )
    {
        // Construct allocator.
        IAllocator_Automation* pAllocator = nullptr;
        intercom::HRESULT hr = CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Automation,
            &pAllocator );
        if( hr != intercom::SC_OK )
            return hr;

        *result = AllocBstr( pAllocator, poop_utf16 );
        *result_ptr = reinterpret_cast<size_t>( *result );

        pAllocator->Release();
        return intercom::SC_OK;
    }

    virtual intercom::HRESULT INTERCOM_CC InvalidString( intercom::BSTR _ ) { return intercom::EC_NOTIMPL; }

    // IUnknown implementation.

    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return 1; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return 1; }
};

TEST_CASE( "type_system_callbacks" )
{
    // Initialize COM.
    InitializeRuntime();

	SECTION( "Callbacks given through Automation type systems work." )
	{
		// Get the first SharedImplementation object.
		ITypeSystemCaller_Automation* pTest = nullptr;
		intercom::HRESULT hr = CreateInstance(
			CLSID_TypeSystemCaller,
			IID_ITypeSystemCaller_Automation,
			&pTest );
		REQUIRE( hr == intercom::SC_OK );

		AutomationImplementation impl;

        for( uint32_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
        {
            auto& tuple = test_data[ i ];
			const char* desc = std::get< DESC_IDX >( tuple );

            SECTION( std::string( "Passing BSTR: " ) + desc )
            {
                intercom::HRESULT hr = pTest->CallString( i, &impl );
                REQUIRE( hr == intercom::SC_OK );
            }

            SECTION( std::string( "Receiving BSTR: " ) + desc )
            {
                intercom::HRESULT hr = pTest->ReceiveString( i, &impl );
                REQUIRE( hr == intercom::SC_OK );
            }
        }

        SECTION( "Pass BSTR" )
        {
            REQUIRE( pTest->PassBstr( &impl ) == intercom::SC_OK );
        }
        SECTION( "Pass BString" )
        {
            REQUIRE( pTest->PassBstring( &impl ) == intercom::SC_OK );
        }
        SECTION( "Receive BString" )
        {
            REQUIRE( pTest->ReceiveBstring( &impl ) == intercom::SC_OK );
        }
        SECTION( "Pass CStr" )
        {
            REQUIRE( pTest->PassCstr( &impl ) == intercom::EC_POINTER );
        }
        SECTION( "Pass CString" )
        {
            REQUIRE( pTest->PassCstring( &impl ) == intercom::SC_OK );
        }
        SECTION( "Receive CString" )
        {
            REQUIRE( pTest->ReceiveCstring( &impl ) == intercom::EC_POINTER );
        }

		pTest->Release();
	}

	SECTION( "Callbacks given through Raw type systems work." )
	{
		// Get the first SharedImplementation object.
		ITypeSystemCaller_Raw* pTest = nullptr;
		intercom::HRESULT hr = CreateInstance(
			CLSID_TypeSystemCaller,
			IID_ITypeSystemCaller_Raw,
			&pTest );
		REQUIRE( hr == intercom::SC_OK );

		RawImplementation impl;

        for( uint32_t i = 0; i < sizeof( test_data ) / sizeof( *test_data ); i++ )
        {
            auto& tuple = test_data[ i ];
			const char* desc = std::get< DESC_IDX >( tuple );

            SECTION( std::string( "Passing char*: " ) + desc )
            {
                intercom::HRESULT hr = pTest->CallString( i, &impl );
                REQUIRE( hr == intercom::SC_OK );
            }

            SECTION( std::string( "Receiving char*: " ) + desc )
            {
                intercom::HRESULT hr = pTest->ReceiveString( i, &impl );
                REQUIRE( hr == intercom::SC_OK );
            }
        }

        SECTION( "Pass BSTR" )
        {
            REQUIRE( pTest->PassBstr( &impl ) == intercom::EC_POINTER );
        }
        SECTION( "Pass BString" )
        {
            REQUIRE( pTest->PassBstring( &impl ) == intercom::SC_OK );
        }
        SECTION( "Receive BString" )
        {
            REQUIRE( pTest->ReceiveBstring( &impl ) == intercom::EC_POINTER );
        }
        SECTION( "Pass CStr" )
        {
            REQUIRE( pTest->PassCstr( &impl ) == intercom::SC_OK );
        }
        SECTION( "Pass CString" )
        {
            REQUIRE( pTest->PassCstring( &impl ) == intercom::SC_OK );
        }
        SECTION( "Receive CString" )
        {
            REQUIRE( pTest->ReceiveCstring( &impl ) == intercom::SC_OK );
        }

		pTest->Release();
	}

    UninitializeRuntime();
}

