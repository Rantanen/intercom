#ifndef INTERCOM_CPP_POSIX_ICLASSFACTORY_H
#define INTERCOM_CPP_POSIX_ICLASSFACTORY_H

#include "../../callingconvention.hpp"
#include "../../posix/iunknown.hpp"

namespace intercom
{

// MIDL_INTERFACE("00000001-0000-0000-C000-000000000046")
struct IClassFactory : public IUnknown
{
public:

    virtual intercom::HRESULT INTERCOM_CC CreateInstance(
        IUnknown *pUnkOuter,
        intercom::REFIID riid,
        void **ppvObject
    ) = 0;

    virtual intercom::HRESULT INTERCOM_CC LockServer(
        int fLock
    ) = 0;

};

static const intercom::IID IID_IClassFactory = { 0x00000001, 0x0000, 0x0000, { 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  0x46 } };

}

#endif
