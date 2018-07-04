
#ifndef INTERCOM_CPP_POSIX_CONVERSIONS_H
#define INTERCOM_CPP_POSIX_CONVERSIONS_H

#include <cstdlib>

#include "converter.h"
#include "memory.h"
#include "../detail/bstr_buffer.h"
#include "../detail/char_buffer.h"

namespace intercom
{
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
        *bstr_string = nullptr;
        if( utf8_string == nullptr )
            return;

        // Prepare output buffer.
        size_t input_length = strlen( utf8_string );
        intercom::detail::BstrBuffer target( intercom::allocate_bstr( input_length ) );

        size_t output_length = target.character_count();
        intercom::posix::Converter converter(
                intercom::posix::Converter::Encoding::Utf8,
                intercom::posix::Converter::Encoding::Utf16 );
        converter.convert( utf8_string, input_length, &target, &output_length, &intercom::realloc_bstr );

        *bstr_string = target.detach();
    }

    /**
     * @brief Converts BSTR string to UTF-8.
     *
     * The received char* must be deallocated with "free".
     */
    inline void bstr_to_utf8(
        const intercom::BSTR bstr_string,
        char** utf8_string
    )
    {
        *utf8_string = nullptr;
        if( bstr_string == nullptr )
            return;

        // Prepare output buffer.
        size_t input_length = get_characters_in_bstr( bstr_string );
        size_t output_length = input_length;
        intercom::detail::CharBuffer< char > target( intercom::allocate_string< char >( output_length ) );
        if( target == nullptr )
            throw std::bad_alloc();

        intercom::posix::Converter converter(
                intercom::posix::Converter::Encoding::Utf16,
                intercom::posix::Converter::Encoding::Utf8 );
        converter.convert( bstr_string, input_length, &target, &output_length, &intercom::realloc_string< char > );

        *utf8_string = target.detach();
    }
}

#endif
