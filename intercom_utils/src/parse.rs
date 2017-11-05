
use super::*;
use intercom_common::*;
use intercom_common::guid::GUID;
use std::io::Read;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interface {
    pub iid : GUID,
    pub name : String,
    pub methods : Vec<ComMethod>,
}

impl Interface {
    fn new(
        iid : GUID,
        name : String,
        methods: Vec<ComMethod>
    ) -> Interface {
        Interface { iid, name, methods }
    }
}

#[derive(Debug)]
pub struct ComMethod {
    pub name : String,
    pub mutability : Mutability,
    pub arguments : Vec<MethodArg>,
    pub rvalue: String,
}

#[derive(Debug)]
pub struct MethodArg {
    pub name: String,
    pub dir : ArgDirection,
    pub ty: String,
}

#[derive(Debug, PartialEq)] pub enum ArgDirection { In, Out, Return }
#[derive(Debug)] pub enum Mutability { Mutable, Immutable }

#[derive(Debug)]
pub struct CoClass {
    pub clsid : GUID,
    pub name : String,
    pub interfaces : Vec<String>
}

impl CoClass {
    fn new(
        clsid : GUID,
        name : String,
        interfaces: Vec<String>
    ) -> CoClass {
        CoClass { clsid, name, interfaces  }
    }
}

#[derive(Default, Debug)]
pub struct ParseResult {
    pub libname : Option<String>,
    pub libid : Option<GUID>,
    pub class_names : Vec<String>,
    pub interfaces : Vec<Interface>,
    pub classes : Vec<CoClass>,
}

pub fn parse_crate(
    path : String,
) -> Result< ( HashMap<String, String>, ParseResult ), AppError > {

    let files = glob::glob( &path )?.collect::<Vec<_>>();

    // Gather renames first.
    let mut renames = HashMap::new();
    for entry in &files {

        let path = match entry {
            &Err( ref e ) => { eprintln!( "{}", e ); continue; },
            &Ok( ref path ) => path,
        };

        let mut f = std::fs::File::open( path )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )?;

        let parse_result = syn::parse_crate( &buf )?;
        gather_renames( parse_result, &mut renames )?;
    }

    // Parse the files.
    let mut result = ParseResult { ..Default::default() };
    for entry in &files {

        let path = match entry {
            &Err( ref e ) => { eprintln!( "{}", e ); continue; },
            &Ok( ref path ) => path,
        };

        let mut f = std::fs::File::open( path )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )?;

        let parse_result = syn::parse_crate( &buf )?;
        process_crate( parse_result, &mut result, &renames )?;
    }

    Ok( ( renames, result ) )
}

pub fn gather_renames(
    c : syn::Crate,
    rn : &mut HashMap<String, String>,
) -> AppResult {

    for item in c.items {
        match item.node {

            syn::ItemKind::Impl(.., ty, items) => {
                let struct_ident = utils::get_ty_ident( &ty ).ok_or(
                    format!( "Could not resolve ident of {:?}", ty ) )?;

                let itf_attr = item.attrs
                        .iter()
                        .find(|attr| attr.value.name() == "com_interface");
                if let Some( itf ) = itf_attr {

                    let iname =
                        format!( "I{}", struct_ident );
                    rn.insert( struct_ident.to_string(), iname );
                }
            },
            _ => {}
        };
    }

    Ok(())
}

pub fn process_crate(
    c : syn::Crate,
    r : &mut ParseResult,
    rn : &HashMap<String, String>,
) -> AppResult {

    for item in c.items {
        let cl_attr = item.attrs
                .iter()
                .find(|attr| attr.value.name() == "com_library");
        if let Some( cl ) = cl_attr {
            process_com_lib_attr( cl, r )?;
        }

        match item.node {
            syn::ItemKind::Trait(.., items)  =>
                process_trait( &item.ident, &item.attrs, &items, r, rn )?,
            syn::ItemKind::Impl(.., ty, items)  =>
                    process_impl(
                        utils::get_ty_ident( &ty ).ok_or(
                            format!( "Could not resolve ident of {:?}", ty ) )?,
                        &item.attrs,
                        &items,
                        r, rn )?,
            syn::ItemKind::Struct(..) =>
                process_struct( &item.ident, &item.attrs, r )?,
            _ => continue,
        };
    }

    Ok(())
}

pub fn process_com_lib_attr(
    attr : &syn::Attribute,
    r : &mut ParseResult,
) -> Result<(), AppError> {

    let ( libname, libid, class_names ) = utils::parse_com_lib_attribute( attr )?;

    r.libname = Some( libname );
    r.libid = Some( libid );
    r.class_names = class_names;

    Ok(())
}

