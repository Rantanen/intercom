
#ifndef INTERCOM_CPP_ACTIVATOR_H
#define INTERCOM_CPP_ACTIVATOR_H

#include "posix/iclassfactory.h"
#include "posix/dlwrapper.h"

using intercom::cpp::posix::DlWrapper;

#include "cominterop.h"

namespace intercom
{
namespace cpp
{

class Activator
{
public:

    typedef intercom::HRESULT ( *GetClassObjectFunc ) ( REFCLSID, REFIID, void** );

public:

    Activator(
        REFCLSID classId  //!< Identifies the class constructed with this activator.
    ) :
        m_classId( classId ),
        m_library( "./libtest_lib.so",
            DlWrapper::rtld::lazy ),
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

    void create(
        REFIID riid,
        void** ppv
    )
    {
        HRESULT error = m_classFactory->CreateInstance( nullptr, riid, ppv );
        if( error != S_OK )
            throw std::exception();
    }

private:

    void init_class_factory()
    {
        HRESULT error = m_getClassObjectFunc( m_classId, IID_IClassFactory,
                (void**) &m_classFactory );
        if( error != S_OK )
            throw std::exception();
    }

private:

    CLSID m_classId;
    DlWrapper m_library;
    GetClassObjectFunc m_getClassObjectFunc;
    IClassFactory* m_classFactory;

};

}
}

#endif
