
use super::*;
use intercom_common::*;
use intercom_common::guid::GUID;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GUIDType { CLSID, IID }

/// COM interface details.
#[derive(Debug)]
pub struct Interface {
    pub iid : GUID,
    pub name : String,
    pub methods : Vec<ComMethod>,
}

impl Interface {

    /// Construct new interface data object.
    fn new(
        iid : GUID,
        name : String,
        methods: Vec<ComMethod>
    ) -> Interface {
        Interface { iid, name, methods }
    }
}

/// COM method details.
#[derive(Debug)]
pub struct ComMethod {
    pub name : String,
    pub mutability : Mutability,
    pub arguments : Vec<MethodArg>,
    pub rvalue: String,
}

/// COM method argument details.
#[derive(Debug)]
pub struct MethodArg {
    pub name: String,
    pub dir : ArgDirection,
    pub ty: String,
}

/// Argument direction.
#[derive(Debug, PartialEq)] pub enum ArgDirection { In, Return }

/// Mutability information.
#[derive(Debug)] pub enum Mutability { Mutable, Immutable }

/// COM `CoClass` details.
#[derive(Debug)]
pub struct CoClass {
    pub clsid : GUID,
    pub name : String,
    pub interfaces : Vec<String>
}

impl CoClass {

    /// Constructs new coclass data object.
    fn new(
        clsid : GUID,
        name : String,
        interfaces: Vec<String>
    ) -> CoClass {
        CoClass { clsid, name, interfaces  }
    }
}

/// The result of a parse.
#[derive(Default, Debug)]
pub struct ParseResult {
    pub libname : String,
    pub libid : Option<GUID>,
    pub class_names : Vec<String>,
    pub interfaces : Vec<Interface>,
    pub classes : Vec<CoClass>,
}

/// Parses a crate.
///
/// * `path` - glob pattern for all the rust source files included in the crate.
///
/// Returns 
pub fn parse_crate(
    crate_root : &str,
) -> Result< ( HashMap<String, String>, ParseResult ), AppError > {

    let cargo_toml = parse_toml( &format!( "{}/Cargo.toml", crate_root ) )?;
    let libname = match cargo_toml {
        toml::Value::Table( root ) => match root.get( "package" ) {
            Some( &toml::Value::Table( ref package ) )
                => match package.get( "name" ) {
                    Some( &toml::Value::String( ref name ) )
                        => name.clone(),
                    _ => Err( "No 'name' parameter under [package]" )?,
                },
            _ => Err( "Could not find [package] in Cargo.toml" )?,
        },
        _ => Err( "Could not parse Cargo.toml" )?
    };

    // Glob the sources.
    let src_path = format!( "{}/src/**/*.rs", crate_root );
    let files = glob::glob( &src_path )?.collect::<Vec<_>>();

    // Gather renames first.
    let mut renames = HashMap::new();
    for entry in &files {

        // Get the path from the directory entry.
        let path = match *entry {
            Err( ref e ) => { eprintln!( "{}", e ); continue; },
            Ok( ref path ) => path,
        };

        // Read the source.
        let mut f = std::fs::File::open( path )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )?;

        // Gather the renames.
        let parse_result = syn::parse_crate( &buf )?;
        gather_renames( parse_result, &mut renames )?;
    }

    // Parse the files.
    let mut result = ParseResult {
        libname: libname,
        ..Default::default()
    };
    for entry in &files {

        // Get the path from the directory entry.
        let path = match *entry {
            Err( ref e ) => { eprintln!( "{}", e ); continue; },
            Ok( ref path ) => path,
        };

        // Read the source.
        let mut f = std::fs::File::open( path )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )?;

        // Parse the types.
        let parse_result = syn::parse_crate( &buf )?;
        process_crate( parse_result, &mut result, &renames )?;
    }

    Ok( ( renames, result ) )
}

