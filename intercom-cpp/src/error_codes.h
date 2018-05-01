
#ifndef INTERCOM_CPP_ERRORCODES_H
#define INTERCOM_CPP_ERRORCODES_H

#include <inttypes.h>
#include "detail/hresult_errors.h"

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

// Use predefined set if available.
#ifdef _MSC_VER

#include<Winerror.h>
static_assert( E_FAIL == intercom::EC_FAIL, "Definition of intercom errors are invalid." );

#else
#include "posix/error_codes.h"
#endif


#endif
