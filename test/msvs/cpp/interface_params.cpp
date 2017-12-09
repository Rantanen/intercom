
#include "catch.hpp"
#include "TestLib_h.h"

class CppImplementation : public ISharedInterface
{
	unsigned int GetValue() { return 5; }

	// These two are not used.
	void SetValue( unsigned int v ) { }
	HRESULT DivideBy( ISharedInterface* divisor, OUT unsigned int* result ) { return E_NOTIMPL; }

	HRESULT QueryInterface( const IID& riid, void** out ) { return E_NOTIMPL; }
	ULONG AddRef() { return 1; }
	ULONG Release() { return 1; }
};

TEST_CASE( "Methods accept COM interfaces as parameters." )
{
	// Initialize COM.
	CoInitializeEx( nullptr, COINIT_APARTMENTTHREADED );

	// Get the first SharedImplementation object.
	ISharedInterface* pItf1 = nullptr;
	HRESULT hr = CoCreateInstance(
		CLSID_SharedImplementation,
		nullptr,
		CLSCTX_INPROC_SERVER,
		IID_ISharedInterface,
		reinterpret_cast< void** >( &pItf1 ) );
	REQUIRE( hr == S_OK );

	// Get the second SharedImplementation object.
	ISharedInterface* pItf2 = nullptr;
	hr = CoCreateInstance(
		CLSID_SharedImplementation,
		nullptr,
		CLSCTX_INPROC_SERVER,
		IID_ISharedInterface,
		reinterpret_cast< void** >( &pItf2 ) );
	REQUIRE( hr == S_OK );

	REQUIRE( pItf1 != nullptr );
	REQUIRE( pItf2 != nullptr );

	SECTION( "Rust CoClass can be used as a parameter." )
	{
		pItf1->SetValue( 10 );
		pItf2->SetValue( 2 );

		unsigned int value = 0;
		hr = pItf1->DivideBy( pItf2, OUT &value );
		REQUIRE( hr == S_OK );
		REQUIRE( value == 5 );
	}

	SECTION( "C++ implementation can be used as a parameter." )
	{
		pItf1->SetValue( 10 );
		CppImplementation cppImpl;

		unsigned int value = 0;
		hr = pItf1->DivideBy( &cppImpl, OUT &value );
		REQUIRE( hr == S_OK );
		REQUIRE( value == 2 );
	}

	pItf1->Release();
	pItf2->Release();

	CoUninitialize();
}