fn parse_toml( path : &str ) -> Result< toml::Value, AppError >
{
    let mut f = std::fs::File::open( path )?;
    let mut buf = String::new();
    f.read_to_string( &mut buf )?;

    Ok( buf.parse::<toml::Value>()? )
}

/// Gathers COM type renames.
///
/// The renames are represented as a map from a Rust name to the expected COM
/// name. Currently this is used only for mapping the implicit interfaces into
/// COM interface names.
///
/// The primary reason for this is the fact that interfaces and coclasses cannot
/// share names and as the coclass already has the struct name the implicit
/// impl interface must have a different name. In our case we prefix the struct
/// name with an 'I'.
///
/// ```
/// struct Foo;
///
/// #[com_interface]
/// impl Foo
/// ```
///
/// results in
///
/// ```
/// coclass Foo { .. }
/// interface IFoo { .. }
/// ```
pub fn gather_renames(
    c : syn::Crate,
    rn : &mut HashMap<String, String>,
) -> AppResult {

    // Process each item.
    for item in c.items {

        // Only implicit impls need renaming.
        if let syn::ItemKind::Impl( .., ty, _ ) = item.node {

            let struct_ident = utils::get_ty_ident( &ty ).ok_or_else( ||
                format!( "Could not resolve ident of {:?}", ty ) )?;

            // Ensure the impl is marked with com_interface attribute.
            // Non-com_interface impls don't matter here.
            let itf_attr = item.attrs
                    .iter()
                    .find(|attr| attr.value.name() == "com_interface");
            if let Some(..) = itf_attr {

                // com_interface attribute was found. Add the rename.
                let iname =
                    format!( "I{}", struct_ident );
                rn.insert( struct_ident.to_string(), iname );
            }
        }
    }

    Ok(())
}

