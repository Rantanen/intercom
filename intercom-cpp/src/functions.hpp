#ifndef INTERCOM_CPP_FUNCTIONS_H
#define INTERCOM_CPP_FUNCTIONS_H

#include "guiddef.h"
#include "datatypes.h"
#include "raw_interface.h"
#include "detail/get_class_factory.hpp"
#include "detail/iclassfactory.hpp"

namespace intercom
{
    /**
     * @brief Create an object of a class.
     *
     * @param class_id Specifies the class to return.
     * @param riid  Specifies the interface to return.
     * @param itf  Receives a pointer to the interface.
     * @return intercom::HRESULT Result.
     */
    inline intercom::HRESULT create_instance(
        const intercom::CLSID& class_id,
        const intercom::IID& riid,
        void** itf
    )
    {
        // Locate factory for the object.
        intercom::RawInterface< intercom::IClassFactory > factory;
        intercom::HRESULT factory_result = intercom::detail::get_class_factory(
                class_id, &factory );
        if( intercom::failed( factory_result  ) )
            return factory_result;

        return factory->CreateInstance( nullptr, riid, itf );
    }

}


#ifdef INTERCOM_FLATTEN_DECLARATIONS

inline intercom::HRESULT CreateInstance( intercom::REFCLSID clsid, intercom::REFIID iid, void** pout )
{
    return intercom::create_instance( clsid, iid, pouc );
}

#endif

#endif
