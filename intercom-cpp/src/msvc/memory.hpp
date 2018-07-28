
#ifndef INTERCOM_CPP_POSIX_STIRNGS_H
#define INTERCOM_CPP_POSIX_STIRNGS_H

#include <cstdlib>
#include <cstring>
#include <limits>
#include <new>
#include <stdexcept>

#include "datatypes.hpp"

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
        return SysAllocStringLen( nullptr, character_count );
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
        SysFreeString( bstr );
    }

    /**
     * @brief Reallocates a bstr.
     *
     * @param bstr
     * @param new_size
     * @return void*
     */
    inline intercom::BSTR* realloc_bstr(
        intercom::BSTR* bstr,
        size_t new_size
    )
    {
        int int_size = _internal::checked_cast< int >( new_size );
        if( ! SysReAllocStringLen( OUT bstr, nullptr, int_size ) )
            throw std::bad_alloc();
        return bstr;
    }

    /**
     * @brief Gets the number of characters available within the given BSTR.
     *
     */
    inline size_t get_characters_in_bstr(
        const intercom::BSTR bstr
    )
    {
        return SysStringLen( bstr );
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
        TCharType* string = reinterpret_cast< TCharType* >(
                CoTaskMemAlloc( sizeof( TCharType ) * ( character_count + 1 ) ) );
        string[ character_count ] = 0;

        return string;
    }

    /**
     * @brief Deallocates the string.
     */
    template< typename TCharType >
    inline void free_string(
        TCharType* string
    )
    {
        CoTaskMemFree( string );
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
        return CoTaskMemRealloc( string, new_size );
    }
}

#endif
