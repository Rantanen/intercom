
#include "test_lib.h"

#ifdef _MSC_VER
    const char test_lib::Descriptor::NAME[] = "test_lib.dll";
#else
    const char test_lib::Descriptor::NAME[] = "libtest_lib.so";
#endif

const char test_lib::Descriptor::WINDOWS_NAME[] = "test_lib.dll";
const char test_lib::Descriptor::POSIX_NAME[] = "libtest_lib.so";
