
#include <cstdint>
#include <string>
using std::char_traits;

#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"
#include "testlib.hpp"
#include <iostream>
using namespace std;

namespace
{
	class ErrorSource :
            public IErrorSource_Automation,
            public ISupportErrorInfo
	{
	public:

		virtual intercom::HRESULT INTERCOM_CC ReturnComerror(
            intercom::HRESULT hr,
            intercom::BSTR bstr
		)
		{
            // Get the error store for storing the message.
            IErrorStore_Automation* pErrorStore = nullptr;
            intercom::HRESULT hr2 = CreateInstance(
                CLSID_ErrorStore,
                IID_IErrorStore_Automation,
                &pErrorStore );
            if( hr2 != intercom::SC_OK )
                return hr2;

            // Set the error message.
            hr2 = pErrorStore->SetErrorMessage( bstr );
            if( hr2 != intercom::SC_OK )
                return hr2;

            pErrorStore->Release();

            // Return the hresult.
            return hr;
		}

		virtual intercom::HRESULT INTERCOM_CC ReturnTesterror(
            intercom::HRESULT hr,
            intercom::BSTR bstr
		)
		{
            // Get the error store for storing the message.
            IErrorStore_Automation* pErrorStore = nullptr;
            intercom::HRESULT hr2 = CreateInstance(
                CLSID_ErrorStore,
                IID_IErrorStore_Automation,
                &pErrorStore );
            if( hr2 != intercom::SC_OK )
                return hr2;

            // Set the error message.
            hr2 = pErrorStore->SetErrorMessage( bstr );
            if( hr2 != intercom::SC_OK )
                return hr2;

            pErrorStore->Release();

            // Return the hresult.
            return hr;
		}

		virtual intercom::HRESULT INTERCOM_CC ReturnIoerror(
            intercom::HRESULT hr,
            intercom::BSTR bstr
		)
		{
            // Get the error store for storing the message.
            IErrorStore_Automation* pErrorStore = nullptr;
            intercom::HRESULT hr2 = CreateInstance(
                CLSID_ErrorStore,
                IID_IErrorStore_Automation,
                &pErrorStore );
            if( hr2 != intercom::SC_OK )
                return hr2;

            // Set the error message.
            hr2 = pErrorStore->SetErrorMessage( bstr );
            if( hr2 != intercom::SC_OK )
                return hr2;

            pErrorStore->Release();

            // Return the hresult.
            return hr;
		}

		virtual intercom::HRESULT INTERCOM_CC QueryInterface(
			const intercom::IID& riid,
			void** out
		)
		{
            if( riid == IID_ISupportErrorInfo ) {
                *out = static_cast< ISupportErrorInfo* >( this );
            } else if( riid == IID_IUnknown || riid == IID_IErrorSource_Automation ) {
                *out = this;
            } else {
                return intercom::EC_NOINTERFACE;
            }

            // Only increment 'qi on successful queries.
            // Intercom might query for the raw interface which we do not implement.
            qi += 1;
			return intercom::SC_OK;
		}

        virtual intercom::HRESULT INTERCOM_CC InterfaceSupportsErrorInfo(
            const intercom::IID& riid
        )
        {
            return intercom::SC_OK;
        }

		virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() {
            addRef += 1;
            return addRef - release;
        }

		virtual intercom::REF_COUNT_32 INTERCOM_CC Release() {
            release += 1;
            return addRef - release;
        }

		int qi = 0;
		int addRef = 0;
		int release = 0;
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
        IUnicodeConversion_Automation* pConverter = nullptr;
        REQUIRE( intercom::SC_OK == CreateInstance(
            CLSID_UnicodeConversion,
            IID_IUnicodeConversion_Automation,
            &pConverter ) );

        IAllocator_Automation* pAllocator = nullptr;
        REQUIRE( intercom::SC_OK == CreateInstance(
            CLSID_Allocator,
            IID_IAllocator_Automation,
            &pAllocator ) );

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

        if( len * 2 != right_len ) {
            char* utf8_expected = nullptr;
            REQUIRE( intercom::SC_OK == pConverter->Utf16ToUtf8(
                    const_cast< uint16_t* >(
                        reinterpret_cast< const uint16_t* >( text ) ),
                    OUT &utf8_expected ) );
            char* utf8_actual = nullptr;
            REQUIRE( intercom::SC_OK == pConverter->Utf16ToUtf8(
                    reinterpret_cast< uint16_t* >( right ), OUT &utf8_actual ) );

            std::string expected = utf8_expected;
            std::string actual = utf8_actual;

            pAllocator->Free( utf8_expected );
            pAllocator->Free( utf8_actual );

            REQUIRE( expected == actual );  // This should fail.
            REQUIRE( len * 2 == right_len );  // This failed above.
        }

        uint16_t right_termination = 0xffff;
        std::memcpy(
                reinterpret_cast< char* >( &right_termination ),
                reinterpret_cast< char* >( right ) + right_len,
                2 );

        REQUIRE( right_termination == 0 );

        for( uint32_t i = 0; i < len; i++ ) {
            if( text[ i ] != right[ i ] ) {
                char* utf8_expected = nullptr;
                REQUIRE( intercom::SC_OK == pConverter->Utf16ToUtf8(
                        const_cast< uint16_t* >(
                            reinterpret_cast< const uint16_t* >( text ) ),
                        OUT &utf8_expected ) );
                char* utf8_actual = nullptr;
                REQUIRE( intercom::SC_OK == pConverter->Utf16ToUtf8(
                        reinterpret_cast< uint16_t* >( right ), OUT &utf8_actual ) );
                cerr << utf8_actual << " != " << utf8_expected << endl;

                std::string expected = utf8_expected;
                std::string actual = utf8_actual;

                pAllocator->Free( utf8_expected );
                pAllocator->Free( utf8_actual );

                REQUIRE( expected == actual );  // This should fail.
                REQUIRE( text[ i ] == right[ i ] );  // This failed above.
            }
        }

        pAllocator->Release();
        pConverter->Release();
    }
}

