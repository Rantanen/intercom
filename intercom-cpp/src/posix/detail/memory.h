#ifndef INTERCOM_CPP_POSIX_MEMORY_DETAIL_H
#define INTERCOM_CPP_POSIX_MEMORY_DETAIL_H

#include <cstdint>
#include <cstring>
#include <memory>

namespace intercom
{
namespace posix
{
namespace detail
{

    /**
     * @brief Sets the data length prefix for a BSTR.
     *
     * @param buffer The bstr buffer.
     * @param
     */
    inline void set_bstr_data_length(
        void* buffer,
        uint32_t data_length
    ) noexcept
    {
        std::memcpy( buffer, &data_length, sizeof( data_length ) );
    }
}
}
}

#endif