/// Processes a single file.
///
/// * `c` - File to process as a `syn::Crate` AST structure.
/// * `r` - Mutable parse result that we append our results in.
/// * `rn` - Rename data to use when resolving type names.
pub fn process_crate(
    c : syn::Crate,
    r : &mut ParseResult,
    rn : &HashMap<String, String>,
) -> AppResult {

    // Process each item in the crate.
    for item in c.items {

        // Check the com_library attribute.
        //
        // This attribute SHOULD be a crate-level attribute, but that causes
        // all sorts of problems everywhere so unfortunately we need to smack
        // it on a random item instead.
        let cl_attr = item.attrs
                .iter()
                .find(|attr| attr.value.name() == "com_library");
        if let Some( cl ) = cl_attr {
            process_com_lib_attr( cl, r )?;
        }

        // Process the item.
        match item.node {
            syn::ItemKind::Trait(.., items) =>
                    process_trait( &item.ident, &item.attrs, &items, r, rn )?,
            syn::ItemKind::Impl(.., ty, items) =>
                    process_impl(
                        utils::get_ty_ident( &ty ).ok_or_else( ||
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

/// Process COM library attribute.
pub fn process_com_lib_attr(
    attr : &syn::Attribute,
    r : &mut ParseResult,
) -> Result<(), AppError> {

    // Store the details defined on the library attribute in the results.
    let ( libid, class_names ) = utils::parse_com_lib_attribute(
            &r.libname,
            attr )?;
    r.libid = Some( libid );
    r.class_names = class_names;

    Ok(())
}

pub fn get_guid(
    results : &ParseResult,
    guid_type : &GUIDType,
    ident : &syn::Ident,
    param : &utils::AttrParam
) -> Result< GUID, AppError >
{
    match *param {
        utils::AttrParam::Literal( &syn::Lit::Str( ref g, .. ) )
            => Ok( GUID::parse( g )? ),
        utils::AttrParam::Word( i ) if i == "AUTO_GUID"
            => Ok( utils::generate_guid(
                &results.libname,
                ident.as_ref(),
                match *guid_type {
                    GUIDType::CLSID => "CLSID",
                    GUIDType::IID => "IID",
                } ) ),
        _ => Err( format!( "Invalid GUID on {}, {:?}", ident, param ) )?,
    }
}

/// Process COM struct.
pub fn process_struct(
    ident: &syn::Ident,
    attrs: &[syn::Attribute],
    r: &mut ParseResult,
) -> Result<(), AppError> {

    // This function is invoked for all structs. If the current item is not
    // marked with the [com_class] attribute we can return early.
    let class_attr = match get_attribute( "com_class", attrs ) {
        Some( a ) => a,
        _ => return Ok(())
    };

    // Get the COM parameters.
    let params = utils::get_parameters( class_attr )
            .ok_or_else( ||
                format!( "Bad parameters on com_class on {}", ident ) )?;
    let ( name_param, itf_params ) = params.split_first()
            .ok_or_else( ||
                format!( "Not enough com_class parameters on {}", ident ) )?;

    // Parse the CLSID from the attribute.
    let clsid = get_guid( r, &GUIDType::CLSID, ident, name_param )?;

    // Parse the interface names.
    let interfaces = itf_params
            .into_iter()
            .map( |itf| match *itf {
                utils::AttrParam::Word( w ) => Ok( format!( "{}", w ) ),
                _ => Err( "Bad interface" ),
            } ).collect::<Result<_,_>>()?;
            
    // Store the class details.
    r.classes.push( CoClass::new(
            clsid,
            format!( "{}", ident ),
            interfaces ) );

    Ok(())
}

/// Process a trait.
///
/// Most of the processing is done in `process_interface`. This method only
/// takes care of the trait-specific bits that don't apply to the case where
/// the interface is defined with an impl.
pub fn process_trait( 
    ident : &syn::Ident,
    attrs : &[syn::Attribute],
    items : &[syn::TraitItem],
    r: &mut ParseResult,
    rn : &HashMap<String, String>,
) -> Result<(), AppError> {

    // Get the interface details from the Trait item for the process_interface
    // call.
    let mut methods = vec![];
    for item in items {
        methods.push( match item.node {
            syn::TraitItemKind::Method( ref method, .. )
                    => ( &item.ident, method ),
            _ => continue,
        } );
    }

    // Process the interface.
    process_interface( ident, attrs, methods, r, rn )
}

/// Process an impl.
///
/// Most of the processing is done in `process_interface`. This method only
/// takes care of the impl-specific bits that don't apply to the case where
/// the interface is defined with a trait.
pub fn process_impl( 
    ident : &syn::Ident,
    attrs : &[syn::Attribute],
    items : &[syn::ImplItem],
    r: &mut ParseResult,
    rn : &HashMap<String, String>,
) -> Result<(), AppError> {

    // Get the interface details from the impl item for the process_interface
    // call.
    let mut methods = vec![];
    for item in items {
        methods.push( match item.node {
            syn::ImplItemKind::Method( ref method, .. )
                    => ( &item.ident, method ),
            _ => continue,
        } );
    }

    // Process the interface.
    process_interface( ident, attrs, methods, r, rn )
}

/// Processes a COM interface.
pub fn process_interface( 
    ident : &syn::Ident,
    attrs : &[syn::Attribute],
    items : Vec<( &syn::Ident, &syn::MethodSig )>,
    r: &mut ParseResult,
    rn : &HashMap<String, String>,
) -> Result<(), AppError> {

    // Ensure we are dealign with a [com_interface] instead of some other item.
    // If this isn't a [com_interface] we can escape early.
    let interface_attr = match get_attribute( "com_interface", attrs ) {
        Some( a ) => a,
        _ => return Ok(())
    };

    // Parse [com_interface(..)] parameters.
    let params = utils::get_parameters( interface_attr )
            .ok_or_else( || format!( "Bad parameters on com_interface on {}", ident ) )?;
    let guid_param = params.first()
            .ok_or_else( || format!( "Not enough com_interface parameters on {}", ident ) )?;

    // Get the interface IID.
    let iid = get_guid( r, &GUIDType::IID, ident, guid_param )?;

    // Process the methods.
    let methods = get_com_methods( rn, items )?;
            
    // Insert the new interface definition in the results.
    r.interfaces.push( Interface::new(
            iid,
            format!( "{}", ident ),
            methods ) );
    Ok(())
}

/// Gets a specific attribute from a list of attributes.
fn get_attribute<'a, 'b>(
    name : &'b str,
    attrs : &'a [syn::Attribute]
) -> Option< &'a syn::Attribute >
{
    attrs.iter().find( |attr| attr.value.name() == name )
}

/// Converts syn method definitions into `ComMethod` data structures.
fn get_com_methods(
    rn : &HashMap<String, String>,
    methods : Vec<( &syn::Ident, &syn::MethodSig )>
) -> Result<Vec<ComMethod>, AppError> {

    // Process all methods.
    let mut v = vec![];
    for ( ident, method ) in methods {

        // Rust should use &self as the first parameter. Split it from the
        // parameter inputs.
        let ( self_arg, other_args ) = match method.decl.inputs.split_first() {
            Some( ( s, other ) ) => ( s, other ),

            // Getting first fails if there are no arguments. This means no
            // 'self' argument, thus not a proper instance method.
            _ => continue,
        };

        // Only self by reference is supported. COM never transfer ownership.
        let mutability = match *self_arg {
            syn::FnArg::SelfRef( _, m ) => match m {
                syn::Mutability::Mutable => Mutability::Mutable,
                syn::Mutability::Immutable => Mutability::Immutable,
            },

            // All other cases are either non-borrowed self values: Either
            // self-by-ownership or non-self parameters in case of static
            // methods. All of these result in the method being skipped.
            //
            // NOTE: Due to new requirements for interfaces, we might need to
            //       error out on these cases. If there are "unsupported"
            //       methods in the traits, we can't support Rust-to-COM calls
            //       for these. However for implicit impls we don't support
            //       Rust-to-COM calls anyway currently and things like
            //       new-constructors, etc. would fall under this case.
            //
            //       Need to consider this in the future.
            _ => continue,
        };

        // Get COM arguments.
        let mut args = match get_com_args( rn, other_args ) {
            Ok(v) => v,
            Err(e) => { eprintln!( "{}", e ); continue },
        };

        // Figure out the return type.
        //
        // We should migrate this to the ComMethodInfo/ReturnHandler infra at
        // some point in the future.
        let rvalue = match method.decl.output {

            // "Default" return type means the type was omitted, thus there is
            // no return value.
            syn::FunctionRetTy::Default => "void".to_owned(),

            // Return type is defined.
            syn::FunctionRetTy::Ty( ref ty ) => {

                // Convert the return type into [retval] and COM return type.
                let ( out_ty_opt, ret_ty ) = utils::get_ret_types( ty )?;

                // If there was a [retval] out-value, add that in the arguemnts.
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
                
                // Return the COM equivalent for the return type.
                get_com_ty( rn, &ret_ty )?
            }
        };

        // Add the ComMethod details in the result vector.
        v.push( ComMethod {
            name : format!( "{}", ident ),
            mutability: mutability,
            arguments: args,
            rvalue: rvalue,
        } );
    }

    Ok( v )
}

/// Gets COM arguments for a method.
fn get_com_args(
    rn : &HashMap<String, String>,
    args : &[syn::FnArg]
) -> Result< Vec<MethodArg>, AppError > {

    // Gather all arguments.
    let mut v = vec![];
    for arg in args {

        // We have already skipped the first self-argument here.
        // All the remaining arguments must be defined properly.
        let ( pat, ty ) = match *arg {
            syn::FnArg::Captured( ref pat, ref ty ) => ( pat, ty ),
            _ => Err( format!( "Unsupported argument type: {:?}", arg ) )?,
        };

        // Currently we only support the simple arguments. No destructuring or
        // other arguemnt types are allowed.
        let ident = match *pat {
            syn::Pat::Ident( _, ref ident, _ ) => ident,
            _ => Err( format!( "Unsupported argument pattern: {:?}", pat ) )?,
        };

        // Argument type.
        let idl_ty = get_com_ty( rn, ty )?;

        // Add the method argument in the result vector.
        v.push( MethodArg {
            name: ident.to_string(),
            dir: ArgDirection::In, 
            ty: idl_ty
        } );
    }
    Ok( v )
}

/// Gets the COM type for a Rust type.
fn get_com_ty(
    rn : &HashMap<String, String>,
    ty : &syn::Ty,
) -> Result< String, AppError > {

    Ok( match *ty {

        // Pointer types.
        syn::Ty::Slice( ref ty )
            => format!( "*{}", get_com_ty( rn, ty )? ),
        syn::Ty::Ptr( ref mutty )
            | syn::Ty::Rptr( .., ref mutty )
            => match mutty.mutability {
                syn::Mutability::Mutable
                    => format!( "*{}", get_com_ty( rn, &mutty.ty )? ),
                syn::Mutability::Immutable
                    => format!( "*const {}", get_com_ty( rn, &mutty.ty )? ),
            },

        // This is quite experimental. Do IDLs even support staticly sized
        // arrays? Currently this turns [u8; 3] into "uint8[3]" IDL type.
        syn::Ty::Array( ref ty, ref count )
            => format!( "{}[{:?}]", get_com_ty( rn, ty.as_ref() )?, count ),

        // Normal Rust struct/trait type.
        syn::Ty::Path(.., ref path )
            => path_to_ty( rn, path )?,

        // Tuple with length 0, ie. Unit tuple: (). This is void-equivalent.
        syn::Ty::Tup( ref l ) if l.is_empty()
            => "void".to_owned(),

        syn::Ty::BareFn(..)
            | syn::Ty::Never
            | syn::Ty::Tup(..)
            | syn::Ty::TraitObject(..)
            | syn::Ty::ImplTrait(..)
            | syn::Ty::Paren(..)
            | syn::Ty::Infer
            | syn::Ty::Mac(..)
            => Err( format!( "Argument type not supported: {:?}", ty ) )?,
    } )
}

/// Gets the Ty-name from a path.
///
/// Essentially extracts the last Path segment.
fn path_to_ty(
    rn : &HashMap<String, String>,
    path : &syn::Path,
) -> Result< String, AppError >
{
    let &syn::Path { ref segments, .. } = path;
    segment_to_ty( rn, segments.last().unwrap() )
}

/// Converts a path segment to a Ty.
///
/// The path segment should be the last segment for this to make any sense.
fn segment_to_ty(
    rn : &HashMap<String, String>,
    segment : &syn::PathSegment,
) -> Result< String, AppError > {

    // Get the segment as a string.
    let ty = format!( "{}", segment.ident );

    // Get the type information.
    let args = match segment.parameters {
        syn::PathParameters::AngleBracketed( ref data )
                => &data.types,

        // Parenthesized path parameters should be valid only for Fn-types.
        // These types are unsupported, but we'll match for them here anyway.
        syn::PathParameters::Parenthesized( ref data )
                => &data.inputs,
    };

    Ok( match ty.as_str() {
        
        // Hardcoded handling for parameter types.
        "ComRc" | "ComItf"
            => format!( "{}*", get_com_ty( rn, &args[0] )? ),
        "String" => "BSTR".to_owned(),
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

        // Default handling checks if we need to rename the type, such as
        // Foo -> IFoo for implicit interfaces.
        t => try_rename( rn, t ),
    } )
}

/// Tries to apply renaming to the name.
pub fn try_rename(
    rn : &HashMap<String, String>,
    name : &str
) -> String
{
    if let Some( n ) = rn.get( name ) {
        // Rename valid. Return the renamed name.
        n.to_owned()
    } else {
        // No rename specified. Return the original name.
        name.to_owned()
    }
}

