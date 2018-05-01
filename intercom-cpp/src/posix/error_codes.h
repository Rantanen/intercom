#ifndef INTERCOM_CPP_POSIX_ERRRORCODES_H
#define INTERCOM_CPP_POSIX_ERRRORCODES_H

#include <inttypes.h>
#include "../detail/hresult_errors.h"

/**
 * @brief Defines the error codes use by "intercom".
 *
 */
namespace intercom
{
    static const intercom::HRESULT SC_OK = intercom::detail::hresult::SC_OK;
    static const intercom::HRESULT EC_FAIL = intercom::detail::hresult::EC_FAIL;
    static const intercom::HRESULT EC_NOTIMPL = intercom::detail::hresult::EC_NOTIMPL;
    static const intercom::HRESULT EC_NOINTERFACE = intercom::detail::hresult::EC_NOINTERFACE;
    static const intercom::HRESULT EC_OUTOFMEMORY = intercom::detail::hresult::EC_OUTOFMEMORY;
    static const intercom::HRESULT EC_INVALIDARG = intercom::detail::hresult::EC_INVALIDARG;
    static_assert( EC_FAIL == 0x80004005, "Internal check failed: Invalid error code structure." );
}

#ifdef INTERCOM_FLATTEN_DECLARATIONS

static const intercom::HRESULT S_OK = intercom::SC_OK;
static const intercom::HRESULT E_FAIL = intercom::EC_FAIL;
static const intercom::HRESULT E_NOTIMPL = intercom::EC_NOTIMPL;
static const intercom::HRESULT E_NOINTERFACE = intercom::EC_NOINTERFACE;
static const intercom::HRESULT E_OUTOFMEMORY = intercom::EC_OUTOFMEMORY;
static const intercom::HRESULT E_INVALIDARG = intercom::EC_INVALIDARG;

#endif

#endif
