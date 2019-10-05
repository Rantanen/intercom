
use crate::*;

pub type MEMBERID = i32;

#[com_class(ITypeLib)]
pub struct TypeLib {
    name : &'static str,
    libid : GUID,
    version: &'static str,
    types : Vec< TypeInfo >,
}

impl TypeLib {

    #[doc(hidden)]
    pub fn __new(
        name: &'static str,
        libid: GUID,
        version: &'static str,
        types: Vec<TypeInfo>
    ) -> TypeLib {
        TypeLib { name, libid, version, types }
    }
}

#[com_interface]
pub trait ITypeLib {
    fn find_name(&self, name: &str, hash_val: u32) -> ComResult<(ComItf<dyn ITypeInfo>, MEMBERID, u16)>;
    fn get_documentation(&self) -> ComResult<(String, String, i32, String)>;
    fn get_lib_attr(&self) -> ComResult<*mut LIBATTR>;
    fn get_type_comp(&self) -> ComResult<ComItf<dyn ITypeComp>>;
    fn get_type_info(&self, idx: u32) -> ComResult<ComItf<dyn ITypeInfo>>;
    fn get_type_info_count(&self) -> u32;
    fn get_type_info_of_guid(&self, typeid: GUID) -> ComResult<ComItf<dyn ITypeInfo>>;
    fn get_type_info_of_type(&self, idx: u32) -> ComResult<TYPEKIND>;
    fn release_tlibattr(&self, libattr: *mut LIBATTR);

    // FIXME: name should be case-corrected, which means it should be &mut.
    fn is_name(&self, name: &str, hash: u32) -> ComResult<bool>;
}

#[com_impl]
impl ITypeLib for TypeLib {

    fn find_name(&self, name: &str, hash_val: u32) -> ComResult<(ComItf<dyn ITypeInfo>, MEMBERID, u16)>
    {
        Err( ComError::E_NOTIMPL )
    }

    fn get_documentation(&self) -> ComResult<(String, String, i32, String)>
    {
        Ok(( "".to_owned(), "".to_owned(), 0, "".to_owned() ))
    }

    fn get_lib_attr(&self) -> ComResult<*mut LIBATTR>
    {
        Ok( Box::into_raw(Box::new( LIBATTR {} )) )
    }

    fn get_type_comp(&self) -> ComResult<ComItf<dyn ITypeComp>>
    {
        Err( ComError::E_NOTIMPL )
    }

    fn get_type_info(&self, idx: u32) -> ComResult<ComItf<dyn ITypeInfo>>
    {
        Err( ComError::E_NOTIMPL )
    }

    fn get_type_info_count(&self) -> u32
    {
        self.types.len() as u32
    }

    fn get_type_info_of_guid(&self, typeid: GUID) -> ComResult<ComItf<dyn ITypeInfo>>
    {
        Err( ComError::E_NOTIMPL )
    }

    fn get_type_info_of_type(&self, idx: u32) -> ComResult<TYPEKIND>
    {
        Err( ComError::E_NOTIMPL )
    }

    fn release_tlibattr(&self, _libattr: *mut LIBATTR)
    {
        // Do nothing. We'll just let the box fall out of scope so it gets
        // destroyed automatically.
    }

    fn is_name(&self, name: &str, hash: u32) -> ComResult<bool>
    {
        Err( ComError::E_NOTIMPL )
    }
}

#[repr(C)]
#[derive(Default, BidirectionalTypeInfo, ExternType)]
pub struct LIBATTR {
}

#[com_interface]
pub trait ITypeInfo {
}

#[com_interface]
pub trait ITypeComp {
}

pub enum TypeInfo {
    Class(CoClass),
    Interface(Interface),
}

pub struct CoClass {
}

pub struct Interface {
}

#[repr(C)]
#[derive(BidirectionalTypeInfo, ExternType)]
pub enum TYPEKIND { None }

impl Default for TYPEKIND {
    fn default() -> Self { TYPEKIND::None }
}
