
#ifndef INTERCOM_CPP_RUNTIMERROR_H
#define INTERCOM_CPP_RUNTIMERROR_H

#include <memory>
#include <stdexcept>
#include <sstream>

#include "guiddef.h"
#include "error_codes.h"

namespace intercom
{

/**
 * @brief A generic runtime error thrown by the intercom.
 *
 */
class RuntimeError : public std::runtime_error
{
public:

    /**
     * @brief Initializes new RuntimeError.
     *
     */
    RuntimeError(
        intercom::HRESULT error_code,
        const char* message
    ) :
        std::runtime_error( message ),
        m_error_code( error_code )
    {
    }

    /**
     * @brief Initializes new RuntimeError.
     *
     */
    RuntimeError(
        intercom::HRESULT error_code,
        const std::string& message
    ) :
        std::runtime_error( message ),
        m_error_code( error_code )
    {
    }

    /**
     * @brief Initializes new RuntimeError.
     *
     */
    RuntimeError(
        intercom::HRESULT error_code,
        std::ostream& stream
    ) :
        std::runtime_error( stream_to_string( stream ) ),
        m_error_code( error_code )
    {
    }

    /**
     * @brief Returns the error associated with this exception.
     *
     * @return intercom::HRESULT Returns the error code.
     */
    intercom::HRESULT error_code() const noexcept { return m_error_code; }

private:

    static std::string stream_to_string(
         std::ostream& stream
    )
    {
        stream.flush();
        std::ostringstream message;
        message << stream.rdbuf();
        return message.str();
    }

    intercom::HRESULT m_error_code;
};

}

#endif
