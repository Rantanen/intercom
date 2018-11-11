
#ifndef INTERCOM_CPP_POSIX_VARIANT_H
#define INTERCOM_CPP_POSIX_VARIANT_H

#include <inttypes.h>
#include <iomanip>
#include <sstream>

#include "datatypes.hpp"
#include "iunknown.hpp"
#include "idispatch.hpp"

namespace intercom
{

    class IRecordInfo;
    class SAFEARRAY;

    typedef unsigned short VARTYPE;

    enum VARENUM {
        VT_EMPTY = 0,
        VT_NULL = 1,
        VT_I2 = 2,
        VT_I4 = 3,
        VT_R4 = 4,
        VT_R8 = 5,
        VT_CY = 6,
        VT_DATE = 7,
        VT_BSTR = 8,
        VT_DISPATCH = 9,
        VT_ERROR = 10,
        VT_BOOL = 11,
        VT_VARIANT = 12,
        VT_UNKNOWN = 13,
        VT_DECIMAL = 14,
        VT_I1 = 16,
        VT_UI1 = 17,
        VT_UI2 = 18,
        VT_UI4 = 19,
        VT_I8 = 20,
        VT_UI8 = 21,
        VT_INT = 22,
        VT_UINT = 23,
        VT_VOID = 24,
        VT_HRESULT = 25,
        VT_PTR = 26,
        VT_SAFEARRAY = 27,
        VT_CARRAY = 28,
        VT_USERDEFINED = 29,
        VT_LPSTR = 30,
        VT_LPWSTR = 31,
        VT_RECORD = 36,
        VT_INT_PTR = 37,
        VT_UINT_PTR = 38,
        VT_FILETIME = 64,
        VT_BLOB = 65,
        VT_STREAM = 66,
        VT_STORAGE = 67,
        VT_STREAMED_OBJECT = 68,
        VT_STORED_OBJECT = 69,
        VT_BLOB_OBJECT = 70,
        VT_CF = 71,
        VT_CLSID = 72,
        VT_VERSIONED_STREAM = 73,
        VT_BSTR_BLOB = 0xFFF,
        VT_VECTOR = 0x1000,
        VT_ARRAY = 0x2000,
        VT_BYREF = 0x4000,
        VT_RESERVED = 0x8000,
        VT_ILLEGAL = 0xffff,
        VT_ILLEGALMASKED = 0xfff,
        VT_TYPEMASK = 0xfff
    };

    struct VARIANT {
        VARTYPE vt;
        WORD wReserved1;
        WORD wReserved2;
        WORD wReserved3;
        union {
            LONGLONG     llVal;
            LONG         lVal;
            BYTE         bVal;
            SHORT        iVal;
            FLOAT        fltVal;
            DOUBLE       dblVal;
            VARIANT_BOOL boolVal;
            SCODE        scode;
            CY           cyVal;
            DATE         date;
            BSTR         bstrVal;
            IUnknown     *punkVal;
            IDispatch    *pdispVal;
            SAFEARRAY    *parray;
            BYTE         *pbVal;
            SHORT        *piVal;
            LONG         *plVal;
            LONGLONG     *pllVal;
            FLOAT        *pfltVal;
            DOUBLE       *pdblVal;
            VARIANT_BOOL *pboolVal;
            SCODE        *pscode;
            CY           *pcyVal;
            DATE         *pdate;
            BSTR         *pbstrVal;
            IUnknown     **ppunkVal;
            IDispatch    **ppdispVal;
            SAFEARRAY    **pparray;
            VARIANT      *pvarVal;
            PVOID        byref;
            CHAR         cVal;
            USHORT       uiVal;
            ULONG        ulVal;
            ULONGLONG    ullVal;
            INT          intVal;
            UINT         uintVal;
            DECIMAL      *pdecVal;
            CHAR         *pcVal;
            USHORT       *puiVal;
            ULONG        *pulVal;
            ULONGLONG    *pullVal;
            INT          *pintVal;
            UINT         *puintVal;
            struct {
                PVOID       pvRecord;
                IRecordInfo *pRecInfo;
            } __VARIANT_NAME_4;
        };
    };

    struct SAFEARR_BSTR { ULONG Size; BSTR* aBstr; };
    struct SAFEARR_UNKNOWN { ULONG Size; IUnknown** apUnknown; };
    struct SAFEARR_DISPATCH { ULONG Size; IDispatch** apDispatch; };
    struct SAFEARR_VARIANT { ULONG Size; VARIANT* aVariant; };
    // struct SAFEARR_BRECORD { ULONG Size; BRECORD* aRecord; };
    struct SAFEARR_HAVEIID { ULONG Size; IUnknown** apUnknown; IID iid; };

    template<typename TData>
    struct SIZED_ARRAY { unsigned long clSize; TData* pData; };
    typedef SIZED_ARRAY<BYTE> BYTE_SIZEDARR;
    typedef SIZED_ARRAY<unsigned short> WORD_SIZEDARR;
    typedef SIZED_ARRAY<unsigned long> DWORD_SIZEDARR;

    union SAFEARRAYUNION {
        SAFEARR_BSTR BstrStr;
        SAFEARR_UNKNOWN UnknownStr;
        SAFEARR_DISPATCH DispatchStr;
        SAFEARR_VARIANT VariantStr;
        // SAFEARR_BRECORD RecordStr;
        SAFEARR_HAVEIID HaveIidStr;
        BYTE_SIZEDARR ByteStr;
        WORD_SIZEDARR WordStr;
        DWORD_SIZEDARR LongStr;
        // HYPER_SIZEDARR HyperStr;
    };

    struct SAFEARRAYBOUND {
        ULONG cElements;
        LONG lLbound;
    };

    struct SAFEARRAY {
        USHORT cDims;
        USHORT fFeatures;
        ULONG cbElements;
        ULONG cLocks;
        SAFEARRAYUNION uArrayStructs;
        SAFEARRAYBOUND rgsabound[];
    };
}

// Visual C++ does not declare the structs in their own namespace.
// Define INTERCOM_FLATTEN_DECLARATIONS to mimic.
#ifdef INTERCOM_FLATTEN_DECLARATIONS

using VARTYPE = intercom::VARTYPE;
using VARIANT = intercom::VARIANT;

#endif

#endif
