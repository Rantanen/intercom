
#ifndef INTERCOM_CPP_POSIX_GUIDDEF_H
#define INTERCOM_CPP_POSIX_GUIDDEF_H

#include <inttypes.h>
#include <iomanip>
#include <sstream>

#include "../detail/utility.h"

namespace intercom
{

typedef struct _GUID {
    uint32_t Data1;
    uint16_t Data2;
    uint16_t Data3;
    uint8_t Data4[8];
} GUID;

typedef struct _IID {
    uint32_t Data1;
    uint16_t Data2;
    uint16_t Data3;
    uint8_t Data4[8];
} IID;

typedef IID CLSID;
typedef const IID& REFCLSID;
typedef const IID& REFIID;

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

// Visual C++ does not declare the structs in their own namespace.
// Define INTERCOM_FLATTEN_DECLARATIONS to mimic.
#ifdef INTERCOM_FLATTEN_DECLARATIONS

#define __IID_DEFINED__
#define CLSID_DEFINED

using GUID = intercom::GUID;
using IID = intercom::IID;
using CLSID = intercom::IID;

using REFCLSID = intercom::REFCLSID;
using REFIID = intercom::REFIID;

#endif

#endif
