
#ifndef INTERCOM_CPP_DETAIL_DLWRAPPER_H
#define INTERCOM_CPP_DETAIL_DLWRAPPER_H

#ifdef _MSC_VER
#include "../msvc/dlwrapper.hpp"
namespace intercom { namespace detail { using DlWrapper = intercom::msvc::DlWrapper; } }
#else

#include "../posix/dlwrapper.hpp"
namespace intercom { namespace detail { using DlWrapper = intercom::posix::DlWrapper; } }

#endif

#endif
