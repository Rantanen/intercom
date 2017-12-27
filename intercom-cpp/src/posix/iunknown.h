
#ifndef INTERCOM_CPP_POSIX_IUNKNOWN_H
#define INTERCOM_CPP_POSIX_IUNKNOWN_H

#include "../callingconvention.h"
#include "../data_types.h"
#include "../error_codes.h"
#include "../guiddef.h"
#include "../msdef.h"



// MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
struct IUnknown
{
public:

    virtual HRESULT INTERCOM_CC QueryInterface(
        REFIID riid,
        void **ppvObject
    ) = 0;

    virtual ULONG INTERCOM_CC AddRef() = 0;


    virtual ULONG INTERCOM_CC Release() = 0;
};

static const IID IID_IUnknown = { 0x00000000, 0x0000, 0x0000, { 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x46 } };

#endif