
#ifndef INTERCOM_CPP_ERRORCODES_H
#define INTERCOM_CPP_ERRORCODES_H

// Use predefined set if available.
#ifdef _MSC_VER
#include<Winerror.h>
#else

#include "posix/error_codes.h"

#endif
#endif
