#include "catch.hpp"
#include "TestLib_h.h"

TEST_CASE( "Interfaces support error info" )
{
	// Initialize COM.
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

	// Get the error source interface.
	IErrorSource* pErrorSource = nullptr;
	HRESULT hr = CoCreateInstance(
		CLSID_ErrorSource,
		nullptr,
		CLSCTX_INPROC_SERVER,
		IID_IErrorSource,
		reinterpret_cast< void** >( &pErrorSource ) );
	REQUIRE( hr == S_OK );
	REQUIRE( pErrorSource != nullptr );

	SECTION( "Error source supports error info interface" )
	{
		ISupportErrorInfo* pSupportErrorInfo = nullptr;
		hr = pErrorSource->QueryInterface(
				IID_ISupportErrorInfo,
				reinterpret_cast< void** >( &pSupportErrorInfo ) );
		REQUIRE( hr == S_OK );
		REQUIRE( pSupportErrorInfo != nullptr );

		SECTION( "IErrorSource supports error info" )
		{
			hr = pSupportErrorInfo->InterfaceSupportsErrorInfo( IID_IErrorSource );
			REQUIRE( hr == S_OK );

			SECTION( "Errors set error info" )
			{
				BSTR bstrError = SysAllocString( L"Error message" );
				hr = pErrorSource->StoreError( E_FAIL, bstrError );

				REQUIRE( hr == E_FAIL );

				IErrorInfo* pErrorInfo = nullptr;
				hr = GetErrorInfo( 0, &pErrorInfo );
				REQUIRE( hr == S_OK );

				BSTR bstrOut = nullptr;
				hr = pErrorInfo->GetDescription( &bstrOut );
				REQUIRE( hr == 0 );
				REQUIRE( wcscmp( bstrOut, L"Error message" ) == 0 );
				SysFreeString( bstrOut );
			}
		}

		SECTION( "IUnknown does not support error info" )
		{
			hr = pSupportErrorInfo->InterfaceSupportsErrorInfo( IID_IUnknown );
			REQUIRE( hr == S_FALSE );
		}

		SECTION( "ISupportErrorInfo does not support error info" )
		{
			hr = pSupportErrorInfo->InterfaceSupportsErrorInfo( IID_ISupportErrorInfo );
			REQUIRE( hr == S_FALSE );
		}

		pSupportErrorInfo->Release();
	}

	pErrorSource->Release();

	CoUninitialize();
}
