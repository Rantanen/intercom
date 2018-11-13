
#ifndef INTERCOM_CPP_DETAIL_HRESULTERRORS_H
#define INTERCOM_CPP_DETAIL_HRESULTERRORS_H

#include "hresult.hpp"

namespace intercom
{
namespace detail
{
/**
 * @brief Defines the HRESULT codes used by intercom.
 *
 */
namespace hresult
{
    static const HRESULT SC_OK = 0;
    static const HRESULT SC_FALSE = 1;
    static const HRESULT EC_NOTIMPL = null_error( 0x4001 );
    static const HRESULT EC_NOINTERFACE = null_error( 0x4002 );
    static const HRESULT EC_POINTER = null_error( 0x4003 );
    static const HRESULT EC_ABORT = null_error( 0x4003 );
    static const HRESULT EC_FAIL = null_error( 0x4005 );
    static const HRESULT EC_CLASSNOTREG = itf_error( 0x0154 );
    static_assert( NullError( 0x4005 ).error_code() == 0x4005, "Internal check failed: Invalid error code storage." );

    static const HRESULT EC_OUTOFMEMORY = win32_error( 0x000E );
    static const HRESULT EC_INVALIDARG = win32_error( 0x0057 );
}
}
}

#endif
