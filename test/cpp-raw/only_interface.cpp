
#include "../cpp-utility/os.hpp"
#include "../dependencies/catch.hpp"

#include "testlib.hpp"

TEST_CASE( "only_interface" )
{
    // This isn't really a runtime test.
    // If the types are not exported, this should fail to compile at all.
    SECTION( "Types are exported" )
    {
        IOnlyInterface_Automation* pAutomation = nullptr;
        IOnlyInterface_Raw* pRaw = nullptr;
    }

    // We could just as well use this to ensure the auto-generated guids don't
    // change.
    SECTION( "GUIDs remain constant" )
    {
        intercom::IID automation = { 0xfe0b0f3a, 0x7710, 0x3e69, 0x48, 0x6b, 0xd6, 0x01, 0x6b, 0xd8, 0xf8, 0x9d };
        intercom::IID raw = { 0xe7de960d, 0x26be, 0x31fa, 0x5c, 0x98, 0x63, 0x0e, 0x66, 0xcc, 0x22, 0x7d };
        REQUIRE( IID_IOnlyInterface_Automation == automation );
        REQUIRE( IID_IOnlyInterface_Raw == raw );
    }
}
