
#ifndef INTERCOM_CPP_POSIX_VARIANT_H
#define INTERCOM_CPP_POSIX_VARIANT_H

#include <inttypes.h>
#include <iomanip>
#include <sstream>

#include "datatypes.hpp"

namespace intercom
{

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
    }
};

}

// Visual C++ does not declare the structs in their own namespace.
// Define INTERCOM_FLATTEN_DECLARATIONS to mimic.
#ifdef INTERCOM_FLATTEN_DECLARATIONS

using VARTYPE = intercom::VARTYPE;
using VARIANT = intercom::VARIANT;

#endif

#endif
