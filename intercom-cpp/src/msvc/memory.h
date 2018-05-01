
#ifndef INTERCOM_CPP_POSIX_STIRNGS_H
#define INTERCOM_CPP_POSIX_STIRNGS_H

#include <cstdlib>
#include <cstring>
#include <limits>
#include <new>
#include <stdexcept>

#include "datatypes.h"


namespace intercom
{
    /**
     * @brief Allocates memory for BSTR.
     *
     * @return intercom::OLECHAR Returns an uninitialized BSTR.
     */
    inline intercom::BSTR allocate_bstr(
        uint32_t character_count
    )
    {
        // TODO.
        std::terminate();
    }

    /**
     * @brief Frees previously allocated bstr.
     *
     * @param bstr buffer to free.
     */
    inline void free_bstr(
        intercom::BSTR bstr
    ) noexcept
    {
       // TODO.
        std::terminate();
    }

    /**
     * @brief Reallocates a bstr.
     *
     * @param bstr
     * @param new_size
     * @return void*
     */
    inline void* realloc_bstr(
        void* bstr,
        size_t new_size
    )
    {
        // TODO.
        std::terminate();
    }

    /**
     * @brief Gets the number of characters available within the given BSTR.
     *
     */
    inline size_t get_characters_in_bstr(
        const intercom::BSTR bstr
    )
    {
        // TODO.
        std::terminate();
    }

    /**
     * @brief Allocates memory for a string.
     *
     * @return intercom::OLECHAR Returns an uninitialized string with null-terminator.
     */
    template< typename TCharType >
    inline TCharType* allocate_string(
        uint32_t character_count
    )
    {
        // TODO.
        std::terminate();
    }

    /**
     * @brief Reallocates a strng.
     *
     * @param string
     * @param new_size The new size fo the string buffer
     * @return void*
     */
    template< typename TCharType >
    inline void* realloc_string(
        void* string,
        size_t new_size
    )
    {
        // TODO.
        std::terminate();
    }
}

#endif
