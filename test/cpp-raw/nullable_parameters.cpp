
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

class CoCallback : public ICallback_Automation {
public:
    CoCallback(uint32_t value) {
        this->value = value;
        this->rc = 1;
    }

    virtual uint32_t Callback() {
        return this->value;
    }

    uint32_t value;

    uint32_t rc;

    // Intercom shouldn't need these.
    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { rc++; return rc; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() {
        rc--;
        int localRc = rc;
        if( localRc == 0 )
            delete this;
        return localRc;
    }
};

class CoNullableInterface : public INullableInterface_Automation {
public:
    virtual uint32_t NullableParameter(ICallback_Automation* pCb) {
        if( pCb == nullptr )
        {
            return 0;
        }
        else
        {
            return pCb->Callback();
        }
    }

    virtual intercom::HRESULT NonnullParameter(
            ICallback_Automation* pCb,
            uint32_t* out
    ) {
        if( pCb == nullptr )
        {
            return intercom::EC_POINTER;
        }
        else
        {
            *out = pCb->Callback();
            return intercom::SC_OK;
        }
    }

    virtual intercom::HRESULT NullableOutput(uint32_t value, ICallback_Automation** pCb) {
        if( value == 0 )
        {
            *pCb = nullptr;
        }
        else
        {
            *pCb = new CoCallback(value);
        }

        return intercom::SC_OK;
    }

    virtual intercom::HRESULT NonnullOutput(
            uint32_t value,
            ICallback_Automation** pCb
    ) {
        if( value == 0 )
            *pCb = nullptr;
        else
            *pCb = new CoCallback(value);
        return intercom::SC_OK;
    }

    // Intercom shouldn't need these.
    virtual intercom::HRESULT INTERCOM_CC QueryInterface( const intercom::IID& riid, void** out ) { return intercom::EC_NOTIMPL; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC AddRef() { return 1; }
    virtual intercom::REF_COUNT_32 INTERCOM_CC Release() { return 1; }
};

TEST_CASE( "Nullable parameters" )
{
    // Initialize COM.
    InitializeRuntime();

    SECTION( "C++ to Rust" )
    {
        INullableInterface_Automation* pTests = nullptr;
        intercom::HRESULT hr = CreateInstance(
                CLSID_NullableTests,
                IID_INullableInterface_Automation,
                &pTests );

        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( pTests != nullptr );

        SECTION( "Nullable parameter" )
        {
            SECTION( "Non-null value" )
            {
                CoCallback cb(123);
                REQUIRE( pTests->NullableParameter(&cb) == 123 );
            }

            SECTION( "Null value" )
            {
                REQUIRE( pTests->NullableParameter(nullptr) == 0 );
            }
        }

        SECTION( "Non-null parameter" )
        {
            SECTION( "Non-null value" )
            {
                CoCallback cb(1234);
                uint32_t output;
                hr = pTests->NonnullParameter(&cb, &output);
                REQUIRE( hr == intercom::SC_OK );
                REQUIRE( output == 1234 );
            }

            SECTION( "Null value" )
            {
                uint32_t output;
                hr = pTests->NonnullParameter(nullptr, &output);
                REQUIRE( hr == intercom::EC_POINTER );
            }
        }

        SECTION( "Nullable return value" )
        {
            SECTION( "Non-null value" )
            {
                ICallback_Automation* pCallback;
                hr = pTests->NullableOutput(123, &pCallback);
                REQUIRE( hr == intercom::SC_OK );
                REQUIRE( pCallback->Callback() == 123 );
                pCallback->Release();
            }

            SECTION( "Null value" )
            {
                ICallback_Automation* pCallback;
                hr = pTests->NullableOutput(0, &pCallback);
                REQUIRE( hr == intercom::SC_OK );
                REQUIRE( pCallback == nullptr );
            }
        }

        SECTION( "Non-null return value" )
        {
            SECTION( "Non-null value" )
            {
                ICallback_Automation* pCallback;
                hr = pTests->NonnullOutput(123, &pCallback);
                REQUIRE( hr == intercom::SC_OK );
                REQUIRE( pCallback->Callback() == 123 );
                pCallback->Release();
            }
        }

        pTests->Release();
    }


    SECTION( "Rust to C++" )
    {
        INullableTests_Automation* pTests = nullptr;
        intercom::HRESULT hr = CreateInstance(
                CLSID_NullableTests,
                IID_INullableTests_Automation,
                &pTests );

        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( pTests != nullptr );

        CoNullableInterface impl;

        SECTION( "Nullable parameter" )
        {
            SECTION( "Non-null value" )
            {
                hr = pTests->NullableParameter(123, &impl);
                REQUIRE( hr == intercom::SC_OK );
            }

            SECTION( "Null value" )
            {
                hr = pTests->NullableParameter(0, &impl);
                REQUIRE( hr == intercom::SC_OK );
            }
        }

        SECTION( "Non-null parameter" )
        {
            SECTION( "Non-null value" )
            {
                hr = pTests->NonnullParameter(1234, &impl);
                REQUIRE( hr == intercom::SC_OK );
            }
        }

        SECTION( "Nullable return value" )
        {
            SECTION( "Non-null value" )
            {
                hr = pTests->NullableOutput(1234, &impl);
                REQUIRE( hr == intercom::SC_OK );
            }

            SECTION( "Null value" )
            {
                hr = pTests->NullableOutput(0, &impl);
                REQUIRE( hr == intercom::SC_OK );
            }
        }

        SECTION( "Non-null return value" )
        {
            SECTION( "Non-null value" )
            {
                hr = pTests->NonnullOutput(1234, &impl);
                REQUIRE( hr == intercom::SC_OK );
            }

            SECTION( "Null value" )
            {
                hr = pTests->NonnullOutput(0, &impl);
                REQUIRE( hr == intercom::EC_POINTER );
            }
        }

        pTests->Release();
    }
}
