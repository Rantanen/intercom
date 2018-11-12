
#ifndef INTERCOM_CPP_POSIX_IUNKNOWN_H
#define INTERCOM_CPP_POSIX_IUNKNOWN_H

#include "../callingconvention.hpp"
#include "../datatypes.hpp"
#include "../error_codes.hpp"
#include "../guiddef.hpp"


// MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
struct ISupportErrorInfo
{
public:

    virtual intercom::HRESULT INTERCOM_CC InterfaceSupportsErrorInfo(
        intercom::REFIID riid
    ) = 0;
};

static const intercom::IID IID_ISupportErrorInfo = { 0xDF0B3D60, 0x548F, 0x101B, { 0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19 } };

#endif