pub fn process_struct(
    ident: &syn::Ident,
    attrs: &Vec<syn::Attribute>,
    r: &mut ParseResult,
) -> Result<(), AppError> {

    let class_attr = match get_attribute( "com_class", attrs ) {
        Some( a ) => a,
        _ => return Ok(())
    };

    let params = match class_attr.value {
        syn::MetaItem::List( _, ref params ) => params,
        _ => Err( format!( "Bad parameters on com_class on {}", ident ) )?,
    };

    let params = utils::get_parameters( class_attr )
            .ok_or( format!( "Bad parameters on com_class on {}", ident ) )?;

    let ( name_param, itf_params ) = params.split_first()
            .ok_or( format!( "Not enough com_class parameters on {}", ident ) )?;

    let clsid = match name_param {
        &utils::AttrParam::Literal( &syn::Lit::Str( ref g, .. ) ) => g,
        _ => Err( format!( "Invalid CLSID on {}", ident ) )?,
    };

    let interfaces = itf_params
            .into_iter()
            .map( |itf| match itf {
                &utils::AttrParam::Word( w ) => Ok( format!( "{}", w ) ),
                _ => Err( format!( "Bad interface" ) ),
            } ).collect::<Result<_,_>>()?;
            
    r.classes.push( CoClass::new(
            GUID::parse( clsid )?,
            format!( "{}", ident ),
            interfaces ) );

    Ok(())
}

pub fn process_trait( 
    ident : &syn::Ident,
    attrs : &Vec<syn::Attribute>,
    items : &Vec<syn::TraitItem>,
    r: &mut ParseResult,
    rn : &HashMap<String, String>,
) -> Result<(), AppError> {

    let mut methods = vec![];
    for item in items {
        methods.push( match item.node {
            syn::TraitItemKind::Method( ref method, .. )
                    => ( &item.ident, method ),
            _ => continue,
        } );
    }

    process_interface( ident, attrs, methods, r, rn )
}

pub fn process_impl( 
    ident : &syn::Ident,
    attrs : &Vec<syn::Attribute>,
    items : &Vec<syn::ImplItem>,
    r: &mut ParseResult,
    rn : &HashMap<String, String>,
) -> Result<(), AppError> {

    let mut methods = vec![];
    for item in items {
        methods.push( match item.node {
            syn::ImplItemKind::Method( ref method, .. )
                    => ( &item.ident, method ),
            _ => continue,
        } );
    }

    process_interface( &ident, attrs, methods, r, rn )
}

pub fn process_interface( 
    ident : &syn::Ident,
    attrs : &Vec<syn::Attribute>,
    items : Vec<( &syn::Ident, &syn::MethodSig )>,
    r: &mut ParseResult,
    rn : &HashMap<String, String>,
) -> Result<(), AppError> {

    let interface_attr = match get_attribute( "com_interface", attrs ) {
        Some( a ) => a,
        _ => return Ok(())
    };

    let params = match interface_attr.value {
        syn::MetaItem::List( _, ref params ) => params,
        _ => Err( format!( "Bad parameters on com_interface on {}", ident ) )?,
    };

    let params = utils::get_parameters( interface_attr )
            .ok_or( format!( "Bad parameters on com_interface on {}", ident ) )?;

    let guid_param = params.first()
            .ok_or( format!( "Not enough com_interface parameters on {}", ident ) )?;

    let iid = match guid_param {
        &utils::AttrParam::Literal( &syn::Lit::Str( ref g, .. ) ) => g,
        _ => Err( format!( "Invalid IID on {}", ident ) )?,
    };

    let methods = get_com_methods( rn, items )?;
            
    r.interfaces.push( Interface::new(
            GUID::parse( iid )?,
            format!( "{}", ident ),
            methods ) );

    Ok(())
}

fn get_attribute<'a, 'b>(
    name : &'b str,
    attrs : &'a Vec<syn::Attribute>
) -> Option< &'a syn::Attribute >
{
    attrs.iter().find( |attr| attr.value.name() == name )
}

fn get_com_methods(
    rn : &HashMap<String, String>,
    methods : Vec<( &syn::Ident, &syn::MethodSig )>
) -> Result<Vec<ComMethod>, AppError> {

    let mut v = vec![];
    for ( ident, method ) in methods {

        let ( self_arg, other_args ) = match method.decl.inputs.split_first() {
            Some( ( s, other ) ) => ( s, other ),

            // Getting first fails if there are no arguments. This means no
            // 'self' argument, thus not a proper instance method.
            _ => continue,
        };

        // Only self by reference is supported. COM never transfer ownership.
        let mutability = match self_arg {
            &syn::FnArg::SelfRef( _, m ) => match m {
                syn::Mutability::Mutable => Mutability::Mutable,
                syn::Mutability::Immutable => Mutability::Immutable,
            },
            _ => continue,
        };

        let mut args = match get_com_args( rn, other_args ) {
            Ok(v) => v,
            Err(e) => { println!( "{}", e ); continue },
        };

        let rvalue = match &method.decl.output {
            &syn::FunctionRetTy::Default => "void".to_owned(),
            &syn::FunctionRetTy::Ty( ref ty ) => {
                let ( out_ty_opt, ret_ty ) = utils::get_ret_types( ty )?;
                if let Some( out_ty ) = out_ty_opt {
                    let arg_ty = get_com_ty( rn, &out_ty )?;
                    if arg_ty != "void" {
                        args.push( MethodArg {
                            name: "__out".to_owned(),
                            dir: ArgDirection::Return,
                            ty: arg_ty
                        } );
                    }
                }
                get_com_ty( rn, &ret_ty )?
            }
        };

        v.push( ComMethod {
            name : format!( "{}", ident ),
            mutability: mutability,
            arguments: args,
            rvalue: rvalue,
        } );
    }
    Ok( v )
}

