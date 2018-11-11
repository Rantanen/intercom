
// Includes declarations associated with COM.

// Use predefined set if available.
#ifdef _MSC_VER
#include<Unknwn.h>
#include<oaidl.h>
#else

#include "posix/iunknown.hpp"
#include "posix/idispatch.hpp"

#endif
