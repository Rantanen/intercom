
// Include declarations required to test "testlib".

#define INTERCOM_FLATTEN_DECLARATIONS
#ifdef _MSC_VER
    #include <Windows.h>
    #import "..\testlib\target\debug\test_lib.dll" raw_interfaces_only named_guids
    using namespace TestLib;
#elif __GNUC__
    #include "../cpp-utility/gcc/test_lib.h"
#else
    #error Architecture not supported
 #endif
