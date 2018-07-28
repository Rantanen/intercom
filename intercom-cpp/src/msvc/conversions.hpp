
#ifndef INTERCOM_CPP_MSVC_CONVERSIONS_H
#define INTERCOM_CPP_MSVC_CONVERSIONS_H

#include <cstdlib>

#include "memory.hpp"
#include "../detail/bstr_buffer.hpp"
#include "../detail/char_buffer.hpp"

namespace intercom
{
    /**
     * @brief Returns the last WINAPI error as an error message.
     */
    inline std::string get_last_error() 
    {
        // Get the error message, if any.
        DWORD error = GetLastError();
        if( error == 0 )
            return std::string( "The operation completed successfully" );

        LPSTR messageBuffer = nullptr;
        size_t size = FormatMessageA(
                FORMAT_MESSAGE_ALLOCATE_BUFFER |
                    FORMAT_MESSAGE_FROM_SYSTEM |
                    FORMAT_MESSAGE_IGNORE_INSERTS,
                nullptr, error,
                MAKELANGID( LANG_NEUTRAL, SUBLANG_DEFAULT ),
                ( LPSTR ) &messageBuffer, 0, nullptr);

        std::string message(messageBuffer, size);

        // Free the temporary string.
        LocalFree(messageBuffer);

        return message;
    }

    /**
     * @brief Converts UTF-8 string to BSTR.
     *
     * The received BSTR must be deallocated with "intercom::free_bstr".
     */
    inline void utf8_to_bstr(
        const char* utf8_string,
        intercom::BSTR* bstr_string
    )
    {
        // Reset output.
        *bstr_string = nullptr;

        // Null UTF-8 string is returned as null BSTR.
        if( utf8_string == nullptr )
            return;

        int utf8_len = _internal::checked_cast< int >( strlen( utf8_string ) );
        int required_size = MultiByteToWideChar( CP_UTF8, 0,
                utf8_string, utf8_len,
                nullptr, 0 );

        if( required_size == 0 )
        {
            _ASSERTE( false );
            throw std::runtime_error( get_last_error() );
        }

        *bstr_string = intercom::allocate_bstr( required_size );
        int written = MultiByteToWideChar( CP_UTF8, 0,
                utf8_string, utf8_len,
                *bstr_string, required_size );

        if( written != required_size ) 
        {
            std::string err_msg = get_last_error();
            intercom::free_bstr( *bstr_string );
            *bstr_string = nullptr;
            throw std::runtime_error( err_msg );
        }
    }

    /**
     * @brief Converts BSTR string to UTF-8.
     *
     * The received char* must be deallocated with "intercom::free_string".
     */
    inline void bstr_to_utf8(
        const intercom::BSTR bstr_string,
        char** utf8_string
    )
    {
        *utf8_string = nullptr;
        if( bstr_string == nullptr )
        {
            // Null BSTR is equivalent to an empty string.
            *utf8_string = intercom::allocate_string< char >( 0 );
            return;
        }

        size_t bstr_len = intercom::get_characters_in_bstr( bstr_string );
        if (bstr_len == 0)
        {
            // Empty string.
            *utf8_string = intercom::allocate_string< char >( 0 );
            return;
        }

        int required_size = WideCharToMultiByte( CP_UTF8, 0,
                bstr_string, _internal::checked_cast< int >( bstr_len ),
                nullptr, 0,
                nullptr, nullptr );

        if( required_size == 0 )
        {
            _ASSERTE( false );
            throw std::runtime_error( get_last_error() );
        }

        *utf8_string = intercom::allocate_string< char >( required_size );
        int written = WideCharToMultiByte( CP_UTF8, 0,
                bstr_string,
                _internal::checked_cast< int >( intercom::get_characters_in_bstr( bstr_string ) ),
                *utf8_string, _internal::checked_cast< int >( required_size ),
                nullptr, nullptr );

        if( written != required_size ) 
        {
            std::string err_msg = get_last_error();
            intercom::free_string< char >( *utf8_string );
            *utf8_string = nullptr;
            throw std::runtime_error( err_msg );
        }
    }
}

#endif
