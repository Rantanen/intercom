
#include <string>
using std::char_traits;

#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#define INTERCOM_FLATTEN_DECLARATIONS
#include "testlib.hpp"

#include <intercom.hpp>

namespace {
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
}

class RawImplementation : public ITypeSystemDifferingInterface_Raw
{
    virtual intercom::HRESULT INTERCOM_CC Func( char* string, char** output ) {

		if( strcmp( string, u8"\U0001F980" ) != 0 )
		{
			return intercom::EC_FAIL;
		}

		char* result = u8"\U0001F980";
		size_t len = strlen( result ) + 1;
		*output = reinterpret_cast<char*>( malloc( strlen( result ) + 1 ) );
		char_traits<char>::copy( *output, result, len );

		return intercom::SC_OK;
	}

    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return 1; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return 1; }
};

class AutomationImplementation : public ITypeSystemDifferingInterface_Automation
{
    virtual intercom::HRESULT INTERCOM_CC Func( intercom::BSTR string, intercom::BSTR* output ) {

		const char16_t* result = u"\U0001F980";
		size_t len = char_traits<char16_t>::length( result );
		uint32_t len32 = static_cast< uint32_t >( len );

		check_equal( len32, result, string );

		// Construct allocator.
		IAllocator_Automation* pAllocator = nullptr;
		intercom::HRESULT hr = CreateInstance(
			CLSID_Allocator,
			IID_IAllocator_Automation,
			&pAllocator );
		REQUIRE( hr == intercom::SC_OK );

		*output = pAllocator->AllocBstr(
				const_cast< uint16_t* >(
					reinterpret_cast< const uint16_t* >( result ) ),
				static_cast< uint32_t>(
					char_traits<char16_t>::length( result ) ) );

		return intercom::SC_OK;
	}

    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return 1; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return 1; }
};

TEST_CASE( "Callback invocations adapt to type systems." )
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

		hr = pTest->Test( &impl );
		REQUIRE( hr == intercom::SC_OK );

		pTest->Release();
	}

	/*
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

		hr = pTest->Test( &impl );
		REQUIRE( hr == intercom::SC_OK );

		pTest->Release();
	}
	*/

    UninitializeRuntime();
}

