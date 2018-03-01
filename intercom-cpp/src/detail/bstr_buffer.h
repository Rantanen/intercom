
#ifndef INTERCOM_CPP_DETAIL_BSTRBUFFER_H
#define INTERCOM_CPP_DETAIL_BSTRBUFFER_H

#include "../datatypes.h"
#include "../memory.h"

#include <cassert>

namespace intercom
{
namespace detail
{

/**
 * @brief Holds a BSTR string.
 *
 * A BSTR is a null terminated UTF-16 string prefixed with a length.
 * On Windows platform it is allocated with the function SysAllocString.
 * On other platforms standard libc malloc is used.
 */
class BstrBuffer
{
// Initialization methods.
public:

    /**
     * @brief Initializes the Bstr from an existing string.
     *
     * @param attached The string attached to the new Bstr object.
     */
    BstrBuffer(
        intercom::BSTR&& attached
    ) :
        m_value( std::forward< intercom::BSTR >( attached ) )
    {
        attached = nullptr;
    }

    ~BstrBuffer()
    {
        intercom::free_bstr( m_value );
    }

    /**
     * @brief Casts the
     *
     * @return operator const intercom::Bstr const
     */
    operator const intercom::BSTR() const noexcept { return m_value; }

    intercom::BSTR* operator&() { return &m_value; }

    /**
     * @brief Gets the number of characters in the buffer.
     *
     * @return size_t Returns the number of characters in the buffer.
     */
    size_t character_count() const noexcept
    {
        uint32_t data_length_in_bytes;
        std::memcpy( &data_length_in_bytes, reinterpret_cast< char* >( m_value ) - 4, sizeof( uint32_t ) );
        return data_length_in_bytes / sizeof( intercom::OLECHAR );
    }

    /**
     * @brief Detaches the buffer.
     *
     * @return intercom::BSTR
     */
    intercom::BSTR detach() noexcept
    {
        intercom::BSTR detached = m_value; m_value = nullptr;
        return detached;
    }

// Private data.
private:

    intercom::BSTR m_value;

};

}
}

#endif
