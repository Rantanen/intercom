#ifndef INTERCOM_CPP_MSVC_DATATYPES_H
#define INTERCOM_CPP_MSVC_DATATYPES_H

#include <cstdint>

// Use predefined set if available.
#include<WinDef.h>

// The generated C++ headers and classes expect the data types in intercom namespace.
namespace intercom
{
    typedef INT8 INT8;
    typedef UINT8 UINT8;
    typedef INT16 INT16;
    typedef UINT16 UINT16;
    typedef INT32 INT32;
    typedef UINT32 UINT32;
    typedef INT64 INT64;
    typedef UINT64 UINT64;

    typedef BOOL BOOL;
    typedef BYTE BYTE;
    typedef ULONG ULONG;
    typedef DWORD DWORD;
    typedef WORD WORD;

    typedef OLECHAR OLECHAR;
    typedef BSTR BSTR;

    typedef HRESULT HRESULT;

    //! 32-bit reference counter. unsigned long is 32-bit in Windows and 64-bit on Unix.
    typedef unsigned long REF_COUNT_32;
}

#endif