
// Include declarations required to test "testlib".

#define INTERCOM_FLATTEN_DECLARATIONS
#ifdef _MSC_VER
    #include <Windows.h>

    // Importing the DLL duplicates the __oud identifier which causes a warning.
    #import "..\testlib\target\debug\test_lib.dll" raw_interfaces_only named_guids \
        rename("__out", "__out_test_lib")
    using namespace TestLib;

#elif __GNUC__
    #include "../cpp-utility/generated/test_lib.hpp"
#else
    #error Architecture not supported
 #endif
