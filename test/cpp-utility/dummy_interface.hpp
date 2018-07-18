
#ifndef CPPRAW_DUMMYINTERFACE_H
#define CPPRAW_DUMMYINTERFACE_H

#include <array>
#include <intercom.hpp>

namespace cppraw
{
namespace utility
{

class DummyLibDescriptor
{
public:
    static const char NAME[];
    static const char WINDOWS_NAME[];
    static const char POSIX_NAME[];
};

/**
 * @brief An interface which does not exist in the library.
 *
 */
struct IDummyInterface : IUnknown
{
    static const intercom::IID ID;
};

class DummyInterfaceDescriptor
{
public:
    static const intercom::CLSID ID;

    static const std::array<intercom::IID, 1> INTERFACES;

    using Library = DummyLibDescriptor;

    DummyInterfaceDescriptor() = delete;
    ~DummyInterfaceDescriptor() = delete;
};

}
}

#endif
