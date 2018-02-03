
#ifndef INTERCOM_CPP_DETAIL_UTILITY_H
#define INTERCOM_CPP_DETAIL_UTILITY_H


#include <cstring>
#include <iomanip>
#include <sstream>
#include <type_traits>

namespace intercom
{
namespace detail
{

    /**
     * @brief Tests whether the machine is little endian or not.
     *
     * @return true The machine is little endian.
     * @return false The machine is big endian.
     */
    inline bool is_little_endian()
    {
        // Create a copy of the data with memcpy.
        // Directly casting with e.g. reinterpret_cast will lead to UB.
        int test_value = 1;
        std::array< uint8_t, sizeof( int ) > asBytes;
        std::memcpy( &asBytes, &test_value, sizeof( int ) );
        return asBytes[ 0 ] == 1;
    }

    /**
     * @brief Writes the specified binary data into the stream as hex.
     *
     * @param stream Target stream.
     * @param converter The binary data to write.
     * @return std::ostream& Returns the stream.
     */
    template< typename TData >
    inline std::ostream& write_as_hex(
        std::ostream& stream,
        TData data
    )
    {
        // Create a copy of the data with memcpy.
        // Directly casting with e.g. reinterpret_cast will lead to UB.
        std::array< uint8_t, sizeof(TData) > asBytes;
        std::memcpy( &asBytes, &data, sizeof( TData ) );

        // Determine the order in which we need to print the data.
        stream << std::hex << std::uppercase;
        if( is_little_endian() )
        {
            // On little-endian machine the bytes are on reverse order.
            for( auto byte = asBytes.rbegin(); byte != asBytes.rend(); ++byte )
            {
                // Explicit cast required, otherwise byte is treated as unsigned char.
                stream << static_cast< unsigned int >( ( *byte & 0xF0 ) >> 4 );
                stream << static_cast< unsigned int >( *byte & 0x0F );
            }
        }
        else
        {
            for( auto byte = asBytes.begin(); byte != asBytes.end(); ++byte )
            {
                // Explicit cast required, otherwise byte is treated as unsigned char.
                stream << static_cast< unsigned int >( ( *byte & 0xF0 ) >> 4 );
                stream << static_cast< unsigned int >( *byte & 0x0F );
            }
        }

        return stream;
    }
}
}

#endif
