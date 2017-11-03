
use super::*;
use std::io::Read;
use com_common::*;
use com_common::guid::GUID;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct Interface {
    iid : GUID,
    name : String,
    methods : Vec<ComMethod>,
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
    name : String,
    mutability : Mutability,
    arguments : Vec<MethodArg>,
    rvalue: String,
}

#[derive(Debug)]
pub struct MethodArg {
    name: String,
    dir : ArgDirection,
    ty: String,
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
    libid : Option<GUID>,
    class_names : Vec<String>,
    interfaces : Vec<Interface>,
    classes : Vec<CoClass>,
}

pub fn run( idl_params : &ArgMatches ) -> AppResult {

    let path_str = format!(
            "{}/src/**/*.rs",
            idl_params.value_of( "path" ).unwrap() );

    let mut result = ParseResult { ..Default::default() };
    let glob_matches = glob::glob( &path_str )?;
    for entry in glob_matches {

        let path = match entry {
            Err( e ) => { eprintln!( "{}", e ); continue; },
            Ok( path ) => path,
        };

        let mut f = std::fs::File::open( path )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )?;

        let parse_result = syn::parse_crate( &buf )?;
        process_crate( parse_result, &mut result )?;
    }

    result_to_idl( &result );

    Ok(())
}

pub fn process_crate( c : syn::Crate, r : &mut ParseResult ) -> AppResult {

    for item in c.items {
        let cl_attr = item.attrs
                .iter()
                .find(|attr| attr.value.name() == "com_library");
        if let Some( cl ) = cl_attr {
            process_com_lib_attr( cl, r )?;
        }

        match item.node {
            syn::ItemKind::Trait(.., items)  =>
                process_trait( &item.ident, &item.attrs, &items, r )?,
            syn::ItemKind::Impl(.., ty, items)  =>
                    process_impl(
                        utils::get_ty_ident( &ty ).ok_or(
                            format!( "Could not resolve ident of {:?}", ty ) )?,
                        &item.attrs,
                        &items,
                        r )?,
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

    let params = get_parameters( attr )
            .ok_or( format!( "Bad parameters on com_library" ) )?;

    let ( libid_param, cls_params ) = params.split_first()
            .ok_or( format!( "Not enough com_library parameters" ) )?;

    r.libid = match libid_param {
        &AttrParam::Literal( &syn::Lit::Str( ref g, .. ) )
            => Some( GUID::parse( g )? ),
        _ => Err( format!( "Invalid LIBID" ) )?,
    };

    r.class_names = cls_params
            .into_iter()
            .map( |cls| match cls {
                &AttrParam::Word( w ) => Ok( format!( "{}", w ) ),
                _ => Err( format!( "Bad interface" ) ),
            } ).collect::<Result<_,_>>()?;

    Ok(())
}

pub fn process_struct(
    ident: &syn::Ident,
    attrs: &Vec<syn::Attribute>,
    r: &mut ParseResult
) -> Result<(), AppError> {

    let class_attr = match get_attribute( "com_class", attrs ) {
        Some( a ) => a,
        _ => return Ok(())
    };

    let params = match class_attr.value {
        syn::MetaItem::List( _, ref params ) => params,
        _ => Err( format!( "Bad parameters on com_class on {}", ident ) )?,
    };

    let params = get_parameters( class_attr )
            .ok_or( format!( "Bad parameters on com_class on {}", ident ) )?;

    let ( name_param, itf_params ) = params.split_first()
            .ok_or( format!( "Not enough com_class parameters on {}", ident ) )?;

    let clsid = match name_param {
        &AttrParam::Literal( &syn::Lit::Str( ref g, .. ) ) => g,
        _ => Err( format!( "Invalid CLSID on {}", ident ) )?,
    };

    let interfaces = itf_params
            .into_iter()
            .map( |itf| match itf {
                &AttrParam::Word( w ) => Ok( format!( "{}", w ) ),
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
    r: &mut ParseResult
) -> Result<(), AppError> {

    let mut methods = vec![];
    for item in items {
        methods.push( match item.node {
            syn::TraitItemKind::Method( ref method, .. )
                    => ( &item.ident, method ),
            _ => continue,
        } );
    }

    process_interface( ident, attrs, methods, r )
}

pub fn process_impl( 
    ident : &syn::Ident,
    attrs : &Vec<syn::Attribute>,
    items : &Vec<syn::ImplItem>,
    r: &mut ParseResult
) -> Result<(), AppError> {

    let mut methods = vec![];
    for item in items {
        methods.push( match item.node {
            syn::ImplItemKind::Method( ref method, .. )
                    => ( &item.ident, method ),
            _ => continue,
        } );
    }

    process_interface( ident, attrs, methods, r )
}

pub fn process_interface( 
    ident : &syn::Ident,
    attrs : &Vec<syn::Attribute>,
    items : Vec<( &syn::Ident, &syn::MethodSig )>,
    r: &mut ParseResult
) -> Result<(), AppError> {

    let interface_attr = match get_attribute( "com_interface", attrs ) {
        Some( a ) => a,
        _ => return Ok(())
    };

    let params = match interface_attr.value {
        syn::MetaItem::List( _, ref params ) => params,
        _ => Err( format!( "Bad parameters on com_interface on {}", ident ) )?,
    };

    let params = get_parameters( interface_attr )
            .ok_or( format!( "Bad parameters on com_interface on {}", ident ) )?;

    let guid_param = params.first()
            .ok_or( format!( "Not enough com_interface parameters on {}", ident ) )?;

    let iid = match guid_param {
        &AttrParam::Literal( &syn::Lit::Str( ref g, .. ) ) => g,
        _ => Err( format!( "Invalid IID on {}", ident ) )?,
    };

    let methods = get_com_methods( items )?;
            
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

enum AttrParam<'a> {
    Literal( &'a syn::Lit ),
    Word( &'a syn::Ident ),
}

fn get_parameters(
    attr : &syn::Attribute
) -> Option< Vec< AttrParam > >
{
    Some( match attr.value {

        syn::MetaItem::Word(..) => return None,
        syn::MetaItem::NameValue(..) => return None,
        syn::MetaItem::List( _, ref l ) =>
            l.iter().map( |i| match i {
                &syn::NestedMetaItem::MetaItem( ref mi ) =>
                        AttrParam::Word( match mi {
                            &syn::MetaItem::Word( ref i ) => i,
                            &syn::MetaItem::List( ref i, _ ) => i,
                            &syn::MetaItem::NameValue( ref i, _ ) => i,
                        } ),
                &syn::NestedMetaItem::Literal( ref l ) =>
                        AttrParam::Literal( l ),
            } ).collect() ,
    } )
}

fn get_com_methods(
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

        let mut args = match get_com_args( other_args ) {
            Ok(v) => v,
            Err(e) => { println!( "{}", e ); continue },
        };

        let rvalue = match &method.decl.output {
            &syn::FunctionRetTy::Default => "void".to_owned(),
            &syn::FunctionRetTy::Ty( ref ty ) => {
                let ( out_ty_opt, ret_ty ) = utils::get_ret_types( ty )?;
                if let Some( out_ty ) = out_ty_opt {
                    let arg_ty = get_com_ty( &out_ty )?;
                    if arg_ty != "void" {
                        args.push( MethodArg {
                            name: "__out".to_owned(),
                            dir: ArgDirection::Return,
                            ty: arg_ty
                        } );
                    }
                }
                get_com_ty( &ret_ty )?
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

        let idl_ty = get_com_ty( ty )?;

        v.push( MethodArg {
            name: ident.to_string(),
            dir: ArgDirection::In, 
            ty: idl_ty
        } );
    }
    Ok( v )
}

fn get_com_ty( ty : &syn::Ty ) -> Result< String, AppError > {

    Ok( match ty {

        // Pointer types.
        &syn::Ty::Slice( ref ty )
            => format!( "*{}", get_com_ty( ty )? ),
        &syn::Ty::Ptr( ref mutty )
            | &syn::Ty::Rptr( .., ref mutty )
            => match mutty.mutability {
                syn::Mutability::Mutable => format!( "*{}", get_com_ty( &mutty.ty )? ),
                syn::Mutability::Immutable => format!( "*const {}", get_com_ty( &mutty.ty )? ),
            },

        &syn::Ty::Array( ref ty, ref count )
            => format!( "{}[{:?}]", get_com_ty( ty.as_ref() )?, count ),

        &syn::Ty::Path(.., ref path )
            => path_to_ty( path )?,

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

fn path_to_ty( path : &syn::Path ) -> Result< String, AppError >
{
    let &syn::Path { ref segments, .. } = path;
    segment_to_ty( segments.last().unwrap() )
}

fn segment_to_ty( segment : &syn::PathSegment ) -> Result< String, AppError > {

    let ty = format!( "{}", segment.ident );
    let args = match segment.parameters {
        syn::PathParameters::AngleBracketed( ref data )
                => &data.types,

        syn::PathParameters::Parenthesized( ref data )
                => &data.inputs,
    };

    Ok( match ty.as_str() {
        "ComRc" => format!( "{}*", get_com_ty( &args[0] )? ),
        "usize" => "size_t".to_owned(),
        "u32" => "uint32_t".to_owned(),
        "i32" => "int32_t".to_owned(),
        "u16" => "uint16_t".to_owned(),
        "i16" => "int16_t".to_owned(),
        "u8" => "uint8_t".to_owned(),
        "i8" => "int8_t".to_owned(),
        t @ _ => t.to_owned(),
    } )
}

fn result_to_idl( r : &ParseResult ) {
    let itfs = r.interfaces.iter().map(|itf| {

        let methods = itf.methods.iter().enumerate().map(|(i,m)| {

            let args = m.arguments.iter().map(|a| {
                let ( attrs, out_ptr ) = match a.dir {
                    ArgDirection::In => ( "in", "" ),
                    ArgDirection::Out => ( "out", "*" ),
                    ArgDirection::Return => ( "out, retval", "*" ),
                };
                format!( "[{}] {}{} {}", attrs, a.ty, out_ptr, a.name )
            } ).collect::<Vec<_>>().join( ", " );

            format!( r###"
                [id({:X})]
                {} {}( {} );
            "###, i, m.rvalue, m.name, args )
        } ).collect::<Vec<_>>().join( "\n" );
        format!( r###"
            [
                object,
                uuid( {:X} ),
                nonextensible,
                pointer_default(unique)
            ]
            interface {} : IUnknown
            {{
                {}
            }}
        "###, itf.iid, itf.name, methods )
    } ).collect::<Vec<_>>().join( "\n" );


    println!( r###"
        #include <stdint.h>
        [
            uuid( {:X} )
        ]
        library {}
        {{
            importlib("stdole2.tlb");
            {}
        }}
    "###, r.libid.as_ref().unwrap(), "Calculator", itfs );
}
