
#include <functional>
#include <cstdint>
#include <string>
using std::char_traits;

#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

namespace
{
    intercom::VARIANT make_variant(
        intercom::VARENUM vtType,
        std::function<void(intercom::VARIANT&)> setter
    )
    {
        intercom::VARIANT v = { 0 };
        v.vt = vtType;
        setter( v );
        return v;
    }

    void check_equal( const char16_t* text, intercom::BSTR right )
    {
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

	class VariantImplementation : public IVariantInterface_Automation
	{
	public:

		virtual intercom::HRESULT INTERCOM_CC DoStuff(
			OUT intercom::VARIANT* pVariant
		)
		{
			// Construct allocator. This will be needed for BSTR tests.
			IAllocator_Automation* pAllocator = nullptr;
			intercom::HRESULT hr = CreateInstance(
				CLSID_Allocator,
				IID_IAllocator_Automation,
				&pAllocator );

			pVariant->vt = intercom::VT_BSTR;
			pVariant->bstrVal = pAllocator->AllocBstr(
					const_cast< uint16_t* >(
						reinterpret_cast< const uint16_t* >( u"text" ) ),
					4 );

			return intercom::SC_OK;
		}

		virtual intercom::HRESULT INTERCOM_CC QueryInterface(
			const intercom::IID& riid,
			void** out
		)
		{
			// We only have one interface implementation.
			// The tests shouldn't be querying bad IIDs.
			*out = this;
			return intercom::SC_OK;
		}

		virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return ++refCount; }
		virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return --refCount; }

		int refCount = 0;
	};
}

