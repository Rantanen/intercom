
#ifndef INTERCOM_CPP_MSVC_VARIANT_H
#define INTERCOM_CPP_MSVC_VARIANT_H

#include <guiddef.h>


// The generated C++ headers and classes expect the VARIANT in intercom namespace.
namespace intercom
{
    typedef ::VARTYPE VARTYPE;
    typedef ::VARENUM VARENUM;
    typedef ::VARIANT VARIANT;

    typedef ::SAFEARR_BSTR SAFEARR_BSTR;
    typedef ::SAFEARR_UNKNOWN SAFEARR_UNKNOWN;
    typedef ::SAFEARR_DISPATCH SAFEARR_DISPATCH;
    typedef ::SAFEARR_VARIANT SAFEARR_VARIANT;
    typedef ::SAFEARR_HAVEIID SAFEARR_HAVEIID;

    typedef ::BYTE_SIZEDARR BYTE_SIZEDARR;
    typedef ::WORD_SIZEDARR WORD_SIZEDARR;
    typedef ::DWORD_SIZEDARR DWORD_SIZEDARR;

    typedef ::SAFEARRAYUNION SAFEARRAYUNION;
    typedef ::SAFEARRAYBOUND SAFEARRAYBOUND;
    typedef ::SAFEARRAY SAFEARRAY;

    using ::VT_EMPTY;
    using ::VT_NULL;
    using ::VT_I2;
    using ::VT_I4;
    using ::VT_R4;
    using ::VT_R8;
    using ::VT_CY;
    using ::VT_DATE;
    using ::VT_BSTR;
    using ::VT_DISPATCH;
    using ::VT_ERROR;
    using ::VT_BOOL;
    using ::VT_VARIANT;
    using ::VT_UNKNOWN;
    using ::VT_DECIMAL;
    using ::VT_I1;
    using ::VT_UI1;
    using ::VT_UI2;
    using ::VT_UI4;
    using ::VT_I8;
    using ::VT_UI8;
    using ::VT_INT;
    using ::VT_UINT;
    using ::VT_VOID;
    using ::VT_HRESULT;
    using ::VT_PTR;
    using ::VT_SAFEARRAY;
    using ::VT_CARRAY;
    using ::VT_USERDEFINED;
    using ::VT_LPSTR;
    using ::VT_LPWSTR;
    using ::VT_RECORD;
    using ::VT_INT_PTR;
    using ::VT_UINT_PTR;
    using ::VT_FILETIME;
    using ::VT_BLOB;
    using ::VT_STREAM;
    using ::VT_STORAGE;
    using ::VT_STREAMED_OBJECT;
    using ::VT_STORED_OBJECT;
    using ::VT_BLOB_OBJECT;
    using ::VT_CF;
    using ::VT_CLSID;
    using ::VT_VERSIONED_STREAM;
    using ::VT_BSTR_BLOB;
    using ::VT_VECTOR;
    using ::VT_ARRAY;
    using ::VT_BYREF;
    using ::VT_RESERVED;
    using ::VT_ILLEGAL;
    using ::VT_ILLEGALMASKED;
    using ::VT_TYPEMASK;
}

#endif
