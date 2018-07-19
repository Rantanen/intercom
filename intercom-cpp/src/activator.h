
#ifndef INTERCOM_CPP_ACTIVATOR_H
#define INTERCOM_CPP_ACTIVATOR_H

#include <stdexcept>

#include "detail/iclassfactory.hpp"
#include "detail/declarations.hpp"
#include "detail/get_class_factory.hpp"
#include "cominterop.h"
#include "datatypes.h"
#include "error_codes.h"
#include "no_such_interface.h"
#include "raw_interface.h"
#include "runtime_error.h"

namespace intercom
{

class Activator
{
public:

public:

    Activator(
        const char* library_name,
        const intercom::CLSID& classId  //!< Identifies the class constructed with this activator.
    ) :
        m_classId( classId ),
        m_classFactory( intercom::detail::get_class_factory( library_name, classId ) )
    {
    }

    Activator(
        const intercom::CLSID& classId  //!< Identifies the class constructed with this activator.
    ) :
        m_classId( classId ),
        m_classFactory( intercom::detail::get_class_factory( classId ) )
    {
    }

    template< typename TInterface >
    intercom::RawInterface<TInterface> create()
    {
        intercom::RawInterface< TInterface > itf;
        intercom::HRESULT error = m_classFactory->CreateInstance( nullptr, TInterface::ID, itf.out() );
        switch( error )
        {
        case intercom::SC_OK:
            break;

        case intercom::EC_NOINTERFACE:
            throw intercom::NoSuchInterface( m_classId, TInterface::ID );

        // Unspecified error.
        default:
            throw intercom::RuntimeError( error, std::stringstream() << "Creating instance of class \""
                    << m_classId << "\" with interface \"" << TInterface::ID << "\" failed." );
        }
        return itf;
    }

    intercom::HRESULT create(
        const intercom::IID& riid,
        void** itf
    )
    {
        return m_classFactory->CreateInstance( nullptr, riid, (void**) itf );
    }

private:

    intercom::CLSID m_classId;
    intercom::RawInterface< IClassFactory > m_classFactory;

};

}


#endif
