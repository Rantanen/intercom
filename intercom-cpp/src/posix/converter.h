#ifndef INTERCOM_CPP_POSIX_CONVERTER_H
#define INTERCOM_CPP_POSIX_CONVERTER_H

#include <algorithm>
#include <cassert>
#include <cerrno>
#include <cstdlib>
#include <cstring>
#include <new>
#include <stdexcept>

#include <iconv.h>

#include "datatypes.h"
#include "../detail/utility.h"


namespace intercom
{
namespace posix
{

/**
 * @brief Converts characters from one character set to another.
 *
 */
class Converter
{
public:

    /**
     * @brief Specifies the encodings available for the conversion.
     *
     */
    enum class Encoding
    {
        Utf8,
        Utf16,
    };

    /**
     * @brief Initializes the converter for converting strings from fromcode to tocode.
     *
     * @param tocode
     * @param fromcode
     */
    explicit Converter(
        Encoding fromcode,
        Encoding tocode
    ) :
    m_state( get_invalid_handle() )
    {
        m_state = iconv_open( get_encoding( tocode ), get_encoding( fromcode ) );
        if( m_state == get_invalid_handle() )
            throw std::runtime_error( std::strerror( errno ) );
    }

    ~Converter()
    {
        int closed = iconv_close( m_state );
        assert( closed == 0 );
    }

    /**
     * @brief Converts the given input characters into ouput.
     *
     * @tparam TInput The type of the input character.
     * @tparam TOutput The type of the output character.
     * @param input Input buffer.
     * @param input_length The number of characters in the input buffer.
     * @param output The output buffer. The buffer is resized to fit the conversion.
     * @param output_length The number of characters available in the output buffer for conversion.
     * @param realloc_output Function for resizing the output buffer.
     */
    template< typename TInput, typename TOutput >
    void convert(
        const TInput* input,
        size_t input_length,
        TOutput** output,
        size_t* output_length,
        void* (*realloc_output)( void*, size_t )
    )
    {
        char* output_temp = reinterpret_cast< char* >( *output );
        size_t output_length_temp = *output_length * sizeof( TOutput );
        size_t uninitialized_bytes = convert_private(
                reinterpret_cast< const char* >( input ),
                input_length * sizeof( TInput ),
                &output_temp, &output_length_temp,
                realloc_output );

        // Null-terminate.
        // We need to adjust the buffer to exact size here in order to set correct length prefix for BSTR strings.
        if( uninitialized_bytes != sizeof( TOutput ) )
            realloc_buffer( &output_temp, &output_length_temp, realloc_output, sizeof( TOutput ) - uninitialized_bytes );
        std::memset( output_temp + ( output_length_temp - sizeof( TOutput ) ), 0, sizeof( TOutput ) );

        *output = reinterpret_cast< TOutput* >( output_temp );
        *output_length = output_length_temp / sizeof( TOutput );
    }

private:

    /**
     * @brief Converts the text in the input buffer to output buffer.
     *
     * @param input
     * @param input_length
     * @param output Output buffer.
     * @param output_length
     * @realloc_output Function for extending the output buffer.
     * @returns Returns the number uninitialized of bytes remaining in the output buffer.
     */
    size_t convert_private(
        const char* input,
        size_t input_length,
        char** output,
        size_t* output_length,
        void* (*realloc_output)( void*, size_t )
    )
    {
        char* input_iterator = const_cast< char* >( input );
        size_t input_remaining = input_length;
        char* output_iterator = *output;
        size_t output_remaining = *output_length;

        while( input_remaining > 0 )
        {
            size_t result = iconv( m_state,
                    &input_iterator, &input_remaining,
                    &output_iterator, &output_remaining );
            if( result == static_cast< size_t >( -1 ) )
            {
                // Extend the output buffer if it was too small.
                switch( errno )
                {
                case E2BIG:
                    output_remaining += extend_buffer( output, output_length, realloc_output );
                    output_iterator = *output + ( *output_length - output_remaining );
                    break;

                // Conversion failed.
                default:
                    throw std::runtime_error( std::strerror( errno ) );
                }
            }
        }

        return output_remaining;
    }

    /**
     * @brief Extends the output buffer.
     *
     * @param output The extended buffer.
     * @param output_length The length of the buffer.
     * @param realloc_output Function for extending the buffer.
     * @return size_t Returns the number of characters the buffer was extended.
     */
    size_t extend_buffer(
        char** output,
        size_t* output_length,
        void* (*realloc_output)( void*, size_t )
    )
    {
        // TODO: Select suitable extension factor after benchmarking.
        size_t extension = std::max( static_cast< size_t >( *output_length * 0.5 ), static_cast< size_t >( 4 ) );

        realloc_buffer( output, output_length, realloc_output, extension );

        return extension;
    }

    /**
     * @brief Reallocates the output buffer.
     *
     * @param output The output buffer.
     * @param output_length The length of the output buffer.
     * @param realloc_output Function for reallocating the buffer.
     * @param realloc_by Specifies the adjustemnt for the buffer.
     * @return size_t Returns the number of characters remining in the extended buffer.
     */
    void realloc_buffer(
        char** output,
        size_t* output_length,
        void* (*realloc_output)( void*, size_t ),
        int64_t realloc_by
    )
    {
        void* reallocated_buffer = realloc_output( *output, *output_length + realloc_by );
        if( reallocated_buffer == nullptr )
            throw std::bad_alloc();

        *output = static_cast< char* >( reallocated_buffer );
        *output_length = *output_length + realloc_by;
    }

    /**
     * @brief Gets the string representation of the specified encoding.
     *
     * @param encoding The encoding.
     * @return const char* String representation for iconv.
     */
    static const char* get_encoding(
        Encoding encoding
    )
    {
        switch( encoding )
        {
        case Encoding::Utf8:
            return "UTF-8";

        // Glibc adds BOM to the beginning of "UTF-16" encoding.
        // We avoid that by specifying the endianess explicitly.
        case Encoding::Utf16:
            return intercom::detail::is_little_endian() ? "UTF-16LE" : "UTF-16BE";
        }
        assert( false );
        return "";
    }

    /**
     * @brief Get the invalid handle value.
     *
     * @return iconv_t Returns a handle which represents an invalid handle.
     */
    static iconv_t get_invalid_handle()
    {
        iconv_t invalid_handle;
        std::memset( &invalid_handle, 0xFF, sizeof( iconv_t ) );
        return invalid_handle;
    }

private:

    iconv_t m_state;

};

}
}

#endif
