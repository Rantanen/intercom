
#ifndef INTERCOM_CPP_ACTIVATOR_H
#define INTERCOM_CPP_ACTIVATOR_H

#include <stdexcept>

#include "iclassfactory.h"
#include "detail/dlwrapper.h"
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

    typedef intercom::HRESULT ( *GetClassObjectFunc ) ( REFCLSID, REFIID, void** );

public:

    Activator(
        const char* name,
        const intercom::CLSID& classId  //!< Identifies the class constructed with this activator.
    ) :
        m_classId( classId ),
        m_library( name, intercom::detail::DlWrapper::rtld::lazy ),
        m_getClassObjectFunc( nullptr ),
        m_classFactory( nullptr )
    {
        m_getClassObjectFunc = m_library.load_function< GetClassObjectFunc >( "DllGetClassObject" );

        init_class_factory();
    }

    ~Activator()
    {
        if( m_classFactory != nullptr )
            m_classFactory->Release();
    }

    template< typename TInterface >
    intercom::RawInterface<TInterface> create()
    {
        intercom::RawInterface< TInterface > interface;
        intercom::HRESULT error = m_classFactory->CreateInstance( nullptr, TInterface::ID, interface.out() );
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
        return interface;
    }

    intercom::HRESULT create(
        const intercom::IID& riid,
        void** itf
    )
    {
        return m_classFactory->CreateInstance( nullptr, riid, (void**) itf );
    }
private:

    void init_class_factory()
    {
        intercom::HRESULT error = m_getClassObjectFunc( m_classId, IID_IClassFactory,
                (void**) &m_classFactory );
        if( error != intercom::SC_OK )
        {
            throw intercom::RuntimeError( error, std::stringstream() << "Creating class factory for class \""
                    << m_classId << "\" failed." );
        }
    }

private:

    intercom::CLSID m_classId;
    intercom::detail::DlWrapper m_library;
    GetClassObjectFunc m_getClassObjectFunc;
    IClassFactory* m_classFactory;

};

}


#endif
