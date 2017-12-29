
#ifndef INTERCOM_CPP_ERRORCODES_H
#define INTERCOM_CPP_ERRORCODES_H

// Use predefined set if available.
#ifdef _MSC_VER
#include<Winerror.h>
#else

// TODO: Implement proper mechanism for constructing HRESULT based errors.
// The values have internal structure, they are not just simplistic numbers.

#include <inttypes.h>

// Success.
#define S_OK 0

// Not implemented.
#define E_NOTIMPL 0x80004001

// Not implemented.
#define E_INVALIDARG 0x80070057

// 	Unspecified failure.
#define E_FAIL 0x80004005

#endif
#endif
