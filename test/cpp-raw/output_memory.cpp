
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"
#include <iostream>
using namespace std;

#include "testlib.hpp"

struct OutputTests : public IOutputMemoryTests_Automation
{
    int references = 0;
    int addRefs = 0;
    int releases = 0;

    virtual intercom::HRESULT INTERCOM_CC Succeed(
        IUnknown* input,
        OUT IUnknown** o1,
        OUT IUnknown** o2
    )
    {
        return intercom::EC_NOTIMPL;
    }

    virtual intercom::HRESULT INTERCOM_CC Fail(
        IUnknown* input,
        OUT IUnknown** o1,
        OUT void** o2,
        OUT IUnknown** o3
    )
    {
        return intercom::EC_NOTIMPL;
    }

    virtual intercom::HRESULT INTERCOM_CC CallSucceed(
        IOutputMemoryTests_Automation* itf,
        IUnknown* input
    )
    {
        return intercom::EC_NOTIMPL;
    }

    virtual intercom::HRESULT INTERCOM_CC CallFail(
        IOutputMemoryTests_Automation* itf,
        IUnknown* input
    )
    {
        return intercom::EC_NOTIMPL;
    }

    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { ++addRefs; return ++references; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { ++releases; return --references; }
};

TEST_CASE( "OutputMemory" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the IPrimitiveOperations interface.
    IOutputMemoryTests_Automation* pTests = nullptr;
    intercom::HRESULT hr = CreateInstance(
            CLSID_OutputMemoryTests,
            IID_IOutputMemoryTests_Automation,
            &pTests );

    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pTests != nullptr );

    SECTION( "Success" )
    {
        OutputTests inputObject;
        REQUIRE( inputObject.references == 0 );

        SECTION( "Rust to foreign" )
        {
            // Intentionally assigned garbage values. Existing values should be
            // ignored by callee.
            IUnknown* o1 = (IUnknown*)-1;
            IUnknown* o2 = (IUnknown*)1;
            REQUIRE( intercom::SC_OK == pTests->Succeed(&inputObject, OUT &o1, OUT &o2) );

            REQUIRE( inputObject.addRefs == 2 );
            REQUIRE( inputObject.releases == 0 );
            REQUIRE( inputObject.references == 2 );

            REQUIRE( o1 == &inputObject );
            REQUIRE( o2 == &inputObject );
            REQUIRE( o2->Release() == 1 );
            REQUIRE( o1->Release() == 0 );

            REQUIRE( inputObject.addRefs == 2 );
            REQUIRE( inputObject.releases == 2 );
            REQUIRE( inputObject.references == 0 );
        }
    }

    SECTION( "Fail" )
    {
        OutputTests inputObject;
        REQUIRE( inputObject.references == 0 );

        SECTION( "Rust to foreign" )
        {
            // Intentionally assigned garbage values. Existing values should be
            // ignored by callee.
            IUnknown* o1 = (IUnknown*)-1;
            void* o2 = (void*)0x08080808;
            IUnknown* o3 = (IUnknown*)1;
            REQUIRE( intercom::EC_FAIL == pTests->Fail(&inputObject, OUT &o1, OUT &o2, OUT &o3) );

            REQUIRE( inputObject.addRefs == 2 );
            REQUIRE( inputObject.releases == 2 );
            REQUIRE( inputObject.references == 0 );

            // We'll want to require these as well, but they aren't implemented currently.
            // REQUIRE( o1 == nullptr );
            // REQUIRE( o2 == nullptr );
            // REQUIRE( o3 == nullptr );
        }
    }

    REQUIRE( pTests->Release() == 0 );

    UninitializeRuntime();
}