TEST_CASE( "error_info" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the error source interface.
    IErrorTests_Automation* pErrorTests = nullptr;
    intercom::HRESULT hr = CreateInstance(
        CLSID_ErrorTests,
        IID_IErrorTests_Automation,
        &pErrorTests );
    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pErrorTests != nullptr );

    // Get the error source interface.
    IErrorSource_Automation* pErrorSource = nullptr;
    hr = pErrorTests->QueryInterface(
            IID_IErrorSource_Automation,
            OUT reinterpret_cast< void** >( &pErrorSource ) );
    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pErrorSource != nullptr );

    // Get the error store.
    IErrorStore_Automation* pErrorStore = nullptr;
    hr = CreateInstance(
        CLSID_ErrorStore,
        IID_IErrorStore_Automation,
        &pErrorStore );
    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pErrorTests != nullptr );

    // Construct allocator.
    IAllocator_Automation* pAllocator = nullptr;
    hr = CreateInstance(
        CLSID_Allocator,
        IID_IAllocator_Automation,
        &pAllocator );
    REQUIRE( hr == intercom::SC_OK );

    SECTION( "Error source supports error info interface" )
    {
        ISupportErrorInfo* pSupportErrorInfo = nullptr;
        hr = pErrorTests->QueryInterface(
                IID_ISupportErrorInfo,
                reinterpret_cast< void** >( &pSupportErrorInfo ) );
        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( pSupportErrorInfo != nullptr );

        SECTION( "IErrorTests supports error info" )
        {
            hr = pSupportErrorInfo->InterfaceSupportsErrorInfo( IID_IErrorTests_Automation );
            REQUIRE( hr == intercom::SC_OK );
        }

        SECTION( "IUnknown does not support error info" )
        {
            hr = pSupportErrorInfo->InterfaceSupportsErrorInfo( IID_IUnknown );
            REQUIRE( hr == intercom::SC_FALSE );
        }

        SECTION( "ISupportErrorInfo does not support error info" )
        {
            hr = pSupportErrorInfo->InterfaceSupportsErrorInfo( IID_ISupportErrorInfo );
            REQUIRE( hr == intercom::SC_FALSE );
        }

        pSupportErrorInfo->Release();
    }

    SECTION( "Errors set error info" )
    {
        SECTION( "Returning ComError" )
        {
            intercom::BSTR bstrError = AllocBstr( pAllocator, u"Error message" );
            hr = pErrorSource->ReturnComerror( 0x81234567, bstrError );

            REQUIRE( hr == 0x81234567 );

            IErrorInfo* pErrorInfo = nullptr;
            hr = pErrorStore->GetErrorInfo( &pErrorInfo );
            REQUIRE( hr == intercom::SC_OK );

            intercom::BSTR bstrOut = nullptr;
            hr = pErrorInfo->GetDescription( &bstrOut );
            check_equal( u"Error message", bstrOut );
            pAllocator->FreeBstr( bstrOut );
            pAllocator->FreeBstr( bstrError );
            pErrorInfo->Release();
        }

        SECTION( "Returning custom error" )
        {
            intercom::BSTR bstrError = AllocBstr( pAllocator, u"Error message" );
            hr = pErrorSource->ReturnTesterror( 0x81234567, bstrError );

            REQUIRE( hr == 0x81234567 );

            IErrorInfo* pErrorInfo = nullptr;
            hr = pErrorStore->GetErrorInfo( &pErrorInfo );
            REQUIRE( hr == intercom::SC_OK );

            intercom::BSTR bstrOut = nullptr;
            hr = pErrorInfo->GetDescription( &bstrOut );
            check_equal( u"Error message", bstrOut );
            pAllocator->FreeBstr( bstrOut );
            pAllocator->FreeBstr( bstrError );
            pErrorInfo->Release();
        }

        SECTION( "Returning std::io::Error" )
        {
            hr = pErrorSource->ReturnIoerror( 5, nullptr );

            REQUIRE( hr == 0x80070005 );

            IErrorInfo* pErrorInfo = nullptr;
            hr = pErrorStore->GetErrorInfo( &pErrorInfo );
            REQUIRE( hr == intercom::SC_OK );

            intercom::BSTR bstrOut = nullptr;
            hr = pErrorInfo->GetDescription( &bstrOut );
            check_equal( u"permission denied", bstrOut );
            pAllocator->FreeBstr( bstrOut );
            pErrorInfo->Release();
        }

        SECTION( "Returning ComError from COM callback" )
        {
            ErrorSource source;
            REQUIRE( intercom::SC_OK == pErrorTests->TestComerror( &source ) );
            REQUIRE( source.addRef == 0 );
            REQUIRE( source.qi == 1 );
            REQUIRE( source.release == 1 );
        }

        SECTION( "Returning custom error from COM callback" )
        {
            ErrorSource source;
            REQUIRE( intercom::SC_OK == pErrorTests->TestTesterror( &source ) );
            REQUIRE( source.addRef == 0 );
            REQUIRE( source.qi == 1 );
            REQUIRE( source.release == 1 );
        }

        SECTION( "Returning std::io::Error from COM callback" )
        {
            ErrorSource source;
            REQUIRE( intercom::SC_OK == pErrorTests->TestIoerror( &source ) );
            REQUIRE( source.addRef == 0 );
            REQUIRE( source.qi == 1 );
            REQUIRE( source.release == 1 );
        }
    }

    REQUIRE( pErrorTests->Release() == 1 );
    REQUIRE( pErrorSource->Release() == 0 );
    REQUIRE( pErrorStore->Release() == 0 );
    REQUIRE( pAllocator->Release() == 0 );

    UninitializeRuntime();
}
