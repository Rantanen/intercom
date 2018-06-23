
#ifndef INTERCOM_CPP_GUIDDEF_H
#define INTERCOM_CPP_GUIDDEF_H


#ifdef _MSC_VER
#include "msvc/guiddef.h"
#else
#include "posix/guiddef.h"
#endif

#include "detail/utility.h"

namespace intercom
{
    /**
     * @brief Writes the specified IID into a stream.
     *
     * @param stream Target stream.
     * @param iid IID to write to the stream.
     * @return std::ostream& Returns the stream.
     */
    inline std::ostream& operator<<(
        std::ostream& stream,
        const IID& iid
    )
    {
        stream << "{";
        intercom::detail::write_as_hex( stream, iid.Data1 );
        stream << "-";
        intercom::detail::write_as_hex( stream, iid.Data2 );
        stream << "-";
        intercom::detail::write_as_hex( stream, iid.Data3 );
        stream << "-";
        intercom::detail::write_as_hex( stream, iid.Data4[0] );
        intercom::detail::write_as_hex( stream, iid.Data4[1] );
        stream << "-";
        for( size_t d = 2; d < sizeof( iid.Data4 ); ++d )
            intercom::detail::write_as_hex( stream, iid.Data4[d] );
        stream << "}";
        return stream;
    }

    /**
     * @brief Compares two interface ids.
     *
     * @param lhs Left side of the operator.
     * @param rhs Right side of the operator.
     * @return true
     * @return false
     */
    inline bool operator==(
        const intercom::IID& lhs,
        const intercom::IID& rhs
    ) noexcept
    {
        return memcmp( &lhs, &rhs, sizeof( intercom::IID ) ) == 0;
    }

    //! Declaration for a generic hash function for intercom types.
    template< typename THashed >
    struct hash;

    //! Calculates a hash code for a IID.
    template<>
    struct hash<intercom::IID>
    {
        inline size_t operator()(
            const intercom::IID& guid
        ) const noexcept
        {
            size_t hash_code = 0;
            intercom::detail::hash_combine( hash_code, guid.Data1 );
            intercom::detail::hash_combine( hash_code, guid.Data2 );
            intercom::detail::hash_combine( hash_code, guid.Data3 );
            for( uint8_t b : guid.Data4 )
                intercom::detail::hash_combine( hash_code, b );
            return hash_code;
        }
    };
}

#endif
