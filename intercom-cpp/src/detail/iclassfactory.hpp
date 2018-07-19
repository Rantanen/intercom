

#ifndef INTERCOM_CPP_ICLASSFACTORY_H
#define INTERCOM_CPP_ICLASSFACTORY_H

#ifdef _MSC_VER
#include <Unknwn.h>

namespace intercom { using IClassFactory = ::IClassFactory; }

#else
#include "posix/iclassfactory.h"
#endif


#endif