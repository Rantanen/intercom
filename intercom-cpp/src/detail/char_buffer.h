
#ifndef INTERCOM_CPP_DETAIL_CHARBUFFER_H
#define INTERCOM_CPP_DETAIL_CHARBUFFER_H

#include "../datatypes.h"
#include "../memory.h"

#include <cassert>
#include <cstring>

namespace intercom
{
namespace detail
{

/**
 * @brief Holds a string allocated with std::malloc.
 */
template< typename TCharType >
class CharBuffer
{
// Initialization methods.
public:

    /**
     * @brief Initializes the buffer from an existing string.
     *
     * @param attached The string attached to the new CharBuffer object.
     */
    CharBuffer(
        TCharType*&& attached
    ) :
        m_value( std::forward< TCharType* >( attached ) )
    {
        attached = nullptr;
    }

    ~CharBuffer()
    {
        std::free( m_value );
    }

    /**
     * @brief Casts the buffer into appropriate string.
     *
     * @return operator const TCharType* const
     */
    operator const TCharType*() const noexcept { return m_value; }

    TCharType** operator&() { return &m_value; }

    /**
     * @brief Comparsion operator for check
     *
     * @return true Returns true if no buffer is attached.
     * @return false Returns false when a valid buffer is attached.
     */
    bool operator==(
        std::nullptr_t
    ) const noexcept
    { return m_value == nullptr; }

    /**
     * @brief Gets the number of characters in the buffer.
     *
     * @return size_t Returns the number of characters in the buffer.
     */
    size_t character_count() const noexcept
    {
        return std::strlen( m_value );
    }

    /**
     * @brief Detaches the buffer.
     *
     * @return intercom::BSTR
     */
    TCharType* detach() noexcept
    {
        TCharType* detached = m_value; m_value = nullptr;
        return detached;
    }

// Private data.
private:

    TCharType* m_value;

};

}
}

#endif