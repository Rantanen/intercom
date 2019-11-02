
#include <string>
using std::char_traits;

#include "../cpp-utility/os.hpp"
#include "../cpp-utility/catch.hpp"

#include "testlib.hpp"

namespace {

    intercom::BSTR AllocBstr(
        IAllocator_Automation* pAllocator,
        const char16_t* str
    )
    {
        return pAllocator->AllocBstr(
            const_cast<uint16_t*>(
                reinterpret_cast<const uint16_t*>(str)),
            static_cast<uint32_t>(
                char_traits<char16_t>::length(str)));
    }

    void check_equal(const char16_t* text, intercom::BSTR right)
    {
        const size_t len_size_t = text == nullptr ? 0 : char_traits<char16_t>::length(text);
        const uint32_t len = static_cast<uint32_t>(len_size_t);

        if (len == 0) {
            REQUIRE(right == nullptr);
            return;
        }

        if (len != 0)
            REQUIRE(right != nullptr);

        uint32_t right_len = 0;
        std::memcpy(
            reinterpret_cast<char*>(&right_len),
            reinterpret_cast<char*>(right) - 4,
            4);

        REQUIRE(len * 2 == right_len);

        uint16_t right_termination = 0xffff;
        std::memcpy(
            reinterpret_cast<char*>(&right_termination),
            reinterpret_cast<char*>(right) + right_len,
            2);

        REQUIRE(right_termination == 0);

        for (uint32_t i = 0; i < len; i++) {
            REQUIRE(text[i] == right[i]);
        }
    }

}

TEST_CASE( "Struct" ) {

    // Initialize COM.
    InitializeRuntime();

    IStructParameterTests_Automation* pTests = nullptr;
    intercom::HRESULT hr = CreateInstance(
        CLSID_StructParameterTests,
        IID_IStructParameterTests_Automation,
        &pTests );
    REQUIRE( hr == intercom::SC_OK );

    SECTION( "Output" )
    {
        SECTION( "Basic struct" )
        {
            BasicStruct_Automation bsa;
            REQUIRE( pTests->GetBasicStruct(1, 2, 3, &bsa) == intercom::SC_OK );
            REQUIRE( bsa.a == 1 );
            REQUIRE( bsa.b == 2 );
            REQUIRE( bsa.c == 3 );
        }

        SECTION( "BString struct" )
        {
            // Construct allocator.
            IAllocator_Automation* pAllocator = nullptr;
            hr = CreateInstance(
                CLSID_Allocator,
                IID_IAllocator_Automation,
                &pAllocator );
            REQUIRE( hr == intercom::SC_OK );

            StringStruct_Automation ssa;
            BSTR a = AllocBstr(pAllocator, u"foo");
            BSTR b = AllocBstr(pAllocator, u"bar");

            REQUIRE( pTests->GetStringStruct(a, b, &ssa ) == intercom::SC_OK );

            check_equal(u"foo", ssa.a);
            check_equal(u"bar", ssa.b);

            pAllocator->FreeBstr( a );
            pAllocator->FreeBstr( b );
            pAllocator->FreeBstr( ssa.a );
            pAllocator->FreeBstr( ssa.b );
            pAllocator->Release();
        }

        SECTION( "CString struct" )
        {
            // Construct allocator.
            IAllocator_Raw* pAllocator = nullptr;
            hr = CreateInstance(
                CLSID_Allocator,
                IID_IAllocator_Raw,
                &pAllocator );
            REQUIRE( hr == intercom::SC_OK );

            IStructParameterTests_Raw* pTests_raw;
            REQUIRE(pTests->QueryInterface(IID_IStructParameterTests_Raw, reinterpret_cast<void**>( &pTests_raw ) ) == intercom::SC_OK);

            StringStruct_Raw ssa;
            REQUIRE( pTests_raw->GetStringStruct("foo", "bar", &ssa ) == intercom::SC_OK );

            REQUIRE( strcmp( ssa.a, "foo" ) == 0 );
            REQUIRE( strcmp( ssa.b, "bar" ) == 0 );

            pAllocator->Free( ssa.a );
            pAllocator->Free( ssa.b );
        }

        SECTION( "Complex struct" )
        {
            Rectangle_Automation rect;

            REQUIRE( pTests->GetComplexStruct(1.0, 100.0, 123.456, 789.123, &rect ) == intercom::SC_OK );

            REQUIRE(rect.top_left.x == 1.0);
            REQUIRE(rect.top_left.y == 100.0);
            REQUIRE(rect.bottom_right.x == 123.456);
            REQUIRE(rect.bottom_right.y == 789.123);
        }
    }

    SECTION( "Input" )
    {
        SECTION( "Basic struct" )
        {
            BasicStruct_Automation bsa;
            bsa.a = 123;
            bsa.b = 2345;
            bsa.c = 34567;
            REQUIRE( pTests->VerifyBasicStruct(bsa, 123, 2345, 34567) == intercom::SC_OK );
        }

        SECTION( "BString struct" )
        {
            // Construct allocator.
            IAllocator_Automation* pAllocator = nullptr;
            hr = CreateInstance(
                CLSID_Allocator,
                IID_IAllocator_Automation,
                &pAllocator );
            REQUIRE( hr == intercom::SC_OK );

            BSTR a = AllocBstr(pAllocator, u"foo");
            BSTR b = AllocBstr(pAllocator, u"bar");

            StringStruct_Automation ssa;
            ssa.a = a;
            ssa.b = b;

            REQUIRE( pTests->VerifyStringStruct(ssa, a, b) == intercom::SC_OK );

            pAllocator->FreeBstr( a );
            pAllocator->FreeBstr( b );
            pAllocator->Release();
        }

        SECTION( "CString struct" )
        {
            // Construct allocator.
            IAllocator_Raw* pAllocator = nullptr;
            hr = CreateInstance(
                CLSID_Allocator,
                IID_IAllocator_Raw,
                &pAllocator );
            REQUIRE( hr == intercom::SC_OK );

            IStructParameterTests_Raw* pTests_raw;
            REQUIRE(pTests->QueryInterface(IID_IStructParameterTests_Raw, reinterpret_cast<void**>( &pTests_raw ) ) == intercom::SC_OK);

            StringStruct_Raw ssa;
            ssa.a = "foo";
            ssa.b = "bar";

            REQUIRE( pTests_raw->VerifyStringStruct(ssa, "foo", "bar") == intercom::SC_OK );
        }

        SECTION( "Complex struct" )
        {
            Rectangle_Automation rect;
            rect.top_left.x = 1.0;
            rect.top_left.y = 100.0;
            rect.bottom_right.x = 123.456;
            rect.bottom_right.y = 789.123;

            REQUIRE( pTests->VerifyComplexStruct(rect, 1.0, 100.0, 123.456, 789.123) == intercom::SC_OK );
        }
    }
}
