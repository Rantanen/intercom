
#include "dummy_interface.hpp"

#ifdef _MSC_VER
    const char cppraw::utility::DummyLibDescriptor::NAME[] = "dummy_lib.dll";
#else
    const char cppraw::utility::DummyLibDescriptor::NAME[] = "libdummy_lib.so";
#endif

const char cppraw::utility::DummyLibDescriptor::WINDOWS_NAME[] = "dummy_lib.dll";
const char cppraw::utility::DummyLibDescriptor::POSIX_NAME[] = "libdummy_lib.so";

const intercom::IID cppraw::utility::IDummyInterface::ID = {0x12345678,0x0,0x0,{0x12,0x34,0x56,0x78,0x00,0x00,0x00,0x00}};

const intercom::CLSID cppraw::utility::DummyInterfaceDescriptor::ID = {0x12345678,0x1234,0x1234,{0x12,0x34,0x56,0x78,0x00,0x00,0x00,0x00}};
const std::array<intercom::IID, 1> cppraw::utility::DummyInterfaceDescriptor::INTERFACES = {
            cppraw::utility::IDummyInterface::ID
};
