
#ifndef INTERCOM_CPP_DLLGETCLASSOBJECT
#define INTERCOM_CPP_DLLGETCLASSOBJECT

#ifdef _MSC_VER
#include <Objbase.h>
#else

#include "comdef.hpp"
#include "datatypes.hpp"
#include "error_codes.hpp"
#include "guiddef.hpp"

intercom::HRESULT DllGetClassObject(
  intercom::REFCLSID rclsid,
  intercom::REFIID riid,
  void** ppv
);

#endif

#endif
