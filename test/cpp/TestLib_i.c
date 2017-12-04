

/* this ALWAYS GENERATED file contains the IIDs and CLSIDs */

/* link this file in with the server and any clients */


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

#pragma warning( disable: 4049 )  /* more than 64k source lines */


#ifdef __cplusplus
extern "C"{
#endif 


#include <rpc.h>
#include <rpcndr.h>

#ifdef _MIDL_USE_GUIDDEF_

#ifndef INITGUID
#define INITGUID
#include <guiddef.h>
#undef INITGUID
#else
#include <guiddef.h>
#endif

#define MIDL_DEFINE_GUID(type,name,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) \
        DEFINE_GUID(name,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8)

#else // !_MIDL_USE_GUIDDEF_

#ifndef __IID_DEFINED__
#define __IID_DEFINED__

typedef struct _IID
{
    unsigned long x;
    unsigned short s1;
    unsigned short s2;
    unsigned char  c[8];
} IID;

#endif // __IID_DEFINED__

#ifndef CLSID_DEFINED
#define CLSID_DEFINED
typedef IID CLSID;
#endif // CLSID_DEFINED

#define MIDL_DEFINE_GUID(type,name,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) \
        EXTERN_C __declspec(selectany) const type name = {l,w1,w2,{b1,b2,b3,b4,b5,b6,b7,b8}}

#endif // !_MIDL_USE_GUIDDEF_

MIDL_DEFINE_GUID(IID, LIBID_TestLib,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x00);


MIDL_DEFINE_GUID(IID, IID_IPrimitiveOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x02);


MIDL_DEFINE_GUID(IID, IID_IStatefulOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x04);


MIDL_DEFINE_GUID(IID, IID_IResultOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x06);


MIDL_DEFINE_GUID(IID, IID_IClassCreator,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x09);


MIDL_DEFINE_GUID(IID, IID_ICreatedClass,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x10);


MIDL_DEFINE_GUID(IID, IID_IRefCountOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x12);


MIDL_DEFINE_GUID(IID, IID_ISharedInterface,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x13);


MIDL_DEFINE_GUID(CLSID, CLSID_RefCountOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x11);


MIDL_DEFINE_GUID(CLSID, CLSID_PrimitiveOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x01);


MIDL_DEFINE_GUID(CLSID, CLSID_StatefulOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x03);


MIDL_DEFINE_GUID(CLSID, CLSID_ResultOperations,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x05);


MIDL_DEFINE_GUID(CLSID, CLSID_ClassCreator,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x07);


MIDL_DEFINE_GUID(CLSID, CLSID_CreatedClass,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x08);


MIDL_DEFINE_GUID(CLSID, CLSID_SharedImplementation,0x12341234,0x1234,0x1234,0x12,0x34,0x12,0x34,0x12,0x34,0x00,0x14);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



