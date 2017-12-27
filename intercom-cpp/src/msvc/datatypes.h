#ifndef INTERCOM_CPP_MSVC_DATATYPES_H
#define INTERCOM_CPP_MSVC_DATATYPES_H

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

    typedef BSTR BSTR;
}

#endif