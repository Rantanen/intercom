
#ifndef INTERCOM_CPP_DETAIL_HRESULT_H
#define INTERCOM_CPP_DETAIL_HRESULT_H


#include "../datatypes.hpp"

namespace intercom
{
namespace detail
{
/**
 * @brief Defines helpers for working with HRESULT error codes returned by COM methods on Windows platform.
 *
 * The same concepts are used in intercom to enable cross-platform development.
 *
 */
namespace hresult
{
      /**
     * @brief Error code for HRESULTs.
     *
     */
    typedef uint16_t HRESULT_CODE;

    // Source: https://blogs.msdn.microsoft.com/heaths/2005/07/21/deciphering-an-hresult/
    static const uint8_t SEVERITY_CLASS_SUCCESS = 0b0;
    static const uint8_t SEVERITY_CLASS_INFORMATIONAL = 0b01;
    static const uint8_t SEVERITY_CLASS_WARNING = 0b10;
    static const uint8_t SEVERITY_CLASS_ERROR = 0b11;

    // Source: https://msdn.microsoft.com/en-us/library/windows/desktop/ms690088(v=vs.85).aspx
    static const uint8_t ERROR_FACILITY_NULL = 0;
    static const uint8_t ERROR_FACILITY_WIN32 = 7;

    /**
     * @brief Error type for defining error codes.
     *
     */
    class Error
    {
    public:

        constexpr Error(
            uint16_t error_code
        ) :
        m_error_code( error_code )
        {
        }

        constexpr uint16_t error_code() const noexcept { return m_error_code; }

    private:
        uint16_t m_error_code;
    };

    /**
     * @brief Error type for FACILITY_NULL errors.
     *
     */
    class NullError : public Error
    {
        using Error::Error;
    };

    /**
     * @brief Error type for FACILITY_WIN32 errors.
     *
     */
    class Win32Error : public Error
    {
        using Error::Error;
    };

     /**
     * @brief Creates HRESULT error code from the specified values.
     *
     * @param severity The severity of the error.
     * @param facility
     * @param error_code The actual error code.
     * @return constexpr HRESULT
     */
     constexpr ::intercom::HRESULT error(
         uint8_t facility,
         HRESULT_CODE error_code
     ) noexcept
     {
         return ((HRESULT)
                ( static_cast< uint32_t >( intercom::detail::hresult::SEVERITY_CLASS_ERROR ) << 31) |
                ( static_cast< uint32_t >( facility ) << 16) |
                ( static_cast< uint16_t >( error_code ) ) );
     }

    /**
     * @brief Creates HRESULT error code from the specified values.
     *
     * @param null_error The actual error code.
     * @return constexpr HRESULT
     */
     constexpr ::intercom::HRESULT null_error(
         uint16_t error_code
     ) noexcept
     {
         return error( intercom::detail::hresult::ERROR_FACILITY_NULL, NullError( error_code ).error_code() );
     }

     /**
     * @brief Creates HRESULT error code from the specified values.
     *
     * @param win32_error The actual error code.
     * @return constexpr HRESULT
     */
     constexpr ::intercom::HRESULT win32_error(
         uint16_t error_code
     ) noexcept
     {
         return error( intercom::detail::hresult::ERROR_FACILITY_WIN32, Win32Error( error_code ).error_code() );
     }

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
}
}

#endif
