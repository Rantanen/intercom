/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0622 */
/* at Tue Jan 19 05:14:07 2038
 */
/* Compiler settings for ..\testlib\TestLib.idl:
    Oicf, W1, Zp8, env=Win64 (32b run), target_arch=AMD64 8.01.0622
    protocol : dce , ms_ext, c_ext, robust
    error checks: allocation ref bounds_check enum stub_data
    VC __declspec() decoration level:
         __declspec(uuid()), __declspec(selectany), __declspec(novtable)
         DECLSPEC_UUID(), MIDL_INTERFACE()
*/
/* @@MIDL_FILE_HEADING(  ) */

#include "../../../intercom-cpp/src/comdef.h"
#include "../../../intercom-cpp/src/guiddef.h"
#include "../../../intercom-cpp/src/data_types.h"
#include "../../../intercom-cpp/src/msdef.h"


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif


#ifndef __TestLib_h_h__
#define __TestLib_h_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

/* Forward Declarations */

#ifndef __IRefCount_FWD_DEFINED__
#define __IRefCount_FWD_DEFINED__
typedef interface IRefCount IRefCount;

#endif 	/* __IRefCount_FWD_DEFINED__ */


#ifndef __IPrimitiveOperations_FWD_DEFINED__
#define __IPrimitiveOperations_FWD_DEFINED__
typedef interface IPrimitiveOperations IPrimitiveOperations;

#endif 	/* __IPrimitiveOperations_FWD_DEFINED__ */


#ifndef __IStatefulOperations_FWD_DEFINED__
#define __IStatefulOperations_FWD_DEFINED__
typedef interface IStatefulOperations IStatefulOperations;

#endif 	/* __IStatefulOperations_FWD_DEFINED__ */


#ifndef __IResultOperations_FWD_DEFINED__
#define __IResultOperations_FWD_DEFINED__
typedef interface IResultOperations IResultOperations;

#endif 	/* __IResultOperations_FWD_DEFINED__ */


#ifndef __IClassCreator_FWD_DEFINED__
#define __IClassCreator_FWD_DEFINED__
typedef interface IClassCreator IClassCreator;

#endif 	/* __IClassCreator_FWD_DEFINED__ */


#ifndef __ICreatedClass_FWD_DEFINED__
#define __ICreatedClass_FWD_DEFINED__
typedef interface ICreatedClass ICreatedClass;

#endif 	/* __ICreatedClass_FWD_DEFINED__ */


#ifndef __IParent_FWD_DEFINED__
#define __IParent_FWD_DEFINED__
typedef interface IParent IParent;

#endif 	/* __IParent_FWD_DEFINED__ */


#ifndef __IRefCountOperations_FWD_DEFINED__
#define __IRefCountOperations_FWD_DEFINED__
typedef interface IRefCountOperations IRefCountOperations;

#endif 	/* __IRefCountOperations_FWD_DEFINED__ */


#ifndef __ISharedInterface_FWD_DEFINED__
#define __ISharedInterface_FWD_DEFINED__
typedef interface ISharedInterface ISharedInterface;

#endif 	/* __ISharedInterface_FWD_DEFINED__ */


#ifndef __IErrorSource_FWD_DEFINED__
#define __IErrorSource_FWD_DEFINED__
typedef interface IErrorSource IErrorSource;

#endif 	/* __IErrorSource_FWD_DEFINED__ */


#ifndef __RefCountOperations_FWD_DEFINED__
#define __RefCountOperations_FWD_DEFINED__

#ifdef __cplusplus
typedef class RefCountOperations RefCountOperations;
#else
typedef struct RefCountOperations RefCountOperations;
#endif /* __cplusplus */

#endif 	/* __RefCountOperations_FWD_DEFINED__ */


#ifndef __PrimitiveOperations_FWD_DEFINED__
#define __PrimitiveOperations_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrimitiveOperations PrimitiveOperations;
#else
typedef struct PrimitiveOperations PrimitiveOperations;
#endif /* __cplusplus */

#endif 	/* __PrimitiveOperations_FWD_DEFINED__ */


#ifndef __StatefulOperations_FWD_DEFINED__
#define __StatefulOperations_FWD_DEFINED__

#ifdef __cplusplus
typedef class StatefulOperations StatefulOperations;
#else
typedef struct StatefulOperations StatefulOperations;
#endif /* __cplusplus */

