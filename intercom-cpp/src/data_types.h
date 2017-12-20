
#ifndef INTERCOM_CPP_DATATYPES_H
#define INTERCOM_CPP_DATATYPES_H

// Use predefined set if available.
#ifdef _MSC_VER
#include<WinDef.h>
#else

#include <inttypes.h>

typedef int BOOL;
typedef int8_t INT8;
typedef uint8_t UINT8;
typedef int16_t INT16;
typedef uint16_t UINT16;
typedef int32_t INT32;
typedef uint32_t UINT32;
typedef int64_t INT64;
typedef uint64_t UINT64;

typedef uint32_t ULONG;
typedef uint32_t DWORD;
typedef uint16_t WORD;


typedef uint8_t BYTE;

typedef wchar_t* BSTR;

#endif
#endif