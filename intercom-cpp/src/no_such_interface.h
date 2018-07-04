
#ifndef INTERCOM_CPP_NOSUCHINTERFACE_H
#define INTERCOM_CPP_NOSUCHINTERFACE_H

#include <stdexcept>
#include <sstream>

#include "guiddef.h"
#include "error_codes.h"

namespace intercom
{

/**
 * @brief Exception thrown when a COM object does not implement the requested interface.
 *
 */
class NoSuchInterface : public std::invalid_argument
{
public:

    /**
     * @brief Error code associated with the exception.
     *
     */
    static const HRESULT ERROR_CODE = intercom::EC_NOINTERFACE;

    explicit NoSuchInterface(
        const intercom::IID& interface_id
    ) :
        std::invalid_argument( get_error_message( interface_id ) ),
        m_interface_id( interface_id )
    {
    }

    explicit NoSuchInterface(
        const intercom::CLSID& class_id,
        const intercom::IID& interface_id
    ) :
        std::invalid_argument( get_error_message( class_id, interface_id ) ),
        m_interface_id( interface_id )
    {
    }

    /**
     * @brief Returns the error associated with this exception.
     *
     * @return intercom::HRESULT Returns the error code.
     */
    intercom::HRESULT error_code() const noexcept { return ERROR_CODE; }

    /**
     * @brief Gets the interface id associated with the error.
     *
     * @return intercom::IID interface_id cosnt
     */
    intercom::IID interface_id() const noexcept { return m_interface_id; }

private:

    /**
     * @brief Constructs error message for the exception from
     *
     * @return std::string Returns message.
     */
    static std::string get_error_message(
        const intercom::IID& interface_id
    )
    {
        std::stringstream fmt;
        fmt << "Interface \"" << interface_id << "\" not available.";
        return fmt.str();
    }

    /**
     * @brief Constructs error message for the exception from
     *
     * @return std::string Returns message.
     */
    static std::string get_error_message(
        const intercom::CLSID& class_id,
        const intercom::IID& interface_id
    )
    {
        std::stringstream fmt;
        fmt << "Interface \"" << interface_id << "\" not available for the class \"" << class_id << "\".";
        return fmt.str();
    }

private:

     const intercom::IID m_interface_id;
};


}

#endif
