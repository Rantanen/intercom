
#ifndef INTERCOM_CPP_MSVC_GUIDDEF_H
#define INTERCOM_CPP_MSVC_GUIDDEF_H

#include <guiddef.h>


// The generated C++ headers and classes expect the IID, GUID and CLSID in intercom namespace.
namespace intercom
{
    typedef IID IID;
    typedef IID CLSID;
    typedef GUID GUID;
}

#endif
