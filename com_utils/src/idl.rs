
use super::*;
use std::io::Read;
use com_common::utils::*;

#[derive(Debug)]
pub struct Interface {
    iid : String,
    name : String,
    methods : Vec<ComMethod>,
}

impl Interface {
    fn new(
        iid : String,
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
}

#[derive(Debug)]
pub struct MethodArg {
    name: String,
    dir : ArgDirection,
    ty: String,
}

#[derive(Debug)] pub enum ArgDirection { In, Out, Return }
#[derive(Debug)] pub enum Mutability { Mutable, Immutable }

#[derive(Debug)]
pub struct CoClass {
    pub clsid : String,
    pub name : String,
    pub interfaces : Vec<String>
}

impl CoClass {
    fn new(
        clsid : String,
        name : String,
        interfaces: Vec<String>
    ) -> CoClass {
        CoClass { clsid, name, interfaces  }
    }
}

#[derive(Default, Debug)]
pub struct ParseResult {
    libid : String,
    class_names : Vec<String>,
    interfaces : Vec<Interface>,
    classes : Vec<CoClass>,
}

pub fn run( idl_params : &ArgMatches ) -> AppResult {

    let path_str = format!(
            "{}/src/**/*.rs",
            idl_params.value_of( "path" ).unwrap() );
    println!( "Globbing... {}", path_str );

    let mut result = ParseResult { ..Default::default() };
    let glob_matches = glob::glob( &path_str )?;
    for entry in glob_matches {

        let path = match entry {
            Err( e ) => { eprintln!( "{}", e ); continue; },
            Ok( path ) => path,
        };

        println!( "{}", path.display() );

        let mut f = std::fs::File::open( path )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )?;

        let parse_result = syn::parse_crate( &buf )?;
        process_crate( parse_result, &mut result );
    }

    result_to_idl( &result );
    // println!( "{:#?}", result );

    Ok(())
}

pub fn process_crate( c : syn::Crate, r : &mut ParseResult ) {

    for item in c.items {
        println!( "{:?}", item.attrs );
        let cl_attr = item.attrs.iter().find(|attr| attr.value.name() == "com_library");
        if let Some( cl ) = cl_attr {
            println!( "{:#?}", cl );
            process_com_lib_attr( cl, r );
        }

        println!( "Processing {:?}", item.ident );
        match item.node {
            syn::ItemKind::Trait(.., items)  =>
                process_trait( &item.ident, &item.attrs, &items, r ),
            syn::ItemKind::Struct(..) =>
                process_struct( &item.ident, &item.attrs, r ),
            _ => continue,
        };
    }
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
        &AttrParam::Literal( &syn::Lit::Str( ref g, .. ) ) => g.to_owned(),
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
            clsid.clone(),
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

    let methods = get_com_methods( items );
            
    r.interfaces.push( Interface::new(
            iid.clone(),
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
    items : &Vec<syn::TraitItem>
) -> Vec<ComMethod> {

    let mut v = vec![];
    for item in items {
        let method = match item.node {
            syn::TraitItemKind::Method( ref m, _ ) => m,
            _ => { println!( "Not a method: {:?}", item.ident ); continue },
        };

        let ( self_arg, other_args ) = match method.decl.inputs.split_first() {
            Some( ( s, other ) ) => ( s, other ),
            _ => { println!( "Not enough arguments: {:?}", method ); continue },
        };

        // Only self by reference is supported. COM never transfer ownership.
        let mutability = match self_arg {
            &syn::FnArg::SelfRef( _, m ) => match m {
                syn::Mutability::Mutable => Mutability::Mutable,
                syn::Mutability::Immutable => Mutability::Immutable,
            },
            _ => { println!( "Self arg not a ref: {:?}", method ); continue },
        };

        let args = match get_com_args( other_args ) {
            Ok(v) => v,
            Err(e) => { println!( "{}", e ); continue },
        };

        v.push( ComMethod {
            name : format!( "{}", item.ident ),
            mutability: mutability,
            arguments: args
        } );
    }
    v
}

fn get_com_args(
    args : &[syn::FnArg]
) -> Result< Vec<MethodArg>, AppError > {

    let mut v = vec![];
    for arg in args {
        let ( pat, ty ) = match arg {
            &syn::FnArg::Captured( ref pat, ref ty ) => ( pat, get_com_ty( ty )? ),
            _ => Err( format!( "Unsupported argument type: {:?}", arg ) )?,
        };

        let ident = match pat {
            &syn::Pat::Ident( _, ref ident, _ ) => ident,
            _ => Err( format!( "Unsupported argument pattern: {:?}", pat ) )?,
        };

        println!( "{:?}", ty );
        v.push( MethodArg {
            name: ident.to_string(),
            dir: ArgDirection::In, 
            ty: ty
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

        &syn::Ty::Path(.., syn::Path { ref segments, .. })
            => segment_to_ty( segments.last().unwrap() )?,

        &syn::Ty::BareFn(..)
            | &syn::Ty::Never
            | &syn::Ty::Tup(..)
            | &syn::Ty::TraitObject(..)
            | &syn::Ty::ImplTrait(..)
            | &syn::Ty::Paren(..)
            | &syn::Ty::Infer
            | &syn::Ty::Mac(..)
            | &syn::Ty::Never
            => Err( format!( "Argument type not supported" ) )?,
    } )
}

fn segment_to_ty( segment : &syn::PathSegment ) -> Result< String, AppError > {

    let ty = format!( "{}", segment.ident );
    match segment.parameters {
        syn::PathParameters::AngleBracketed(..) => println!( "Angled {:?}", segment ),
        syn::PathParameters::Parenthesized(..) => println!( "Angled {:?}", segment ),
    };

    Ok( match ty.as_str() {
        "ComRc" => "*".to_owned(),
        t @ _ => t.to_owned(),
    } )
}

fn result_to_idl( r : &ParseResult ) {
    let itfs = r.interfaces.iter().enumerate().map(|(i,itf)| {

        let methods = itf.methods.iter().map(|m| {

            let args = m.arguments.iter().map(|a| {
                format!( "{} {}", a.ty, a.name )
            } ).collect::<Vec<_>>().join( ", " );

            format!( r###"
                [id({})]
                HRESULT {}( {} );
            "###, i, m.name, args )
        } ).collect::<Vec<_>>().join( "\n" );
        format!( r###"
            [
                object,
                uuid( {} ),
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
        [
            uuid( {} ),
        ]
        library {}
        {{
            importlib("stdole2.tlb")
            {}
        }}
    "###, r.libid, "Calculator", itfs );
}
