
#define NOMINMAX
#include <limits>

#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "primitive_tests" )
{
    // Initialize COM.
    InitializeRuntime();

    // Get the IPrimitiveOperations_Automation interface.
    IPrimitiveOperations_Automation* pOps = nullptr;
    intercom::HRESULT hr = CreateInstance(
            CLSID_PrimitiveOperations,
            IID_IPrimitiveOperations_Automation,
            &pOps );

    REQUIRE( hr == intercom::SC_OK );
    REQUIRE( pOps != nullptr );

    #define PRIMITIVE_TEST( F, T, V ) REQUIRE( pOps->F( ((T)V) ) == ((T)~( ((T)V) + ((T)1) ) ))

    SECTION( "int8" )
    {
        auto min = std::numeric_limits<int8_t>::min();
        auto max = std::numeric_limits<int8_t>::max();
        PRIMITIVE_TEST( I8, int8_t, min );
        PRIMITIVE_TEST( I8, int8_t, max );
        PRIMITIVE_TEST( I8, int8_t, 1 );
        PRIMITIVE_TEST( I8, int8_t, 100 );
        PRIMITIVE_TEST( I8, int8_t, 123 );
        PRIMITIVE_TEST( I8, int8_t, -1 );
        PRIMITIVE_TEST( I8, int8_t, -100 );
        PRIMITIVE_TEST( I8, int8_t, -123 );
    }
    SECTION( "uint8" )
    {
        auto min = std::numeric_limits<uint8_t>::min();
        auto max = std::numeric_limits<uint8_t>::max();
        PRIMITIVE_TEST( U8, uint8_t, -min );
        PRIMITIVE_TEST( U8, uint8_t, -max );
        PRIMITIVE_TEST( U8, uint8_t, -1 );
        PRIMITIVE_TEST( U8, uint8_t, -100 );
        PRIMITIVE_TEST( U8, uint8_t, -123 );
    }
    SECTION( "int16" )
    {
        auto min = std::numeric_limits<int16_t>::min();
        auto max = std::numeric_limits<int16_t>::max();
        PRIMITIVE_TEST( I16, int16_t, min );
        PRIMITIVE_TEST( I16, int16_t, max );
        PRIMITIVE_TEST( I16, int16_t, 1 );
        PRIMITIVE_TEST( I16, int16_t, 100 );
        PRIMITIVE_TEST( I16, int16_t, 123 );
        PRIMITIVE_TEST( I16, int16_t, -1 );
        PRIMITIVE_TEST( I16, int16_t, -100 );
        PRIMITIVE_TEST( I16, int16_t, -123 );
    }
    SECTION( "uint16" )
    {
        auto min = std::numeric_limits<uint16_t>::min();
        auto max = std::numeric_limits<uint16_t>::max();
        PRIMITIVE_TEST( U16, uint16_t, min );
        PRIMITIVE_TEST( U16, uint16_t, max );
        PRIMITIVE_TEST( U16, uint16_t, 1 );
        PRIMITIVE_TEST( U16, uint16_t, 100 );
        PRIMITIVE_TEST( U16, uint16_t, 123 );
    }
    SECTION( "int32" )
    {
        auto min = std::numeric_limits<int32_t>::min();
        auto max = std::numeric_limits<int32_t>::max();
        PRIMITIVE_TEST( I32, int32_t, min );
        PRIMITIVE_TEST( I32, int32_t, max );
        PRIMITIVE_TEST( I32, int32_t, 1 );
        PRIMITIVE_TEST( I32, int32_t, 100 );
        PRIMITIVE_TEST( I32, int32_t, 123 );
        PRIMITIVE_TEST( I32, int32_t, -1 );
        PRIMITIVE_TEST( I32, int32_t, -100 );
        PRIMITIVE_TEST( I32, int32_t, -123 );
    }
    SECTION( "uint32" )
    {
        auto min = std::numeric_limits<uint32_t>::min();
        auto max = std::numeric_limits<uint32_t>::max();
        PRIMITIVE_TEST( U32, uint32_t, min );
        PRIMITIVE_TEST( U32, uint32_t, max );
        PRIMITIVE_TEST( U32, uint32_t, 1 );
        PRIMITIVE_TEST( U32, uint32_t, 100 );
        PRIMITIVE_TEST( U32, uint32_t, 123 );
    }
    SECTION( "int64" )
    {
        auto min = std::numeric_limits<int64_t>::min();
        auto max = std::numeric_limits<int64_t>::max();
        PRIMITIVE_TEST( I64, int64_t, min );
        PRIMITIVE_TEST( I64, int64_t, max );
        PRIMITIVE_TEST( I64, int64_t, 1 );
        PRIMITIVE_TEST( I64, int64_t, 100 );
        PRIMITIVE_TEST( I64, int64_t, 123 );
        PRIMITIVE_TEST( I64, int64_t, -1 );
        PRIMITIVE_TEST( I64, int64_t, -100 );
        PRIMITIVE_TEST( I64, int64_t, -123 );
    }
    SECTION( "uint64" )
    {
        auto min = std::numeric_limits<uint64_t>::min();
        auto max = std::numeric_limits<uint64_t>::max();
        PRIMITIVE_TEST( U64, uint64_t, min );
        PRIMITIVE_TEST( U64, uint64_t, max );
        PRIMITIVE_TEST( U64, uint64_t, 1 );
        PRIMITIVE_TEST( U64, uint64_t, 100 );
        PRIMITIVE_TEST( U64, uint64_t, 123 );
    }
    SECTION( "float" )
    {
        REQUIRE( pOps->F32( 1.0f ) == 1.0f );
        REQUIRE( pOps->F32( 2.0f ) == 0.5f );
        REQUIRE( pOps->F32( 12.34f ) == ( 1.0f / 12.34f ) );
        REQUIRE( pOps->F32( 0.1234f ) == ( 1.0f / 0.1234f ) );
        REQUIRE( pOps->F32( 3.0f ) == ( 1.0f / 3.0f ) );
    }
    SECTION( "float" )
    {
        REQUIRE( pOps->F64( 1.0 ) == 1.0 );
        REQUIRE( pOps->F64( 2.0 ) == 0.5 );
        REQUIRE( pOps->F64( 12.34 ) == ( 1.0 / 12.34 ) );
        REQUIRE( pOps->F64( 0.1234 ) == ( 1.0 / 0.1234 ) );
        REQUIRE( pOps->F64( 3.0 ) == ( 1.0 / 3.0 ) );
    }

    REQUIRE( pOps->Release() == 0 );

    UninitializeRuntime();
}

