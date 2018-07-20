
#ifndef INTERCOM_CPP_ERRORCODES_H
#define INTERCOM_CPP_ERRORCODES_H

#include <inttypes.h>
#include "detail/hresult_errors.hpp"

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
    static const intercom::HRESULT EC_POINTER = intercom::detail::hresult::EC_POINTER;
    static const intercom::HRESULT EC_CLASSNOTREG = intercom::detail::hresult::EC_CLASSNOTREG;
    static_assert( EC_FAIL == 0x80004005, "Internal check failed: Invalid error code structure." );

    /**
     * @brief Checks whether the error represents success.
     *
     * @param error_code Specifies the error code.
     * @return true If the error code represents success.
     * @return false If the error code represents failure.
     */
    constexpr bool succeeded(
         intercom::HRESULT error_code
     ) noexcept
     {
         // The first bit of HRESULT codes is always '1' if the error code indicates a failure.
         return static_cast< int32_t >( error_code ) >= 0;
     }

    /**
     * @brief Checks whether the error represents failure.
     *
     * @param error_code
     * @return true If the error code represents failure.
     * @return false If the error code represents success.
     */
     constexpr bool failed(
         intercom::HRESULT error_code
     ) noexcept
     {
         // The first bit of HRESULT codes is always '1' if the error code indicates a failure.
         return static_cast< int32_t >( error_code ) < 0;
     }
}

// Use predefined set if available.
#ifdef _MSC_VER

#include<Winerror.h>
static_assert( E_FAIL == intercom::EC_FAIL, "Definition of intercom errors are invalid." );

#else
#include "posix/error_codes.hpp"
#endif


#endif
