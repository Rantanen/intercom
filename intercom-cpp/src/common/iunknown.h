
#ifndef INTERCOM_CPP_IUNKNOWN_H
#define INTERCOM_CPP_IUNKNOWN_H

#include "../comdef.h"
#include "../data_types.h"
#include "../error_codes.h"
#include "../guiddef.h"
#include "../msdef.h"


    MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
    IUnknown
    {
    public:
        virtual /* [id] */ ULONG STDMETHODCALLTYPE AddRef() = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE QueryInterface(
            const IID& riid,
            void **ppvObject
        ) = 0;

        virtual ULONG STDMETHODCALLTYPE Release() = 0;
    };

    static const GUID IID_IUnknown = { 0x00000000, 0x0000, 0x0000, { 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x46 } };

#endif