
#ifndef INTERCOM_LIBRARY_test_lib_H
#define INTERCOM_LIBRARY_test_lib_H

#include <array>
#include <intercom.h>

namespace test_lib
{
    class Descriptor
    {
    public:
        static const char NAME[];
        static const char WINDOWS_NAME[];
        static const char POSIX_NAME[];
    };

namespace raw
{
    struct Foo;
    struct IAllocator;
    struct Foo : IUnknown
    {
        static const intercom::IID ID;
        virtual uint32_t INTERCOM_CC Method(uint32_t a) = 0;
    };
    struct IAllocator : IUnknown
    {
        static const intercom::IID ID;
        virtual intercom::BSTR INTERCOM_CC AllocBstr(intercom::BSTR text, uint32_t len) = 0;
        virtual void INTERCOM_CC FreeBstr(intercom::BSTR bstr) = 0;
        virtual void* INTERCOM_CC Alloc(size_t len) = 0;
        virtual void INTERCOM_CC Free(void* ptr) = 0;
    };
    class AllocatorDescriptor
    {
    public:
        static const intercom::CLSID ID;

        static const std::array<intercom::IID, 1> INTERFACES;

        using Library = test_lib::Descriptor;

        AllocatorDescriptor() = delete;
        ~AllocatorDescriptor() = delete;
    };

}
}

#ifdef INTERCOM_FLATTEN_DECLARATIONS
    static constexpr intercom::IID IID_Foo = {0x4cb2a593,0xc19f,0x320b,{0x62,0xb2,0x03,0xed,0x6a,0x7c,0x33,0xf0}};
    using Foo = test_lib::raw::Foo;
    static constexpr intercom::IID IID_IAllocator = {0x18ee22b3,0xb0c6,0x44a5,{0xa9,0x4a,0x7a,0x41,0x76,0x76,0xfb,0x66}};
    using IAllocator = test_lib::raw::IAllocator;
    static constexpr intercom::CLSID CLSID_Allocator = {0xdf3c35c1,0xcdd2,0x3b15,{0x6a,0x24,0xa7,0xe9,0xd6,0xb3,0xdd,0xf0}};
#endif

#endif