#endif 	/* __StatefulOperations_FWD_DEFINED__ */


#ifndef __ResultOperations_FWD_DEFINED__
#define __ResultOperations_FWD_DEFINED__

#ifdef __cplusplus
typedef class ResultOperations ResultOperations;
#else
typedef struct ResultOperations ResultOperations;
#endif /* __cplusplus */

#endif 	/* __ResultOperations_FWD_DEFINED__ */


#ifndef __ClassCreator_FWD_DEFINED__
#define __ClassCreator_FWD_DEFINED__

#ifdef __cplusplus
typedef class ClassCreator ClassCreator;
#else
typedef struct ClassCreator ClassCreator;
#endif /* __cplusplus */

#endif 	/* __ClassCreator_FWD_DEFINED__ */


#ifndef __CreatedClass_FWD_DEFINED__
#define __CreatedClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class CreatedClass CreatedClass;
#else
typedef struct CreatedClass CreatedClass;
#endif /* __cplusplus */

#endif 	/* __CreatedClass_FWD_DEFINED__ */


#ifndef __SharedImplementation_FWD_DEFINED__
#define __SharedImplementation_FWD_DEFINED__

#ifdef __cplusplus
typedef class SharedImplementation SharedImplementation;
#else
typedef struct SharedImplementation SharedImplementation;
#endif /* __cplusplus */

#endif 	/* __SharedImplementation_FWD_DEFINED__ */


#ifndef __ErrorSource_FWD_DEFINED__
#define __ErrorSource_FWD_DEFINED__

#ifdef __cplusplus
typedef class ErrorSource ErrorSource;
#else
typedef struct ErrorSource ErrorSource;
#endif /* __cplusplus */

#endif 	/* __ErrorSource_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif



#ifndef __TestLib_LIBRARY_DEFINED__
#define __TestLib_LIBRARY_DEFINED__

/* library TestLib */
/* [uuid] */












EXTERN_C const IID LIBID_TestLib;

#ifndef __IRefCount_INTERFACE_DEFINED__
#define __IRefCount_INTERFACE_DEFINED__

/* interface IRefCount */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IRefCount;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("aa5b7352-5d7a-35b9-5206-145b041f2c1c")
    IRefCount : public IUnknown
    {
    public:
        virtual /* [id] */ UINT32 STDMETHODCALLTYPE GetRefCount( void) = 0;

    };


