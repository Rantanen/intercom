
#include "test_lib.h"

#ifdef _MSC_VER
    const char test_lib::Descriptor::NAME[] = "test_lib.dll";
#else
    const char test_lib::Descriptor::NAME[] = "libtest_lib.so";
#endif

const char test_lib::Descriptor::WINDOWS_NAME[] = "test_lib.dll";
const char test_lib::Descriptor::POSIX_NAME[] = "libtest_lib.so";

const intercom::IID test_lib::raw::Foo::ID = {0x4cb2a593,0xc19f,0x320b,{0x62,0xb2,0x03,0xed,0x6a,0x7c,0x33,0xf0}};
const intercom::IID test_lib::raw::IAllocator::ID = {0x18ee22b3,0xb0c6,0x44a5,{0xa9,0x4a,0x7a,0x41,0x76,0x76,0xfb,0x66}};


const intercom::CLSID test_lib::raw::AllocatorDescriptor::ID = {0xdf3c35c1,0xcdd2,0x3b15,{0x6a,0x24,0xa7,0xe9,0xd6,0xb3,0xdd,0xf0}};
const std::array<intercom::IID, 1> test_lib::raw::AllocatorDescriptor::INTERFACES = {
            test_lib::raw::IAllocator::ID
};
