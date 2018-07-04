
#ifndef INTERCOM_CPP_DLLGETCLASSOBJECT
#define INTERCOM_CPP_DLLGETCLASSOBJECT

#ifdef _MSC_VER
#include <Objbase.h>
#else

#include "comdef.h"
#include "datatypes.h"
#include "error_codes.h"
#include "guiddef.h"

intercom::HRESULT DllGetClassObject(
  intercom::REFCLSID rclsid,
  intercom::REFIID riid,
  void** ppv
);

#endif

#endif
