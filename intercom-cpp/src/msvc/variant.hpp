
#ifndef INTERCOM_CPP_MSVC_VARIANT_H
#define INTERCOM_CPP_MSVC_VARIANT_H

#include <guiddef.h>


// The generated C++ headers and classes expect the VARIANT in intercom namespace.
namespace intercom
{
    typedef ::VARIANT VARIANT;
}

#endif
