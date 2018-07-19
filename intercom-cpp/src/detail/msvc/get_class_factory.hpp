
#ifndef INTERCOM_CPP_DETAIL_MSVC_GETCLASSFACTORY_H
#define INTERCOM_CPP_DETAIL_MSVC_GETCLASSFACTORY_H


#include <Objbase.h>

#include "../../guiddef.hpp"
#include "../../error_codes.hpp"
#include "../declarations.hpp"
#include "../../raw_interface.hpp"
#include "../../runtime_error.hpp"

namespace intercom { using IClassFactory = ::IClassFactory; }

namespace intercom
{
namespace detail
{
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
        // On Windows we can rely on COM to discover the library implementing the class.
        return ::CoGetClassObject( class_id, CLSCTX_INPROC_SERVER, nullptr,
                IID_IClassFactory, (void**) class_factory );
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
        return get_class_factory( nullptr, class_id, class_factory );
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
        // On Windows platform the COM runtime handles the discovery of the classes.
        // This is a no-op for now.
        // NOTE: We could do a sanity check here to ensure that the classes are actually available on Windows platform.
        // If implemented, it is important to check that acquiring the global "loader lock" does not cause problems
        // when "static" variables are being initialized.
        return true;
    }
}
}

#endif
