
#ifndef INTERCOM_CPP_POSIX_STIRNGS_H
#define INTERCOM_CPP_POSIX_STIRNGS_H

#include <iconv.h>
#include <cstdlib>
#include <cstring>
#include <limits>
#include <new>
#include <stdexcept>

#include "converter.hpp"
#include "datatypes.hpp"
#include "detail/memory.hpp"


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
        // BSTR has the length of the string at the beginning.
        void* buffer = std::malloc( 4 + ( character_count + 1 ) * sizeof( intercom::OLECHAR ) );
        if( buffer == nullptr )
            throw std::bad_alloc();
        intercom::posix::detail::set_bstr_data_length( buffer, character_count * sizeof( intercom::OLECHAR ) );

        // Ensure null-termination.
        std::memset( static_cast< char* >( buffer ) + 4 + ( character_count ) * sizeof( intercom::OLECHAR ),
                0, sizeof( intercom::OLECHAR ) );

        char* bstr = static_cast< char* >( buffer ) + 4;
        return reinterpret_cast< intercom::BSTR >( bstr );
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
        if( bstr == nullptr )
            return;

        char* buffer = reinterpret_cast< char* >( bstr );
        std::free( buffer - 4 );
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
        if( new_size > static_cast< size_t >( std::numeric_limits< uint32_t >::max() ) )
            throw std::invalid_argument( "The maximum length for BSTR exceeded." );

        void* original_buffer = reinterpret_cast< char* >( bstr ) - 4;
        void* extended_buffer = realloc( original_buffer, new_size + 4 );
        if( extended_buffer == nullptr )
            throw std::bad_alloc();

        // Add the length
        intercom::posix::detail::set_bstr_data_length( extended_buffer, new_size - sizeof( intercom::OLECHAR ) );

        return static_cast< char* >( extended_buffer ) + 4;
    }

    /**
     * @brief Gets the number of characters available within the given BSTR.
     *
     */
    inline size_t get_characters_in_bstr(
        const intercom::BSTR bstr
    )
    {
        const void* buffer = reinterpret_cast< const char* >( bstr ) - 4;
        uint32_t data_length_in_bytes;
        std::memcpy( &data_length_in_bytes, buffer, sizeof( data_length_in_bytes ) );
        return data_length_in_bytes / sizeof( intercom::OLECHAR );
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
        // Allocate enough memory to hold the string and null terminator.
        void* buffer = std::malloc( ( character_count + 1 ) * sizeof( TCharType ) );
        if( buffer == nullptr )
            throw std::bad_alloc();

        // Ensure null-termination.
        std::memset( static_cast< char* >( buffer ) + ( character_count ) * sizeof( TCharType ),
                0, sizeof( TCharType ) );

        return reinterpret_cast< TCharType* >( buffer );
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
        void* extended_buffer = realloc( string, new_size );
        if( extended_buffer == nullptr )
            throw std::bad_alloc();
        return extended_buffer;
    }
}

#endif