fn get_com_args(
    rn : &HashMap<String, String>,
    args : &[syn::FnArg]
) -> Result< Vec<MethodArg>, AppError > {

    let mut v = vec![];
    for arg in args {
        let ( pat, ty ) = match arg {
            &syn::FnArg::Captured( ref pat, ref ty ) => ( pat, ty ),
            _ => Err( format!( "Unsupported argument type: {:?}", arg ) )?,
        };

        let ident = match pat {
            &syn::Pat::Ident( _, ref ident, _ ) => ident,
            _ => Err( format!( "Unsupported argument pattern: {:?}", pat ) )?,
        };

        let idl_ty = get_com_ty( rn, ty )?;

        v.push( MethodArg {
            name: ident.to_string(),
            dir: ArgDirection::In, 
            ty: idl_ty
        } );
    }
    Ok( v )
}

fn get_com_ty(
    rn : &HashMap<String, String>,
    ty : &syn::Ty,
) -> Result< String, AppError > {

    Ok( match ty {

        // Pointer types.
        &syn::Ty::Slice( ref ty )
            => format!( "*{}", get_com_ty( rn, ty )? ),
        &syn::Ty::Ptr( ref mutty )
            | &syn::Ty::Rptr( .., ref mutty )
            => match mutty.mutability {
                syn::Mutability::Mutable
                    => format!( "*{}", get_com_ty( rn, &mutty.ty )? ),
                syn::Mutability::Immutable
                    => format!( "*const {}", get_com_ty( rn, &mutty.ty )? ),
            },

        &syn::Ty::Array( ref ty, ref count )
            => format!( "{}[{:?}]", get_com_ty( rn, ty.as_ref() )?, count ),

        &syn::Ty::Path(.., ref path )
            => path_to_ty( rn, path )?,

        &syn::Ty::Tup( ref l ) if l.len() == 0
            => "void".to_owned(),

        &syn::Ty::BareFn(..)
            | &syn::Ty::Never
            | &syn::Ty::Tup(..)
            | &syn::Ty::TraitObject(..)
            | &syn::Ty::ImplTrait(..)
            | &syn::Ty::Paren(..)
            | &syn::Ty::Infer
            | &syn::Ty::Mac(..)
            | &syn::Ty::Never
            => Err( format!( "Argument type not supported: {:?}", ty ) )?,
    } )
}

fn path_to_ty(
    rn : &HashMap<String, String>,
    path : &syn::Path,
) -> Result< String, AppError >
{
    let &syn::Path { ref segments, .. } = path;
    segment_to_ty( rn, segments.last().unwrap() )
}

fn segment_to_ty(
    rn : &HashMap<String, String>,
    segment : &syn::PathSegment,
) -> Result< String, AppError > {

    let ty = format!( "{}", segment.ident );
    let args = match segment.parameters {
        syn::PathParameters::AngleBracketed( ref data )
                => &data.types,

        syn::PathParameters::Parenthesized( ref data )
                => &data.inputs,
    };

    Ok( match ty.as_str() {
        "ComRc" => format!( "{}*", get_com_ty( rn, &args[0] )? ),
        "usize" => "size_t".to_owned(),
        "u64" => "uint64".to_owned(),
        "i64" => "int64".to_owned(),
        "u32" => "uint32".to_owned(),
        "i32" => "int32".to_owned(),
        "u16" => "uint16".to_owned(),
        "i16" => "int16".to_owned(),
        "u8" => "uint8".to_owned(),
        "i8" => "int8".to_owned(),
        "f64" => "double".to_owned(),
        "f32" => "float".to_owned(),
        t @ _ => try_rename( rn, t ),
    } )
}

pub fn try_rename(
    rn : &HashMap<String, String>,
    name : &str
) -> String
{
    if let Some( n ) = rn.get( name ) {
        n.to_owned()
    } else {
        name.to_owned()
    }
}

pub fn camel_case( input : &str ) -> String {
    let mut output = String::new();
    output.reserve( input.len() );

    let mut capitalize = true;
    for c in input.chars() {
        if c == '_' {
            capitalize = true;
        } else {
            if capitalize { 
                for c_up in c.to_uppercase() {
                    output.push( c_up )
                }
                capitalize = false;
            } else {
                output.push( c );
            }
            capitalize = false;
        }

    }
    output
}

