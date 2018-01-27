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
    static const intercom::HRESULT E_INVALIDARG = intercom::detail::hresult::E_INVALIDARG;
}

#ifdef INTERCOM_FLATTEN_DECLARATIONS

static const intercom::HRESULT S_OK = intercom::S_OK;
static const intercom::HRESULT E_FAIL = intercom::E_FAIL;
static const intercom::HRESULT E_NOTIMPL = intercom::E_NOTIMPL;
static const intercom::HRESULT E_INVALIDARG = intercom::E_INVALIDARG;

#endif

#endif