TEST_CASE( "variant" )
{
    // Initialize COM.
    InitializeRuntime();

    // Construct allocator. This will be needed for BSTR tests.
    IAllocator_Automation* pAllocator = nullptr;
    intercom::HRESULT hr = CreateInstance(
        CLSID_Allocator,
        IID_IAllocator_Automation,
        &pAllocator );
    REQUIRE( hr == intercom::SC_OK );

    // Construct string test interface.
    IVariantTests_Automation* pVariantTests = nullptr;
    hr = CreateInstance(
        CLSID_VariantTests,
        IID_IVariantTests_Automation,
        &pVariantTests );
    REQUIRE( hr == intercom::SC_OK );

    SECTION( "Variant as COM to Rust parameter" )
    {
        SECTION( "VT_EMPTY" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_EMPTY, { 0 } ) );
        }

        SECTION( "VT_NULL" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_EMPTY,  // Intercom handles NULL and intercom::VTEMPTY the same.
                    make_variant( intercom::VT_NULL,
                        [&]( auto& variant ) { } ) ) );
        }

        SECTION( "VT_I2" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_I2,
                    make_variant( intercom::VT_I2,
                        [&]( auto& variant ) { variant.iVal = -1; } ) ) );
        }

        SECTION( "VT_I4" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_I4,
                    make_variant( intercom::VT_I4,
                        [&]( auto& variant ) { variant.lVal = -1; } ) ) );
        }

        SECTION( "VT_R4" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_R4,
                    make_variant( intercom::VT_R4,
                        [&]( auto& variant ) { variant.fltVal = -1.234f; } ) ) );
        }

        SECTION( "VT_R8" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_R8,
                    make_variant( intercom::VT_R8,
                        [&]( auto& variant ) { variant.dblVal = -1.234; } ) ) );
        }

        SECTION( "VT_CY" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_CY,
                    make_variant( intercom::VT_CY,
                        [&]( auto& variant ) { variant.cyVal = {}; } ) ) );
        }

        SECTION( "VT_DATE" )
        {
            double time = ( ( 3.0*60.0 + 4.0 )*60.0 + 5.0 ) / ( 24.0 * 60.0 * 60.0 );
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_DATE * 100 + 1,
                    make_variant( intercom::VT_DATE,
                        [&]( auto& variant ) { variant.date = 0.0; } ) ) );
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_DATE * 100 + 2,
                    make_variant( intercom::VT_DATE,
                        [&]( auto& variant ) { variant.date = 36527 + time; } ) ) );
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_DATE * 100 + 3,
                    make_variant( intercom::VT_DATE,
                        [&]( auto& variant ) { variant.date = 36526; } ) ) );
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_DATE * 100 + 4,
                    make_variant( intercom::VT_DATE,
                        [&]( auto& variant ) { variant.date = -36521 - time; } ) ) );
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_DATE * 100 + 5,
                    make_variant( intercom::VT_DATE,
                        [&]( auto& variant ) { variant.date = -36522; } ) ) );
        }

        SECTION( "VT_BSTR" )
        {
            intercom::BSTR bstr = pAllocator->AllocBstr(
                    const_cast< uint16_t* >(
                        reinterpret_cast< const uint16_t* >( u"text" ) ),
                    4 );

            // We pass the VARIANT by value so the receiver should take the
            // ownership of the contained BSTR.
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_BSTR,
                    make_variant( intercom::VT_BSTR,
                        [&]( auto& variant ) { variant.bstrVal = bstr; } ) ) );
        }

        SECTION( "VT_BOOL" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_BOOL,
                    make_variant( intercom::VT_BOOL,
                        [&]( auto& variant ) { variant.boolVal = -1; } ) ) );
        }

        SECTION( "VT_I1" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_I1,
                    make_variant( intercom::VT_I1,
                        [&]( auto& variant ) { variant.cVal = -1; } ) ) );
        }

        SECTION( "VT_UI1" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_UI1,
                    make_variant( intercom::VT_UI1,
                        [&]( auto& variant ) { variant.bVal = 129; } ) ) );
        }

        SECTION( "VT_UI2" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_UI2,
                    make_variant( intercom::VT_UI2,
                        [&]( auto& variant ) { variant.uiVal = 12929; } ) ) );
        }

        SECTION( "VT_UI4" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_UI4,
                    make_variant( intercom::VT_UI4,
                        [&]( auto& variant ) { variant.ulVal = 1292929; } ) ) );
        }

        SECTION( "VT_I8" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_I8,
                    make_variant( intercom::VT_I8,
                        [&]( auto& variant ) { variant.llVal = -1; } ) ) );
        }

        SECTION( "VT_UI8" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
                    intercom::VT_UI8,
                    make_variant( intercom::VT_UI8,
                        [&]( auto& variant ) { variant.ullVal = 129292929; } ) ) );
        }
    }

    SECTION( "Bad variant conversion" )
    {
        SECTION( "VT_EMPTY" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_EMPTY,
                    make_variant( intercom::VT_I2,
                        [&]( auto& variant ) { variant.iVal = -1; } ) ) );
        }

        SECTION( "VT_I2" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_I2, { 0 } ) );
        }

        SECTION( "VT_I4" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_I4, { 0 } ) );
        }

        SECTION( "VT_R4" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_R4, { 0 } ) );
        }

        SECTION( "VT_R8" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_R8, { 0 } ) );
        }

        SECTION( "VT_DATE" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_DATE, { 0 } ) );
        }

        SECTION( "VT_BSTR" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_BSTR, { 0 } ) );
        }

        SECTION( "VT_BOOL" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_BOOL, { 0 } ) );
        }

        SECTION( "VT_I1" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_I1, { 0 } ) );
        }

        SECTION( "VT_UI1" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_UI1, { 0 } ) );
        }

        SECTION( "VT_UI2" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_UI2, { 0 } ) );
        }

        SECTION( "VT_UI4" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_UI4, { 0 } ) );
        }

        SECTION( "VT_I8" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_I8, { 0 } ) );
        }

        SECTION( "VT_UI8" )
        {
            REQUIRE( intercom::SC_OK == pVariantTests->BadVariantParameter(
                    intercom::VT_UI8, { 0 } ) );
        }
    }

    SECTION( "Variant from Rust to COM return value" )
    {
        // Shorter alias.
        IVariantTests_Automation* p = pVariantTests;

        SECTION( "VT_EMPTY" )
        {
            intercom::VARENUM vt = intercom::VT_EMPTY;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
        }

        SECTION( "VT_I2" )
        {
            intercom::VARENUM vt = intercom::VT_I2;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.iVal == -1 );
        }

        SECTION( "VT_I4" )
        {
            intercom::VARENUM vt = intercom::VT_I4;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.lVal == -1 );
        }

        SECTION( "VT_R4" )
        {
            intercom::VARENUM vt = intercom::VT_R4;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.fltVal == -1.234f );
        }

        SECTION( "VT_R8" )
        {
            intercom::VARENUM vt = intercom::VT_R8;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.dblVal == -1.234 );
        }

        SECTION( "VT_DATE" )
        {
            intercom::VARENUM vt = intercom::VT_DATE;
            intercom::VARIANT v = {};

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 1, OUT &v ) );
            REQUIRE( v.vt == vt );
            REQUIRE( v.date == 0.0 );

            double time = ( ( 3.0*60.0 + 4.0 )*60.0 + 5.0 ) / ( 24.0 * 60.0 * 60.0 );

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 2, OUT &v ) );
            REQUIRE( v.vt == vt );
            REQUIRE( v.date == 36527 + time );

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 3, OUT &v ) );
            REQUIRE( v.vt == vt );
            REQUIRE( v.date == 36526 );

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 4, OUT &v ) );
            REQUIRE( v.vt == vt );
            REQUIRE( v.date == -36521 - time );

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 5, OUT &v ) );
            REQUIRE( v.vt == vt );
            REQUIRE( v.date == -36522 );
        }

        SECTION( "VT_BSTR" )
        {
            intercom::VARENUM vt = intercom::VT_BSTR;
            intercom::VARIANT v = {};

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 1, OUT &v ) );
            REQUIRE( v.vt == vt );
            check_equal( u"text", v.bstrVal );
            pAllocator->FreeBstr( v.bstrVal );

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 2, OUT &v ) );
            REQUIRE( v.vt == vt );
            check_equal( u"text", v.bstrVal );
            pAllocator->FreeBstr( v.bstrVal );

            REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + 3, OUT &v ) );
            REQUIRE( v.vt == vt );
            check_equal( u"text", v.bstrVal );
            pAllocator->FreeBstr( v.bstrVal );
        }

        SECTION( "VT_BOOL" )
        {
            intercom::VARENUM vt = intercom::VT_BOOL;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.boolVal == -1 );
        }

        SECTION( "VT_I1" )
        {
            intercom::VARENUM vt = intercom::VT_I1;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.cVal == -1 );
        }

        SECTION( "VT_UI1" )
        {
            intercom::VARENUM vt = intercom::VT_UI1;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.bVal == 129 );
        }

        SECTION( "VT_UI2" )
        {
            intercom::VARENUM vt = intercom::VT_UI2;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.uiVal == 12929 );
        }

        SECTION( "VT_UI4" )
        {
            intercom::VARENUM vt = intercom::VT_UI4;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.ulVal == 1292929 );
        }

        SECTION( "VT_I8" )
        {
            intercom::VARENUM vt = intercom::VT_I8;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.llVal == -1 );
        }

        SECTION( "VT_UI8" )
        {
            intercom::VARENUM vt = intercom::VT_UI8;
            intercom::VARIANT v = {};
            REQUIRE( intercom::SC_OK == p->VariantResult( vt, OUT &v ) );

            REQUIRE( v.vt == vt );
            REQUIRE( v.ullVal == 129292929 );
        }

        SECTION( "VT_UNKNOWN" )
        {
			for( int i = 1; i <= 3; i++ )
			{
				SECTION( "Alternative " + std::to_string( i ) )
				{
					intercom::VARENUM vt = intercom::VT_UNKNOWN;
					intercom::VARIANT v = {};
					REQUIRE( intercom::SC_OK == p->VariantResult( vt * 100 + i, OUT &v ) );

					REQUIRE( v.vt == vt );

					IVariantInterface_Automation* pimpl = nullptr;
					REQUIRE( intercom::SC_OK == v.punkVal->QueryInterface(
							IID_IVariantInterface_Automation,
							OUT reinterpret_cast<void**>( &pimpl ) ) );

					intercom::VARIANT v2 = {};
					REQUIRE( intercom::SC_OK == pimpl->DoStuff( OUT &v2 ) );

					REQUIRE( v2.vt == intercom::VT_R8 );
					REQUIRE( v2.dblVal == 1.0 / 3.0 );

					REQUIRE( pimpl->Release() == 1 );
					REQUIRE( v.punkVal->Release() == 0 );
				}
			}
        }
    }

	SECTION( "IUnknown from COM to Rust" )
	{
		VariantImplementation vi;
		vi.AddRef();

		// We'll start with ref count of 1.
		REQUIRE( vi.refCount == 1 );

		REQUIRE( intercom::SC_OK == pVariantTests->VariantParameter(
				intercom::VT_UNKNOWN,
				make_variant( intercom::VT_UNKNOWN,
					[&]( auto& variant ) {
						variant.punkVal = &vi;
						variant.punkVal->AddRef();
					} ) ) );

		// Intercom should have released the VARIANT.
		REQUIRE( vi.refCount == 1 );
	}

    REQUIRE( pAllocator->Release() == 0 );
    REQUIRE( pVariantTests->Release() == 0 );
}
