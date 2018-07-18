
#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

#include <intercom.hpp>

class CppImplementation : public ISharedInterface
{
    unsigned int INTERCOM_CC GetValue() { return 5; }

    // These two are not used.
    void INTERCOM_CC SetValue( unsigned int v ) { }
    intercom::HRESULT INTERCOM_CC DivideBy( ISharedInterface* divisor, OUT unsigned int* result ) { return E_NOTIMPL; }

    HRESULT INTERCOM_CC QueryInterface( const IID& riid, void** out ) { return E_NOTIMPL; }
    ULONG INTERCOM_CC AddRef() { return 1; }
    ULONG INTERCOM_CC Release() { return 1; }
};

TEST_CASE( "Methods accept COM interfaces as parameters." )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the first SharedImplementation object.
    ISharedInterface* pItf1 = nullptr;
    HRESULT hr = CreateInstance(
        CLSID_SharedImplementation,
        IID_ISharedInterface,
        &pItf1 );
    REQUIRE( hr == S_OK );

    // Get the second SharedImplementation object.
    ISharedInterface* pItf2 = nullptr;
    hr = CreateInstance(
        CLSID_SharedImplementation,
        IID_ISharedInterface,
        &pItf2 );
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

    UninitializeRuntime();
}

