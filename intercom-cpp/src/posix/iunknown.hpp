
#ifndef INTERCOM_CPP_POSIX_IUNKNOWN_H
#define INTERCOM_CPP_POSIX_IUNKNOWN_H

#include "../callingconvention.hpp"
#include "../datatypes.hpp"
#include "../error_codes.hpp"
#include "../guiddef.hpp"


// MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
struct IUnknown
{
public:

    virtual intercom::HRESULT INTERCOM_CC QueryInterface(
        intercom::REFIID riid,
        void **ppvObject
    ) = 0;

    virtual uint32_t INTERCOM_CC AddRef() = 0;


    virtual uint32_t INTERCOM_CC Release() = 0;
};

static const intercom::IID IID_IUnknown = { 0x00000000, 0x0000, 0x0000, { 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x46 } };

#endif
