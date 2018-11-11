#ifndef INTERCOM_CPP_POSIX_DATATYPES_H
#define INTERCOM_CPP_POSIX_DATATYPES_H


#include <inttypes.h>

// Declare data types used on Windows platform to facilitate code sharing.
namespace intercom
{
    typedef int32_t INT;
    typedef uint32_t UINT;
    typedef int8_t INT8;
    typedef uint8_t UINT8;
    typedef int16_t INT16;
    typedef uint16_t UINT16;
    typedef int32_t INT32;
    typedef uint32_t UINT32;
    typedef int64_t INT64;
    typedef uint64_t UINT64;

    typedef int BOOL;
    typedef uint32_t DWORD;
    typedef uint16_t WORD;

    typedef int8_t CHAR;
    typedef int16_t SHORT;
    typedef int32_t LONG;
    typedef int64_t LONGLONG;
    typedef uint8_t BYTE;
    typedef uint16_t USHORT;
    typedef uint32_t ULONG;
    typedef uint64_t ULONGLONG;
    typedef double DOUBLE;
    typedef float FLOAT;

    typedef char16_t OLECHAR;
    typedef OLECHAR* BSTR;

    typedef uint32_t HRESULT;
    typedef uint32_t SCODE;

    typedef double DATE;
    typedef int16_t VARIANT_BOOL;
    typedef int64_t CY, CURRENCY;

    typedef void* PVOID;

    //! 32-bit reference counter. unsigned long is 32-bit in Windows and 64-bit on Unix.
    typedef uint32_t REF_COUNT_32;

    struct DECIMAL {
        WORD wReserved;
        BYTE scale;
        BYTE sign;
        ULONG Hi32;
        ULONGLONG Lo64;
    };
}

// Visual C++ does not declare the data types in their own namespace.
// Define INTERCOM_FLATTEN_DECLARATIONS to mimic.
#ifdef INTERCOM_FLATTEN_DECLARATIONS

using INT8 = intercom::INT8;
using UINT8 = intercom::UINT8;
using INT16 = intercom::INT16;
using UINT16 = intercom::UINT16;
using INT32 = intercom::INT32;
using UINT32 = intercom::UINT32;
using INT64 = intercom::INT64;
using UINT64 = intercom::UINT64;

using BOOL = intercom::BOOL;
using BYTE = intercom::BYTE;
using ULONG = intercom::ULONG;
using DWORD = intercom::DWORD;
using WORD = intercom::WORD;

using BSTR = intercom::BSTR;
using HRESULT = intercom::HRESULT;

#endif

#endif