#else 	/* C style interface */

    typedef struct IRefCountVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IRefCount * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IRefCount * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IRefCount * This);

        /* [id] */ UINT32 ( STDMETHODCALLTYPE *GetRefCount )(
            IRefCount * This);

        END_INTERFACE
    } IRefCountVtbl;

    interface IRefCount
    {
        CONST_VTBL struct IRefCountVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IRefCount_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IRefCount_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IRefCount_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IRefCount_GetRefCount(This)	\
    ( (This)->lpVtbl -> GetRefCount(This) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRefCount_INTERFACE_DEFINED__ */


#ifndef __IPrimitiveOperations_INTERFACE_DEFINED__
#define __IPrimitiveOperations_INTERFACE_DEFINED__

/* interface IPrimitiveOperations */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IPrimitiveOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("12341234-1234-1234-1234-123412340002")
    IPrimitiveOperations : public IUnknown
    {
    public:
        virtual /* [id] */ INT8 STDMETHODCALLTYPE I8(
            /* [in] */ INT8 v) = 0;

        virtual /* [id] */ UINT8 STDMETHODCALLTYPE U8(
            /* [in] */ UINT8 v) = 0;

        virtual /* [id] */ UINT16 STDMETHODCALLTYPE U16(
            /* [in] */ UINT16 v) = 0;

        virtual /* [id] */ INT16 STDMETHODCALLTYPE I16(
            /* [in] */ INT16 v) = 0;

        virtual /* [id] */ INT32 STDMETHODCALLTYPE I32(
            /* [in] */ INT32 v) = 0;

        virtual /* [id] */ UINT32 STDMETHODCALLTYPE U32(
            /* [in] */ UINT32 v) = 0;

        virtual /* [id] */ INT64 STDMETHODCALLTYPE I64(
            /* [in] */ INT64 v) = 0;

        virtual /* [id] */ UINT64 STDMETHODCALLTYPE U64(
            /* [in] */ UINT64 v) = 0;

        virtual /* [id] */ double STDMETHODCALLTYPE F64(
            /* [in] */ double v) = 0;

        virtual /* [id] */ float STDMETHODCALLTYPE F32(
            /* [in] */ float v) = 0;

    };


#else 	/* C style interface */

    typedef struct IPrimitiveOperationsVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IPrimitiveOperations * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IPrimitiveOperations * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IPrimitiveOperations * This);

        /* [id] */ INT8 ( STDMETHODCALLTYPE *I8 )(
            IPrimitiveOperations * This,
            /* [in] */ INT8 v);

        /* [id] */ UINT8 ( STDMETHODCALLTYPE *U8 )(
            IPrimitiveOperations * This,
            /* [in] */ UINT8 v);

        /* [id] */ UINT16 ( STDMETHODCALLTYPE *U16 )(
            IPrimitiveOperations * This,
            /* [in] */ UINT16 v);

        /* [id] */ INT16 ( STDMETHODCALLTYPE *I16 )(
            IPrimitiveOperations * This,
            /* [in] */ INT16 v);

        /* [id] */ INT32 ( STDMETHODCALLTYPE *I32 )(
            IPrimitiveOperations * This,
            /* [in] */ INT32 v);

        /* [id] */ UINT32 ( STDMETHODCALLTYPE *U32 )(
            IPrimitiveOperations * This,
            /* [in] */ UINT32 v);

        /* [id] */ INT64 ( STDMETHODCALLTYPE *I64 )(
            IPrimitiveOperations * This,
            /* [in] */ INT64 v);

        /* [id] */ UINT64 ( STDMETHODCALLTYPE *U64 )(
            IPrimitiveOperations * This,
            /* [in] */ UINT64 v);

        /* [id] */ double ( STDMETHODCALLTYPE *F64 )(
            IPrimitiveOperations * This,
            /* [in] */ double v);

        /* [id] */ float ( STDMETHODCALLTYPE *F32 )(
            IPrimitiveOperations * This,
            /* [in] */ float v);

        END_INTERFACE
    } IPrimitiveOperationsVtbl;

    interface IPrimitiveOperations
    {
        CONST_VTBL struct IPrimitiveOperationsVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IPrimitiveOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IPrimitiveOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IPrimitiveOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IPrimitiveOperations_I8(This,v)	\
    ( (This)->lpVtbl -> I8(This,v) )

#define IPrimitiveOperations_U8(This,v)	\
    ( (This)->lpVtbl -> U8(This,v) )

#define IPrimitiveOperations_U16(This,v)	\
    ( (This)->lpVtbl -> U16(This,v) )

#define IPrimitiveOperations_I16(This,v)	\
    ( (This)->lpVtbl -> I16(This,v) )

#define IPrimitiveOperations_I32(This,v)	\
    ( (This)->lpVtbl -> I32(This,v) )

#define IPrimitiveOperations_U32(This,v)	\
    ( (This)->lpVtbl -> U32(This,v) )

#define IPrimitiveOperations_I64(This,v)	\
    ( (This)->lpVtbl -> I64(This,v) )

#define IPrimitiveOperations_U64(This,v)	\
    ( (This)->lpVtbl -> U64(This,v) )

#define IPrimitiveOperations_F64(This,v)	\
    ( (This)->lpVtbl -> F64(This,v) )

#define IPrimitiveOperations_F32(This,v)	\
    ( (This)->lpVtbl -> F32(This,v) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrimitiveOperations_INTERFACE_DEFINED__ */


#ifndef __IStatefulOperations_INTERFACE_DEFINED__
#define __IStatefulOperations_INTERFACE_DEFINED__

/* interface IStatefulOperations */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IStatefulOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("2b9bddd2-31f5-3d4b-79a0-ac8e8d11a93e")
    IStatefulOperations : public IUnknown
    {
    public:
        virtual /* [id] */ void STDMETHODCALLTYPE PutValue(
            /* [in] */ INT32 v) = 0;

        virtual /* [id] */ INT32 STDMETHODCALLTYPE GetValue( void) = 0;

    };


#else 	/* C style interface */

    typedef struct IStatefulOperationsVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IStatefulOperations * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IStatefulOperations * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IStatefulOperations * This);

        /* [id] */ void ( STDMETHODCALLTYPE *PutValue )(
            IStatefulOperations * This,
            /* [in] */ INT32 v);

        /* [id] */ INT32 ( STDMETHODCALLTYPE *GetValue )(
            IStatefulOperations * This);

        END_INTERFACE
    } IStatefulOperationsVtbl;

    interface IStatefulOperations
    {
        CONST_VTBL struct IStatefulOperationsVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IStatefulOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IStatefulOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IStatefulOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IStatefulOperations_PutValue(This,v)	\
    ( (This)->lpVtbl -> PutValue(This,v) )

#define IStatefulOperations_GetValue(This)	\
    ( (This)->lpVtbl -> GetValue(This) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStatefulOperations_INTERFACE_DEFINED__ */


#ifndef __IResultOperations_INTERFACE_DEFINED__
#define __IResultOperations_INTERFACE_DEFINED__

/* interface IResultOperations */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IResultOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("ffb673d9-7896-3a4c-4fa8-f72406588aa1")
    IResultOperations : public IUnknown
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SOk( void) = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE NotImpl( void) = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Sqrt(
            /* [in] */ double value,
            /* [retval][out] */ double *__out) = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Tuple(
            /* [in] */ UINT32 value,
            /* [out] */ UINT16 *__out1,
            /* [out] */ UINT16 *__out2) = 0;

    };


#else 	/* C style interface */

    typedef struct IResultOperationsVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IResultOperations * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IResultOperations * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IResultOperations * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SOk )(
            IResultOperations * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NotImpl )(
            IResultOperations * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Sqrt )(
            IResultOperations * This,
            /* [in] */ double value,
            /* [retval][out] */ double *__out);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Tuple )(
            IResultOperations * This,
            /* [in] */ UINT32 value,
            /* [out] */ UINT16 *__out1,
            /* [out] */ UINT16 *__out2);

        END_INTERFACE
    } IResultOperationsVtbl;

    interface IResultOperations
    {
        CONST_VTBL struct IResultOperationsVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IResultOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IResultOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IResultOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IResultOperations_SOk(This)	\
    ( (This)->lpVtbl -> SOk(This) )

#define IResultOperations_NotImpl(This)	\
    ( (This)->lpVtbl -> NotImpl(This) )

#define IResultOperations_Sqrt(This,value,__out)	\
    ( (This)->lpVtbl -> Sqrt(This,value,__out) )

#define IResultOperations_Tuple(This,value,__out1,__out2)	\
    ( (This)->lpVtbl -> Tuple(This,value,__out1,__out2) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResultOperations_INTERFACE_DEFINED__ */


#ifndef __IClassCreator_INTERFACE_DEFINED__
#define __IClassCreator_INTERFACE_DEFINED__

/* interface IClassCreator */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IClassCreator;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("2e7e23e8-f66d-3779-6c74-61898d7b40cd")
    IClassCreator : public IUnknown
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateRoot(
            /* [in] */ INT32 id,
            /* [retval][out] */ ICreatedClass **__out) = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CreateChild(
            /* [in] */ INT32 id,
            /* [in] */ IParent *parent,
            /* [retval][out] */ ICreatedClass **__out) = 0;

    };


#else 	/* C style interface */

    typedef struct IClassCreatorVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IClassCreator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IClassCreator * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IClassCreator * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateRoot )(
            IClassCreator * This,
            /* [in] */ INT32 id,
            /* [retval][out] */ ICreatedClass **__out);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CreateChild )(
            IClassCreator * This,
            /* [in] */ INT32 id,
            /* [in] */ IParent *parent,
            /* [retval][out] */ ICreatedClass **__out);

        END_INTERFACE
    } IClassCreatorVtbl;

    interface IClassCreator
    {
        CONST_VTBL struct IClassCreatorVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IClassCreator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IClassCreator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IClassCreator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IClassCreator_CreateRoot(This,id,__out)	\
    ( (This)->lpVtbl -> CreateRoot(This,id,__out) )

#define IClassCreator_CreateChild(This,id,parent,__out)	\
    ( (This)->lpVtbl -> CreateChild(This,id,parent,__out) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IClassCreator_INTERFACE_DEFINED__ */


#ifndef __ICreatedClass_INTERFACE_DEFINED__
#define __ICreatedClass_INTERFACE_DEFINED__

/* interface ICreatedClass */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_ICreatedClass;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("104eb174-fd00-3ecf-7e0d-d965564279e7")
    ICreatedClass : public IUnknown
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetId(
            /* [retval][out] */ INT32 *__out) = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetParentId(
            /* [retval][out] */ INT32 *__out) = 0;

    };


#else 	/* C style interface */

    typedef struct ICreatedClassVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            ICreatedClass * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            ICreatedClass * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            ICreatedClass * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetId )(
            ICreatedClass * This,
            /* [retval][out] */ INT32 *__out);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetParentId )(
            ICreatedClass * This,
            /* [retval][out] */ INT32 *__out);

        END_INTERFACE
    } ICreatedClassVtbl;

    interface ICreatedClass
    {
        CONST_VTBL struct ICreatedClassVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define ICreatedClass_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define ICreatedClass_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define ICreatedClass_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define ICreatedClass_GetId(This,__out)	\
    ( (This)->lpVtbl -> GetId(This,__out) )

#define ICreatedClass_GetParentId(This,__out)	\
    ( (This)->lpVtbl -> GetParentId(This,__out) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreatedClass_INTERFACE_DEFINED__ */


#ifndef __IParent_INTERFACE_DEFINED__
#define __IParent_INTERFACE_DEFINED__

/* interface IParent */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IParent;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("cea1c199-bf71-3b0a-5a4c-ee3f5a0ae5ce")
    IParent : public IUnknown
    {
    public:
        virtual /* [id] */ INT32 STDMETHODCALLTYPE GetId( void) = 0;

    };


#else 	/* C style interface */

    typedef struct IParentVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IParent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IParent * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IParent * This);

        /* [id] */ INT32 ( STDMETHODCALLTYPE *GetId )(
            IParent * This);

        END_INTERFACE
    } IParentVtbl;

    interface IParent
    {
        CONST_VTBL struct IParentVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IParent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IParent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IParent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IParent_GetId(This)	\
    ( (This)->lpVtbl -> GetId(This) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IParent_INTERFACE_DEFINED__ */


#ifndef __IRefCountOperations_INTERFACE_DEFINED__
#define __IRefCountOperations_INTERFACE_DEFINED__

/* interface IRefCountOperations */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IRefCountOperations;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("6b198a07-2d86-340e-717e-f416a3905d6e")
    IRefCountOperations : public IUnknown
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetNew(
            /* [retval][out] */ IRefCountOperations **__out) = 0;

        virtual /* [id] */ UINT32 STDMETHODCALLTYPE GetRefCount( void) = 0;

    };


#else 	/* C style interface */

    typedef struct IRefCountOperationsVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IRefCountOperations * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IRefCountOperations * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IRefCountOperations * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetNew )(
            IRefCountOperations * This,
            /* [retval][out] */ IRefCountOperations **__out);

        /* [id] */ UINT32 ( STDMETHODCALLTYPE *GetRefCount )(
            IRefCountOperations * This);

        END_INTERFACE
    } IRefCountOperationsVtbl;

    interface IRefCountOperations
    {
        CONST_VTBL struct IRefCountOperationsVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IRefCountOperations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IRefCountOperations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IRefCountOperations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IRefCountOperations_GetNew(This,__out)	\
    ( (This)->lpVtbl -> GetNew(This,__out) )

#define IRefCountOperations_GetRefCount(This)	\
    ( (This)->lpVtbl -> GetRefCount(This) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRefCountOperations_INTERFACE_DEFINED__ */


#ifndef __ISharedInterface_INTERFACE_DEFINED__
#define __ISharedInterface_INTERFACE_DEFINED__

/* interface ISharedInterface */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_ISharedInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("1df08ff6-aafb-37ec-53cf-cde249ceeb4b")
    ISharedInterface : public IUnknown
    {
    public:
        virtual /* [id] */ UINT32 STDMETHODCALLTYPE GetValue( void) = 0;

        virtual /* [id] */ void STDMETHODCALLTYPE SetValue(
            /* [in] */ UINT32 v) = 0;

        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DivideBy(
            /* [in] */ ISharedInterface *divisor,
            /* [retval][out] */ UINT32 *__out) = 0;

    };


#else 	/* C style interface */

    typedef struct ISharedInterfaceVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            ISharedInterface * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            ISharedInterface * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            ISharedInterface * This);

        /* [id] */ UINT32 ( STDMETHODCALLTYPE *GetValue )(
            ISharedInterface * This);

        /* [id] */ void ( STDMETHODCALLTYPE *SetValue )(
            ISharedInterface * This,
            /* [in] */ UINT32 v);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DivideBy )(
            ISharedInterface * This,
            /* [in] */ ISharedInterface *divisor,
            /* [retval][out] */ UINT32 *__out);

        END_INTERFACE
    } ISharedInterfaceVtbl;

    interface ISharedInterface
    {
        CONST_VTBL struct ISharedInterfaceVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define ISharedInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define ISharedInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define ISharedInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define ISharedInterface_GetValue(This)	\
    ( (This)->lpVtbl -> GetValue(This) )

#define ISharedInterface_SetValue(This,v)	\
    ( (This)->lpVtbl -> SetValue(This,v) )

#define ISharedInterface_DivideBy(This,divisor,__out)	\
    ( (This)->lpVtbl -> DivideBy(This,divisor,__out) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISharedInterface_INTERFACE_DEFINED__ */


#ifndef __IErrorSource_INTERFACE_DEFINED__
#define __IErrorSource_INTERFACE_DEFINED__

/* interface IErrorSource */
/* [unique][nonextensible][uuid][object] */


EXTERN_C const IID IID_IErrorSource;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("5505b7b6-5ca4-3e38-667b-a9823f1d5a0f")
    IErrorSource : public IUnknown
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE StoreError(
            /* [in] */ HRESULT hr,
            /* [in] */ BSTR desc) = 0;

    };


#else 	/* C style interface */

    typedef struct IErrorSourceVtbl
    {
        BEGIN_INTERFACE

        HRESULT ( STDMETHODCALLTYPE *QueryInterface )(
            IErrorSource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */
            _COM_Outptr_  void **ppvObject);

        ULONG ( STDMETHODCALLTYPE *AddRef )(
            IErrorSource * This);

        ULONG ( STDMETHODCALLTYPE *Release )(
            IErrorSource * This);

        /* [id] */ HRESULT ( STDMETHODCALLTYPE *StoreError )(
            IErrorSource * This,
            /* [in] */ HRESULT hr,
            /* [in] */ BSTR desc);

        END_INTERFACE
    } IErrorSourceVtbl;

    interface IErrorSource
    {
        CONST_VTBL struct IErrorSourceVtbl *lpVtbl;
    };



#ifdef COBJMACROS


#define IErrorSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) )

#define IErrorSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) )

#define IErrorSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) )


#define IErrorSource_StoreError(This,hr,desc)	\
    ( (This)->lpVtbl -> StoreError(This,hr,desc) )

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IErrorSource_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_RefCountOperations;

#ifdef __cplusplus

class DECLSPEC_UUID("f06af5f0-745a-3b29-4839-d2d39a3f08cd")
RefCountOperations;
#endif

EXTERN_C const CLSID CLSID_PrimitiveOperations;

#ifdef __cplusplus

class DECLSPEC_UUID("12341234-1234-1234-1234-123412340001")
PrimitiveOperations;
#endif

EXTERN_C const CLSID CLSID_StatefulOperations;

#ifdef __cplusplus

class DECLSPEC_UUID("694c1893-2fa8-3d4c-6acf-69c59366721e")
StatefulOperations;
#endif

EXTERN_C const CLSID CLSID_ResultOperations;

#ifdef __cplusplus

class DECLSPEC_UUID("e5ce34c4-c1ad-34bc-69f6-d1bfa6bb2596")
ResultOperations;
#endif

EXTERN_C const CLSID CLSID_ClassCreator;

#ifdef __cplusplus

class DECLSPEC_UUID("3323cccd-1a38-33a4-4ae1-4dc92a7e8dc5")
ClassCreator;
#endif

EXTERN_C const CLSID CLSID_CreatedClass;

#ifdef __cplusplus

class DECLSPEC_UUID("51ed4fb8-35d8-36c6-78fd-6bc582c84b19")
CreatedClass;
#endif

EXTERN_C const CLSID CLSID_SharedImplementation;

#ifdef __cplusplus

class DECLSPEC_UUID("88687644-9cb2-3bd6-4c23-db547d399029")
SharedImplementation;
#endif

EXTERN_C const CLSID CLSID_ErrorSource;

#ifdef __cplusplus

class DECLSPEC_UUID("2af563c2-dc1c-339a-6035-f5f8180fae86")
ErrorSource;
#endif
#endif /* __TestLib_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif
