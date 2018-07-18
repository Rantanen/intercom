
#ifndef INTERCOM_CPP_GUIDDEF_H
#define INTERCOM_CPP_GUIDDEF_H


#ifdef _MSC_VER
#include "msvc/guiddef.hpp"
#else
#include "posix/guiddef.hpp"
#endif

#include "detail/utility.hpp"

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
}

#endif
