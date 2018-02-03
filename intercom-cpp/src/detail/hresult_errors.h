
#ifndef INTERCOM_CPP_DETAIL_HRESULTERRORS_H
#define INTERCOM_CPP_DETAIL_HRESULTERRORS_H

#include "hresult.h"

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

    // "FACILITY_NULL" error codes.
    static const NullError ERROR_NOTIMPL = NullError( 0x4001 );
    static const NullError ERROR_NOINTERFACE = NullError( 0x4002 );
    static const NullError ERROR_POINTER = NullError( 0x4003 );
    static const NullError ERROR_ABORT = NullError( 0x4003 );
    static const NullError ERROR_FAIL = NullError( 0x4005 );
    static_assert( NullError( 0x4005 ).error_code() == 0x4005, "Internal check failed: Invalid error code storage." );

    // "FACILITY_WIN32" error codes.
    static const Win32Error ERROR_OUTOFMEMORY = Win32Error( 0x000E );
    static const Win32Error ERROR_INVALIDARG = Win32Error( 0x0057 );

    static const HRESULT S_OK = 0;
    static const HRESULT E_NOTIMPL = null_error( ERROR_NOTIMPL );
    static const HRESULT E_NOINTERFACE = null_error( ERROR_NOINTERFACE );
    static const HRESULT E_POINTER = null_error( ERROR_POINTER );
    static const HRESULT E_ABORT = null_error( ERROR_ABORT );
    static const HRESULT E_FAIL = null_error( ERROR_FAIL );

    static const HRESULT E_OUTOFMEMORY = win32_error( ERROR_OUTOFMEMORY );
    static const HRESULT E_INVALIDARG = win32_error( ERROR_INVALIDARG );
}
}
}

#endif