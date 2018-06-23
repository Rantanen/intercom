
#include "../cpp-utility/os.h"
#include "../cpp-utility/catch.hpp"

#include "libraries.h"
#include "generated/test_lib.h"

#include <intercom.h>
#include <src/detail/dlwrapper.h>

#include <unordered_set>

TEST_CASE( "IntercomListClassObjects" )
{
    size_t total_class_count = 0;
    std::unordered_set< intercom::CLSID, intercom::hash< intercom::CLSID > > unique_classes;

    // Testlib
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
        total_class_count += class_count;

        // Ensure correct classes were found.
        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( class_count ==  test_lib::Descriptor::CLASSES.size() );
        for( size_t c = 0; c < class_count; ++c )
        {
            REQUIRE( std::find(
                    test_lib::Descriptor::CLASSES.begin(), test_lib::Descriptor::CLASSES.end(), classes[ c ] ) != test_lib::Descriptor::CLASSES.end() );

            unique_classes.insert( classes[ c ] );
        }
    }

    // Multilib
    {
        intercom::detail::DlWrapper library(
                multi_lib::Descriptor::NAME,
                intercom::detail::DlWrapper::rtld::lazy );

        // Fetch the class ids of classes that are creatable in multi_lib.
        intercom::Activator::IntercomListClassObjectsFunc listClassObjectsFunc =
                library.load_function< intercom::Activator::IntercomListClassObjectsFunc >( "IntercomListClassObjects" );
        REQUIRE( listClassObjectsFunc != nullptr );
        size_t class_count = 0;
        intercom::CLSID* classes = nullptr;
        intercom::HRESULT hr = listClassObjectsFunc( &class_count, &classes );
        total_class_count += class_count;

        // Ensure correct classes were found.
        REQUIRE( hr == intercom::SC_OK );
        REQUIRE( class_count ==  multi_lib::Descriptor::CLASSES.size() );
        for( size_t c = 0; c < class_count; ++c )
        {
            REQUIRE( std::find(
                    multi_lib::Descriptor::CLASSES.begin(), multi_lib::Descriptor::CLASSES.end(), classes[ c ] ) != multi_lib::Descriptor::CLASSES.end() );

            unique_classes.insert( classes[ c ] );
        }
    }

    REQUIRE( total_class_count == unique_classes.size() );

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