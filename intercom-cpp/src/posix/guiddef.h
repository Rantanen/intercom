
#ifndef INTERCOM_CPP_POSIX_GUIDDEF_H
#define INTERCOM_CPP_POSIX_GUIDDEF_H

#include <inttypes.h>

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
