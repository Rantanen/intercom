
#ifndef INTERCOM_CPP_GUIDDEF_H
#define INTERCOM_CPP_GUIDDEF_H

#include "data_types.h"

typedef struct _GUID {
    DWORD Data1;
    WORD Data2;
    WORD Data3;
    BYTE Data4[8];
} GUID;


#define __IID_DEFINED__
typedef GUID IID;

#define CLSID_DEFINED
typedef IID CLSID;

typedef const IID& REFCLSID;
typedef const IID& REFIID;


#endif