
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
