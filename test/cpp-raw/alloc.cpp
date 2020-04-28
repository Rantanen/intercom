#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "alloc" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the error source interface.
    IAllocator_Automation* pAllocator = nullptr;
    HRESULT hr = CreateInstance(
        CLSID_Allocator,
        IID_IAllocator_Automation,
        &pAllocator );
    REQUIRE( hr == S_OK );
    REQUIRE( pAllocator != nullptr );

    IAllocTests_Automation* pAllocTests = nullptr;
    hr = CreateInstance(
        CLSID_AllocTests,
        IID_IAllocTests_Automation,
        &pAllocTests );
    REQUIRE( hr == S_OK );
    REQUIRE( pAllocTests != nullptr );

    SECTION( "Dealloc BSTR return value" )
    {
        BSTR bstr = nullptr;
        bstr = pAllocTests->GetBstr( 123 );

        REQUIRE( bstr != nullptr );
        REQUIRE( bstr[ 0 ] == L'1' );
        REQUIRE( bstr[ 1 ] == L'2' );
        REQUIRE( bstr[ 2 ] == L'3' );
        REQUIRE( bstr[ 3 ] == 0 );

        // Nothing to assert after this. :<
        pAllocator->FreeBstr( bstr );
    }

    SECTION( "Dealloc BSTR result" )
    {
        BSTR bstr = nullptr;
        hr = pAllocTests->GetBstrResult( 999, OUT &bstr );
        REQUIRE( hr == S_OK );

        REQUIRE( bstr != nullptr );
        REQUIRE( bstr[ 0 ] == L'9' );
        REQUIRE( bstr[ 1 ] == L'9' );
        REQUIRE( bstr[ 2 ] == L'9' );
        REQUIRE( bstr[ 3 ] == 0 );

        // Nothing to assert after this. :<
        pAllocator->FreeBstr( bstr );
    }

    pAllocator->Release();

    UninitializeRuntime();
}
