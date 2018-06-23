
#include "../cpp-utility/os.h"
#include "../cpp-utility/catch.hpp"

#include "testlib.h"
#include "generated/test_lib.h"

#include <intercom.h>
#include <src/detail/dlwrapper.h>

TEST_CASE( "IntercomListClassObjects" )
{

    SECTION( "TestLib" )
    {
        intercom::detail::DlWrapper library(
                test_lib::Descriptor::NAME,
                intercom::detail::DlWrapper::rtld::lazy );


        // Fetch the class ids of classes that are creatable in test_lib.
        intercom::Activator::IntercomListClassObjectsFunc listClassObjectsFunc =
                library.load_function< intercom::Activator::IntercomListClassObjectsFunc >( "IntercomListClassObjects" );
        REQUIRE( listClassObjectsFunc != nullptr );
        size_t class_count = 0;
        intercom::CLSID* classes = nullptr;
        intercom::HRESULT hr = listClassObjectsFunc( &class_count, &classes );

        // Ensure correct classes were found.
        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( class_count ==  test_lib::Descriptor::CLASSES.size() );
        for( size_t c = 0; c < class_count; ++c )
        {
            REQUIRE( std::find(
                    test_lib::Descriptor::CLASSES.begin(), test_lib::Descriptor::CLASSES.end(), classes[ c ] ) != test_lib::Descriptor::CLASSES.end() );
        }
    }

    SECTION( "Invalid parameters" )
    {
        intercom::detail::DlWrapper library(
                test_lib::Descriptor::NAME,
                intercom::detail::DlWrapper::rtld::lazy );
        intercom::Activator::IntercomListClassObjectsFunc listClassObjectsFunc =
                library.load_function< intercom::Activator::IntercomListClassObjectsFunc >( "IntercomListClassObjects" );
        REQUIRE( listClassObjectsFunc != nullptr );

        {
            intercom::CLSID* classes = nullptr;
            intercom::HRESULT hr = listClassObjectsFunc( nullptr, &classes );
            REQUIRE( hr == intercom::EC_POINTER );
        }

        {
            size_t class_count = 0;
            intercom::HRESULT hr = listClassObjectsFunc( &class_count, nullptr );
            REQUIRE( hr == intercom::EC_POINTER );
        }

    }
}