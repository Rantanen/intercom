
#ifndef INTERCOM_CPP_POSIX_ISUPPORTERRORINFO_H
#define INTERCOM_CPP_POSIX_ISUPPORTERRORINFO_H

#include "../callingconvention.hpp"
#include "../datatypes.hpp"
#include "../error_codes.hpp"
#include "../guiddef.hpp"

#include "datatypes.hpp"

// MIDL_INTERFACE("1CF2B120-547D-101B-8E65-08002B2BD119")
struct IErrorInfo : public IUnknown
{
public:

	virtual intercom::HRESULT INTERCOM_CC GetGUID(
		intercom::GUID *pGUID) = 0;

	virtual intercom::HRESULT INTERCOM_CC GetSource(
		intercom::BSTR *pBstrSource) = 0;

	virtual intercom::HRESULT INTERCOM_CC GetDescription(
		intercom::BSTR *pBstrDescription) = 0;

	virtual intercom::HRESULT INTERCOM_CC GetHelpFile(
		intercom::BSTR *pBstrHelpFile) = 0;

	virtual intercom::HRESULT INTERCOM_CC GetHelpContext(
		intercom::DWORD *pdwHelpContext) = 0;
};

static const intercom::IID IID_IErrorInfo = {
		0x1CF2B120, 0x547D, 0x101B,
		{ 0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19 } };

// MIDL_INTERFACE("DF0B3D60-548F-101B-8E65-08002B2BD119" )
struct ISupportErrorInfo : public IUnknown
{
public:

    virtual intercom::HRESULT INTERCOM_CC InterfaceSupportsErrorInfo(
        intercom::REFIID riid
    ) = 0;
};

static const intercom::IID IID_ISupportErrorInfo = { 0xDF0B3D60, 0x548F, 0x101B, { 0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19 } };

#endif
