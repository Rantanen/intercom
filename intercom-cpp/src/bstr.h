
#ifndef INTERCOM_CPP_BSTR_H
#define INTERCOM_CPP_BSTR_H

#include <cassert>

#include "datatypes.h"
#include "detail/bstr_buffer.h"

namespace intercom
{

/**
 * @brief Holds a BSTR string.
 *
 * A BSTR is a null terminated UTF-16 string prefixed with a length.
 * On Windows plataform it is allocated with the function SysAllocString.
 * On other platforms standard libc malloc is used.
 */
class Bstr
{
// Initialization methods.
public:

    /**
     * @brief Initializes Bstr string from UTF-8 formatted string.
     *
     * @param utf8_string
     * @return Bstr The
     */
    static Bstr from_utf8(
        char* utf8_string
    );

    /**
     * @brief Initializes Bstr string from a UTF-8 formatted string.
     *
     * @param utf16_string
     * @return Bstr
     */
    static Bstr from_utf16(
        char16_t* utf16_string
    );

    /**
     * @brief Initializes Bstr string from a string formatted using the current
     *
     */
    static Bstr from_ansi(
        char* locale_string
    );

    /**
     * @brief Clones a BSTR string into a new Bstr string.
     *
     */
    static Bstr clone(
        intercom::BSTR bstr_string
    );

    /**
     * @brief Initializes the Bstr from an existing string.
     *
     * @param attached The string attached to the new Bstr object.
     */
    Bstr(
        intercom::BSTR&& attached
    ) :
        m_value( attached )
    {
        attached = nullptr;
    }

    /**
     * @brief Casts the
     *
     * @return operator const intercom::Bstr const
     */
    operator const intercom::BSTR() const noexcept { return m_value; }

    intercom::BSTR* operator&() { assert( m_value == nullptr ); return &m_value; }

// Private data.
private:

    intercom::BSTR m_value;

};

}

#endif