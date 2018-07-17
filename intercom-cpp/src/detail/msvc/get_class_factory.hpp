
#ifndef INTERCOM_CPP_DETAIL_MSVC_GETCLASSFACTORY_H
#define INTERCOM_CPP_DETAIL_MSVC_GETCLASSFACTORY_H


#include <Objbase.h>

#include "../../guiddef.h"
#include "../../error_codes.h"
#include "../declarations.hpp"
#include "../../raw_interface.h"
#include "../../runtime_error.h"

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
}
}

#endif
