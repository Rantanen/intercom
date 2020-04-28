
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#define INTERCOM_FLATTEN_DECLARATIONS
#include "testlib.hpp"

#include <intercom.hpp>

class CppImplementation : public ISharedInterface_Automation
{
    virtual unsigned int INTERCOM_CC GetValue() { return 5; }

    // These two are not used.
    virtual void INTERCOM_CC SetValue( unsigned int v ) { }
    virtual intercom::HRESULT INTERCOM_CC DivideBy( ISharedInterface_Automation* divisor, OUT unsigned int* result )
    { return intercom::EC_NOTIMPL; }

    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return 1; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return 1; }
};

TEST_CASE( "interface_params" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the first SharedImplementation object.
    ISharedInterface_Automation* pItf1 = nullptr;
    intercom::HRESULT hr = CreateInstance(
        CLSID_SharedImplementation,
        IID_ISharedInterface_Automation,
        &pItf1 );
    REQUIRE( hr == intercom::SC_OK );

    // Get the second SharedImplementation object.
    ISharedInterface_Automation* pItf2 = nullptr;
    hr = CreateInstance(
        CLSID_SharedImplementation,
        IID_ISharedInterface_Automation,
        &pItf2 );
    REQUIRE( hr == intercom::SC_OK );

    REQUIRE( pItf1 != nullptr );
    REQUIRE( pItf2 != nullptr );

    SECTION( "Rust CoClass can be used as a parameter." )
    {
        pItf1->SetValue( 10 );
        pItf2->SetValue( 2 );

        unsigned int value = 0;
        hr = pItf1->DivideBy( pItf2, OUT &value );
        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( value == 5 );
    }

    SECTION( "C++ implementation can be used as a parameter." )
    {
        pItf1->SetValue( 10 );
        CppImplementation cppImpl;

        unsigned int value = 0;
        hr = pItf1->DivideBy( &cppImpl, OUT &value );
        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( value == 2 );
    }

    pItf1->Release();
    pItf2->Release();

    UninitializeRuntime();
}

