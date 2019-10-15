use intercom::{
    prelude::*,
    ComItf, ComRc,
    type_system::{TypeSystemName, AutomationTypeSystem},
    typelib::{
        IIntercomTypeLib,
        IIntercomTypeInfo,
        IIntercomCoClass,
        IIntercomInterface,
        IIntercomMethod,
        TypeInfoKind
    },
};
use std::path::Path;

#[derive(Fail, Debug)]
pub enum TypeLibError {
    #[fail( display = "Could not acquire IntercomTypeLib: {}", _0 )]
    AcquiringTypeLib( String ),
}

pub fn read_typelib( path : &Path ) -> Result<intercom::typelib::TypeLib, failure::Error>
{
    let lib = libloading::Library::new(path)?;
    let typelib = unsafe {
        let fn_get_type_lib : libloading::Symbol<
            unsafe extern fn(
                TypeSystemName,
                *mut intercom::RawComPtr,
            ) -> intercom::raw::HRESULT>
                = lib.get( b"IntercomTypeLib" )?;

        let mut ptr : intercom::RawComPtr = std::ptr::null_mut();
        println!( "{:?}", fn_get_type_lib( TypeSystemName::Automation, &mut ptr as *mut _ ) );
        println!( "{:?}", ptr );

        let comptr = intercom::raw::InterfacePtr::<AutomationTypeSystem, IIntercomTypeLib>::new(ptr);
        let comitf = intercom::ComItf::maybe_wrap(comptr)
            .ok_or_else(|| TypeLibError::AcquiringTypeLib( "Null ptr".to_owned() ))?;

        intercom::ComRc::attach(comitf)
    };

    println!( "GetInfo: {:?}", typelib.get_info()? );

    for i in 0..typelib.get_type_count()? {
        read_type(&typelib.get_type(i)?)?;
    }

    Ok(intercom::typelib::TypeLib::from_comrc(&typelib)?)
}

fn read_type(ty: &ComRc<IIntercomTypeInfo>) -> Result<(), failure::Error>
{
    let name = ty.get_name()?;
    let kind = ty.get_kind()?;
    println!( "{:?}: {}", kind, name);

    match kind {
        TypeInfoKind::CoClass => {

            // Funnily enough this query_interface will end up preferring
            // the raw interface.
            read_coclass( &ComItf::query_interface(&ty)? )?;
        },
        TypeInfoKind::Interface => {

            read_interface( &ComItf::query_interface(&ty)? )?;
        }
    }

    Ok(())
}

fn read_coclass(coclassinfo: &ComRc<IIntercomCoClass>) -> Result<(), failure::Error>
{
    println!( " - CLSID: {}", coclassinfo.get_clsid()? );
    for n in 0..coclassinfo.get_interface_count()? {
        let (itf_name, itf_guid) = coclassinfo.get_interface_ref(
            n, TypeSystemName::Automation)?;
        println!( " - impls {} ({})", itf_name, itf_guid );
    }

    Ok(())
}

fn read_interface(itf_info: &ComRc<IIntercomInterface>) -> Result<(), failure::Error>
{
    for n in 0..itf_info.get_variant_count()? {
        let itf_variant = itf_info.get_variant(n)?;
        let iid = itf_variant.get_iid()?;
        let ts = itf_variant.get_type_system()?;
        println!( " - IID: {} ({:?})", iid, ts );

        for m in 0..itf_variant.get_method_count()? {
            read_method( &itf_variant.get_method(m)? )?;
        }
    }

    Ok(())
}

fn read_method(method: &ComRc<IIntercomMethod>) -> Result<(), failure::Error>
{
    let mut params = vec![];
    let (rv_ty, rv_ptrs) = method.get_return_type()?;
    for i in 0..method.get_parameter_count()? {
        let (name, ty, ptrs) = method.get_parameter(i)?;
        params.push(format!("{}: {}", name, get_ty_name(ptrs, &ty)));
    }
    println!( "   - {}({}) -> {}", method.get_name()?, params.join(", "), get_ty_name(rv_ptrs, &rv_ty));

    Ok(())
}

fn get_ty_name(ptrs: u32, ty: &str) -> String {
    format!( "{}{}", ty, "*".repeat(ptrs as usize))
}
