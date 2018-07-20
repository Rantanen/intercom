
#ifndef INTERCOM_CPP_DETAIL_POSIX_GETCLASSFACTORY_H
#define INTERCOM_CPP_DETAIL_POSIX_GETCLASSFACTORY_H


#include <cstring>
#include <iomanip>
#include <sstream>
#include <type_traits>
#include <functional>

#include "../../guiddef.hpp"
#include "../declarations.hpp"
#include "../../raw_interface.hpp"
#include "library_index.hpp"

namespace intercom { struct IClassFactory; }

namespace intercom
{
namespace detail
{

    static std::unique_ptr< LibraryIndex > LIBRARY_INDEX = std::make_unique< LibraryIndex >();

    /**
     * @brief Get the class factory object for a class.
     *
     * @param library_name Identities the library that implements the class.
     * @param class_id Identifies the class
     * @param class_factory Receives the class factory if the function succeeded.
     * @return intercom::HRESULT The error code
     */
    inline intercom::HRESULT get_class_factory(
        const char* library_name,
        const intercom::CLSID& class_id,
        intercom::IClassFactory** class_factory
    ) noexcept
    {
        // The library must implement "DllGetClassObject".
        intercom::posix::DlWrapper library( library_name,
                intercom::posix::DlWrapper::rtld::lazy );
        intercom::detail::GetClassObjectFunc get_class_object;
        if( library.try_load_function< intercom::detail::GetClassObjectFunc >(
                "DllGetClassObject",
                &get_class_object ) == false )
        {
            return intercom::EC_CLASSNOTREG;
        }

        return get_class_object( class_id, intercom::IID_IClassFactory,
                (void**) class_factory );
    }

    /**
     * @brief Get the class factory object for a class.
     *
     * @param class_id Identifies the class
     * @param class_factory Receives the class factory if the function succeeded.
     * @return intercom::HRESULT The error code
     */
    inline intercom::HRESULT get_class_factory(
        const intercom::CLSID& class_id,
        intercom::IClassFactory** class_factory
    ) noexcept
    {
        const char* library_name = LIBRARY_INDEX->find_library( class_id );
        if( library_name == nullptr )
            return intercom::EC_CLASSNOTREG;

        return get_class_factory( library_name, class_id, class_factory );
    }

    /**
     * @brief Get the class factory object for a class.
     *
     * @param library_name Identities the library that implements the class.
     * @param class_id Identifies the class
     * @param class_factory Receives the class factory if the function succeeded.
     * @return intercom::RawInterface< intercom::IClassFactory > Receives the class factory.
     */
    inline intercom::RawInterface< intercom::IClassFactory > get_class_factory(
        const char* library_name,
        const intercom::CLSID& class_id
    )
    {
        intercom::RawInterface< intercom::IClassFactory > class_factory;
        intercom::HRESULT hr = get_class_factory( library_name, class_id, &class_factory );
        if( intercom::failed( hr ) )
            throw intercom::RuntimeError( hr, "Loading class factory failed." );

        return class_factory;
    }

    /**
     * @brief Get the class factory object for a class.
     *
     * @param class_id Identifies the class
     * @param class_factory Receives the class factory if the function succeeded.
     * @return intercom::RawInterface< intercom::IClassFactory > Receives the class factory.
     */
    inline intercom::RawInterface< intercom::IClassFactory > get_class_factory(
        const intercom::CLSID& class_id
    )
    {
        intercom::RawInterface< intercom::IClassFactory > class_factory;
        intercom::HRESULT hr = get_class_factory( class_id, &class_factory );
        if( intercom::failed( hr ) )
            throw intercom::RuntimeError( hr, "Loading class factory failed." );

        return class_factory;
    }

    /**
     * @brief Attempts to register a library for the "intercom".
     *
     * @param library_name The name of the library to register
     * @param expected_classes_begin A pointer to the beginning of an array of classes the library is expected to implement.
     * @param expected_classes_end A pointer to the end of an array of classes the library is expected to implement.
     * @return Returns true if registering the library succeed and all the expected classes are availab.e.
     */
    inline bool try_register_library(
        const char* library_name,
        const intercom::CLSID* expected_classes_begin,
        const intercom::CLSID* expected_classes_end
    )
    {
        return LIBRARY_INDEX->try_register_library( library_name, expected_classes_begin, expected_classes_end );
    }
}
}

#endif
