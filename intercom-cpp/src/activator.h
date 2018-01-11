
#ifndef INTERCOM_CPP_ACTIVATOR_H
#define INTERCOM_CPP_ACTIVATOR_H

#include <stdexcept>

#include "posix/iclassfactory.h"
#include "posix/dlwrapper.h"

using intercom::cpp::posix::DlWrapper;

#include "cominterop.h"
#include "raw_interface.h"

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
        m_library( name, DlWrapper::rtld::lazy ),
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
        if( error != S_OK )
        {
            throw std::runtime_error( "Could not create instance" );
        }
        return interface;
    }

    intercom::HRESULT create(
        intercom::REFIID riid,
        void** interface
    )
    {
        return m_classFactory->CreateInstance( nullptr, riid, (void**) interface );
    }
private:

    void init_class_factory()
    {
        intercom::HRESULT error = m_getClassObjectFunc( m_classId, IID_IClassFactory,
                (void**) &m_classFactory );
        if( error != S_OK )
            throw std::runtime_error( "Could not acquire class factory" );
    }

private:

    intercom::CLSID m_classId;
    DlWrapper m_library;
    GetClassObjectFunc m_getClassObjectFunc;
    IClassFactory* m_classFactory;

};

}


#endif
