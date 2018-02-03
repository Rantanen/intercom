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
    static const intercom::HRESULT S_OK = intercom::detail::hresult::S_OK;
    static const intercom::HRESULT E_FAIL = intercom::detail::hresult::E_FAIL;
    static const intercom::HRESULT E_NOTIMPL = intercom::detail::hresult::E_NOTIMPL;
    static const intercom::HRESULT E_NOINTERFACE = intercom::detail::hresult::E_NOINTERFACE;
    static const intercom::HRESULT E_OUTOFMEMORY = intercom::detail::hresult::E_OUTOFMEMORY;
    static const intercom::HRESULT E_INVALIDARG = intercom::detail::hresult::E_INVALIDARG;
    static_assert( E_FAIL == 0x80004005, "Internal check failed: Invalid error code structure." );
}

#ifdef INTERCOM_FLATTEN_DECLARATIONS

static const intercom::HRESULT S_OK = intercom::S_OK;
static const intercom::HRESULT E_FAIL = intercom::E_FAIL;
static const intercom::HRESULT E_NOTIMPL = intercom::E_NOTIMPL;
static const intercom::HRESULT E_NOINTERFACE = intercom::E_NOINTERFACE;
static const intercom::HRESULT E_OUTOFMEMORY = intercom::E_OUTOFMEMORY;
static const intercom::HRESULT E_INVALIDARG = intercom::E_INVALIDARG;

#endif

#endif
