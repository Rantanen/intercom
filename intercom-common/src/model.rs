///
/// COM library parse model.
///
/// Defines the items constructed from the various COM attributes.
///
/// Should unify COM attribute expansion and crate parsing for IDL/Manifest/etc.
/// purposes in the future.
///

use ::guid::GUID;
use ::ast_converters::*;
use ::methodinfo::ComMethodInfo;
use ::builtin_model;
use ::syn::{Ident, Visibility, Unsafety};
use ::std::path::{Path, PathBuf};
use ::std::fs;
use ::std::io::Read;
use ::ordermap::OrderMap;
use ::std::iter::FromIterator;
use toml;

#[derive(Fail, Debug)]
#[non_exhaustive]
pub enum ParseError {
    #[fail( display = "Parsing [com_library] failed: {}", _0 )]
    ComLibrary( String ),

    #[fail( display = "Parsing [com_class] item {} failed: {}", _0, _1 )]
    ComStruct( String, String ),

    #[fail( display = "Parsing [com_interface] item {} failed: {}", _0, _1 )]
    ComInterface( String, String ),

    #[fail( display = "Parsing [com_impl] {} for {} failed: {}", _0, _1, _2 )]
    ComImpl( String, String, String ),

    #[fail( display = "Processing crate failed: {}", _0 )]
    ComCrate( String ),

    #[fail( display = "Reading TOML failed: {}", _0 )]
    CargoToml( String ),
}

type ParseResult<T> = Result<T, ParseError>;

/// COM library details derived from the `com_library` attribute.
#[derive(Debug, PartialEq)]
pub struct ComLibrary {
    name : String,
    libid : GUID,
    coclasses : Vec<Ident>,
}

impl ComLibrary
{
    /// Parses a [com_library] attribute.
    pub fn parse(
        crate_name : &str,
        attr_params : &str
    ) -> ParseResult<ComLibrary>
    {
        // Parse the attribute parameters into an iterator.
        let attr = ::utils::parse_attr_tokens( "com_library", attr_params )
            .map_err( |_| ParseError::ComLibrary( "Syntax error".into() ) )?;

        Self::from_ast( crate_name, &attr )
    }

    /// Creates ComStruct from AST elements.
    pub fn from_ast(
        crate_name : &str,
        attr : &::syn::Attribute,
    ) -> ParseResult< ComLibrary >
    {
        let mut iter = ::utils::iter_parameters( attr );

        // The first parameter is the LIBID of the library.
        let libid = ::utils::parameter_to_guid(
                &iter.next()
                    .ok_or_else( || ParseError::ComLibrary(
                                        "LIBID required".into() ) )?,
                crate_name, "", "LIBID" )
            .map_err( |_| ParseError::ComLibrary( "Bad LIBID format".into() ) )?
            .ok_or_else( || ParseError::ComLibrary( "LIBID required".into() ) )?;

        // The remaining parameters are coclasses exposed by the library.
        let coclasses : Vec<Ident> = iter
                .map( |coclass| coclass.get_ident().map( |i| i.clone() ) )
                .collect::<Result<_,_>>()
                .map_err( |_| ParseError::ComLibrary( "Bad class name".into() ) )?;

        Ok( ComLibrary {
            name: crate_name.to_owned(),
            libid,
            coclasses,
        } )
    }

    /// Library name.
    pub fn name( &self ) -> &str { &self.name }

    /// Library LIBID.
    pub fn libid( &self ) -> &GUID { &self.libid }

    /// CoClasses exposed by the library.
    pub fn coclasses( &self ) -> &[Ident] { &self.coclasses }
}

/// Details of a struct marked with `#[com_class]` attribute.
#[derive(Debug, PartialEq)]
pub struct ComStruct
{
    name : Ident,
    clsid : Option<GUID>,
    visibility : Visibility,
    interfaces : Vec<Ident>,
}

impl ComStruct
{
    /// Parses a #[com_class] attribute and the associated struct.
    pub fn parse(
        crate_name : &str,
        attr_params : &str,
        item : &str,
    ) -> ParseResult< ComStruct >
    {
        // Parse the inputs.
        let item = ::syn::parse_item( item )
            .map_err( |_| ParseError::ComStruct(
                    "<Unknown>".into(),
                    "Item syntax error".into() ) )?;
        let attr = ::utils::parse_attr_tokens( "com_class", attr_params )
            .map_err( |_| ParseError::ComStruct(
                    item.ident.to_string(),
                    "Attribute syntax error".into() ) )?;

        Self::from_ast( crate_name, &attr, &item )
    }

    /// Creates ComStruct from AST elements.
    pub fn from_ast(
        crate_name : &str,
        attr : &::syn::Attribute,
        item : &::syn::Item,
    ) -> ParseResult< ComStruct >
    {

        // First attribute parameter is the CLSID. Parse it.
        let mut iter = ::utils::iter_parameters( attr );
        let clsid = ::utils::parameter_to_guid(
                &iter.next()
                    .ok_or_else( || ParseError::ComStruct(
                            item.ident.to_string(),
                            "No CLSID specified".into() ) )?,
                crate_name, item.ident.as_ref(), "CLSID" )
            .map_err( |_| ParseError::ComStruct(
                    item.ident.to_string(),
                    "Bad CLSID format".into() ) )?;

        // Remaining parameters are coclasses.
        let interfaces : Vec<Ident> = iter
                .map( |itf| itf.get_ident().map( |i| i.clone() ) )
                .collect::<Result<_,_>>()
                .map_err( |_| ParseError::ComStruct(
                        item.ident.to_string(),
                        "Bad interface name".into() ) )?;

        Ok( ComStruct {
            name: item.ident.clone(),
            visibility: item.vis.clone(),
            clsid,
            interfaces,
        } )
    }

    /// Struct name.
    pub fn name( &self ) -> &Ident { &self.name }

    /// Struct CLSID.
    pub fn clsid( &self ) -> &Option<GUID> { &self.clsid }

    /// Struct visibility.
    pub fn visibility( &self ) -> &::syn::Visibility { &self.visibility }

    /// Interfaces implemented by the struct.
    pub fn interfaces( &self ) -> &[Ident] { &self.interfaces }
}

#[derive(Debug, PartialEq)]
pub struct ComInterface
{
    name : Ident,
    iid : GUID,
    visibility : Visibility,
    base_interface : Option<Ident>,
    methods : Vec<ComMethodInfo>,
    item_type: ::utils::InterfaceType,
    is_unsafe : bool,
}

impl ComInterface
{
    /// Parses a #[com_interface] attribute and the associated item.
    pub fn parse(
        crate_name : &str,
        attr_params : &str,
        item : &str,
    ) -> ParseResult<ComInterface>
    {
        // Parse the input code.
        let item = ::syn::parse_item( item )
            .map_err( |_| ParseError::ComInterface(
                    "<Unknown>".into(),
                    "Item syntax error".into() ) )?;
        let attr = ::utils::parse_attr_tokens( "com_interface", attr_params )
            .map_err( |_| ParseError::ComInterface(
                    item.ident.to_string(),
                    "Attribute syntax error".into() ) )?;

        Self::from_ast( crate_name, &attr, &item )
    }

    /// Creates ComInterface from AST elements.
    pub fn from_ast(
        crate_name : &str,
        attr : &::syn::Attribute,
        item : &::syn::Item,
    ) -> ParseResult< ComInterface >
    {
        // Get the interface details. As [com_interface] can be applied to both
        // impls and traits this handles both of those.
        let ( itf_ident, fns, itf_type, unsafety ) =
                ::utils::get_ident_and_fns( item )
                    .ok_or_else( || ParseError::ComInterface(
                            item.ident.to_string(),
                            "Unsupported associated item".into() ) )?;

        // The first attribute parameter is the IID. Parse that.
        let mut iter = ::utils::iter_parameters( attr );
        let iid = ::utils::parameter_to_guid(
                    &iter.next()
                        .ok_or_else( || ParseError::ComInterface(
                                item.ident.to_string(),
                                "IID required".into() ) )?,
                    crate_name, itf_ident.as_ref(), "IID" )
                .map_err( |_| ParseError::ComInterface(
                        item.ident.to_string(),
                        "Bad IID format".into() ) )?
                .ok_or_else( || ParseError::ComInterface(
                        item.ident.to_string(),
                        "IID required".into() ) )?;

        // The second argument is the optional base class. If there's no base
        // class defined, use IUnknown as the default. The value of NO_BASE will
        // construct an interface that has no base class.
        //
        // In practice the NO_BASE should be used ONLY for the IUnknown itself.
        let base = iter.next()
                .map( |base| base.get_ident()
                    .map( |i| i.clone() )
                    .map_err( |_| ParseError::ComInterface(
                            item.ident.to_string(),
                            "Invalid base interface".into() ) ) )
                .map_or( Ok(None), |o| o.map(Some) )?
                .unwrap_or_else( || "IUnknown".into() );
        let base = if base == "NO_BASE" { None } else { Some( base ) };

        // Visibility for trait interfaces is the visibility of the trait.
        //
        // For implicit interfaces (impl Struct) the visibility is always public.
        // These interfaces should only exist for COM types that are meant to be
        // called from external sources as they can't be impl'd for random ComItf.
        //
        // Note this may conflict with visibility of the actual [com_class], but
        // nothing we can do for this really.
        let visibility = if itf_type == ::utils::InterfaceType::Trait {
                item.vis.clone()
            } else {
                Visibility::Public
            };

        // Read the method details.
        //
        // TODO: Currently we ignore invalid methods. We should probably do
        //       something smarter.
        let methods = fns.into_iter()
            .map( |( ident, sig )|
                ComMethodInfo::new( ident, &sig ) )
            .filter_map( |r| r.ok() )
            .collect::<Vec<_>>();

        Ok( ComInterface {
            name: itf_ident.clone(),
            iid: iid,
            visibility: visibility,
            base_interface: base,
            methods: methods,
            item_type: itf_type,
            is_unsafe : unsafety == Unsafety::Unsafe,
        } )
    }

    /// Interface name.
    pub fn name( &self ) -> &Ident { &self.name }

    /// Interface IID.
    pub fn iid( &self ) -> &GUID { &self.iid }

    /// Interface visibility.
    pub fn visibility( &self ) -> &Visibility { &self.visibility }

    /// The base interface.
    pub fn base_interface( &self ) -> &Option<Ident> { &self.base_interface }

    /// Interface methods.
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }

    /// The type of the associated item for the #[com_interface] attribute.
    ///
    /// Either an impl or a trait.
    pub fn item_type( &self ) -> ::utils::InterfaceType { self.item_type }

    /// True, if the interface requires unsafe impl.
    pub fn is_unsafe( &self ) -> bool { self.is_unsafe }
}

#[derive(Debug, PartialEq)]
pub struct ComImpl
{
    struct_name : Ident,
    interface_name : Ident,
    is_trait_impl : bool,
    methods : Vec<ComMethodInfo>,
}

impl ComImpl
{
    /// Parses the associated item of the #[com_impl] attribute.
    pub fn parse(
        item : &str,
    ) -> ParseResult<ComImpl>
    {
        // Get the item details from the associated item.
        let item = ::syn::parse_item( item )
                .map_err( |_| ParseError::ComImpl(
                        "<Unknown>".into(),
                        "<Unknown>".into(),
                        "Could not parse [com_impl]".into() ) )?;

        Self::from_ast( &item )
    }

    /// Creates ComImpl from AST elements.
    pub fn from_ast(
        item : &::syn::Item,
    ) -> ParseResult< ComImpl >
    {
        let ( itf_ident_opt, struct_ident, fns ) =
                ::utils::get_impl_data( item )
                    .ok_or_else( || ParseError::ComImpl(
                            item.ident.to_string(),
                            "<Unknown>".into(),
                            "Unsupported associated item".into() ) )?;

        // Turn the impl methods into MethodInfo.
        //
        // TODO: Currently we ignore invalid methods. We should probably do
        //       something smarter.
        let methods = fns.into_iter()
            .map( |( ident, sig )|
                ComMethodInfo::new( ident, &sig ).map_err( |_| ident ) )
            .filter_map( |r| r.ok() )
            .collect::<Vec<_>>();

        Ok( ComImpl {
            struct_name: struct_ident.clone(),
            is_trait_impl: itf_ident_opt.is_some(),
            interface_name: itf_ident_opt
                    .unwrap_or_else( || struct_ident.clone() ),
            methods: methods,
        } )
    }

    /// Struct name that the trait is implemented for.
    pub fn struct_name( &self ) -> &Ident { &self.struct_name }

    /// Trait name that is implemented. Struct name if this is an implicit impl.
    pub fn interface_name( &self ) -> &Ident { &self.interface_name }

    /// True if a valid trait is implemented, false for implicit impls.
    pub fn is_trait_impl( &self ) -> bool { self.is_trait_impl }

    /// Implemented methods.
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }
}

#[derive(Debug, PartialEq)]
pub struct ComCrate {
    lib : Option<ComLibrary>,
    interfaces : OrderMap<String, ComInterface>,
    structs : OrderMap<String, ComStruct>,
    impls : Vec<ComImpl>,
    incomplete : bool,
}

#[derive(Default)]
struct ComCrateBuilder {
    pub libs : Vec<ComLibrary>,
    pub interfaces : Vec<ComInterface>,
    pub structs : Vec<ComStruct>,
    pub impls : Vec<ComImpl>,
    pub incomplete : bool,
}

impl ComCrateBuilder {

    pub fn build( self ) -> ParseResult<ComCrate>
    {
        if self.libs.len() > 1 {
            return Err( ParseError::ComLibrary(
                    "Multiple [com_library] attributes".into() ) );
        }

        Ok( ComCrate {
            lib: self.libs.into_iter().next(),
            interfaces: OrderMap::from_iter(
                self.interfaces.into_iter().map( |i| ( i.name().to_string(), i ) ) ),
            structs: OrderMap::from_iter(
                self.structs.into_iter().map( |i| ( i.name().to_string(), i ) ) ),
            impls: self.impls,
            incomplete: self.incomplete,
        } )
    }
}

impl ComCrate
{
    pub fn parse(
        crate_name : &str,
        sources : &[&str]
    ) -> ParseResult<ComCrate>
    {
        let mut builder : ComCrateBuilder = Default::default();

        for bti in builtin_model::builtin_intercom_types( crate_name ) {
            builder.structs.push( bti.class );
            builder.interfaces.push( bti.interface );
            builder.impls.push( bti.implementation );
        }

        for src in sources {
            let krate = ::syn::parse_crate( src )
                .map_err( |_| ParseError::ComCrate(
                        "Failed to parse source".into() ) )?;

            Self::process_crate_items(
                crate_name,
                None,
                &krate.items,
                &mut builder )?;
        }

        builder.build()
    }

    pub fn parse_package(
        crate_path : &Path,
    ) -> ParseResult<ComCrate>
    {
        if crate_path.is_file() {
            Self::parse_cargo_toml( crate_path )
        } else {
            Self::parse_cargo_toml( &crate_path.join( "Cargo.toml" ) )
        }
    }

    pub fn parse_cargo_toml(
        toml_path : &Path,
    ) -> ParseResult<ComCrate>
    {
        let mut f = fs::File::open( toml_path )
                .map_err( |_| ParseError::CargoToml(
                        "Could not open Cargo toml".into() ) )?;
        let mut buf = String::new();
        f.read_to_string( &mut buf )
                .map_err( |_| ParseError::CargoToml(
                        "Could not read Cargo toml".into() ) )?;

        let toml = buf.parse::<toml::Value>()
                .map_err( |_| ParseError::CargoToml(
                        "Could not parse Cargo toml".into() ) )?;
        let root = match toml {
            toml::Value::Table( root ) => root,
            _ => return Err( ParseError::CargoToml(
                        "Invalid TOML root element".into() ) ),
        };

        let lib_name = match root.get( "package" ) {
                    Some( &toml::Value::Table( ref package ) )
                        => match package.get( "name" ) {
                            Some( &toml::Value::String( ref name ) )
                                => name,
                            _ => return Err( ParseError::CargoToml(
                                    "No 'name' parameter under [package]".into() ) ),
                        },
                    _ => return Err( ParseError::CargoToml( 
                            "Could not find [package] in Cargo.toml".into() ) ),
                };

        let rel_lib_path = PathBuf::from( &match root.get( "lib" ) {
                    Some( &toml::Value::Table( ref package ) )
                        => match package.get( "path" ) {
                            Some( &toml::Value::String( ref path ) )
                                => path.clone(),
                            _ => "src/lib.rs".to_owned(),
                        },
                    _ => "src/lib.rs".to_owned(),
                } );
        let lib_path = match toml_path.parent() {
                    Some( p ) => p.join( rel_lib_path ),
                    _ => rel_lib_path
                };

        Self::parse_file( lib_name, &lib_path )
    }

    pub fn parse_file(
        crate_name : &str,
        path : &Path
    ) -> ParseResult<ComCrate>
    {
        let mut builder = Default::default();
        Self::parse_file_internal( crate_name, path, &mut builder )?;
        builder.build()
    }

    fn parse_file_internal(
        crate_name : &str,
        path : &Path,
        b : &mut ComCrateBuilder
    ) -> ParseResult<()>
    {
        let mut f = fs::File::open( path )
                .map_err( |_| ParseError::ComCrate(
                        format!( "Could not open file {}", path.to_string_lossy() ) ) )?;

        let mut buf = String::new();
        f.read_to_string( &mut buf )
                .map_err( |_| ParseError::ComCrate(
                        format!( "Could not read file {}", path.to_string_lossy() ) ) )?;

        let krate = ::syn::parse_crate( &buf )
                .map_err( |_| ParseError::ComCrate(
                        format!( "Failed to parse source {}", path.to_string_lossy() ) ) )?;

        Self::process_crate_items( crate_name, Some( path ), &krate.items, b )
    }

    fn process_crate_items(
        crate_name : &str,
        path : Option< &Path >,
        items : &[ ::syn::Item ], 
        b : &mut ComCrateBuilder,
    ) -> ParseResult<()>
    {
        Self::collect_items( crate_name, items, b )?;

        for item in items {
            let opt_mod_items =
                    if let ::syn::ItemKind::Mod( ref items ) = item.node {
                        items
                    } else {
                        continue;
                    };

            match *opt_mod_items {
                None => {

                    // The mod doesn't have immediate items so this is an
                    // external mod. We need to resolve the file.
                    let path = if let Some( p ) = path { p } else {

                        // No path given. Mark the crate as incomplete as we
                        // couldn't resolve all pieces but return with Ok
                        // result.
                        //
                        // This is a case where we were given file contents
                        // without the caller knowing (or telling) where the
                        // file was located. We can't resolve relative mod-paths
                        // in this case.
                        b.incomplete = true;
                        return Ok(());
                    };

                    // We have couple of options. Find the first one that
                    // matches an existing file.
                    let mut mod_paths = vec![
                        path.parent().unwrap().join( format!( "{}.rs", item.ident ) ),
                        path.parent().unwrap().join( format!( "{}/mod.rs", item.ident ) ),
                    ].into_iter()
                        .filter( |p| p.exists() );

                    let mod_path = mod_paths.next()
                        .ok_or_else( || ParseError::ComCrate(
                                format!( "Could not find mod {}", item.ident ) ) )?;

                    let more = mod_paths.next();
                    if more.is_some() {
                        return Err( ParseError::ComCrate(
                                format!( "Ambiguous mod, both {0}.rs and \
                                          {0}/mod.rs present", item.ident ) ) );
                    }

                    Self::parse_file_internal( crate_name, &mod_path, b )?;
                },
                Some( ref mod_items )
                    => Self::process_crate_items( crate_name, path, mod_items, b )?
            }
        }
        
        Ok(())
    }

    fn collect_items(
        crate_name : &str,
        items : &[ ::syn::Item ], 
        b : &mut ComCrateBuilder,
    ) -> ParseResult<()>
    {
        for item in items {
            for attr in &item.attrs {
                match attr.value.name() {
                    "com_library" =>
                        b.libs.push( ComLibrary::from_ast( crate_name, attr )? ),
                    "com_interface" =>
                        b.interfaces.push( ComInterface::from_ast(
                                crate_name, attr, item )? ),
                    "com_class" =>
                        b.structs.push( ComStruct::from_ast(
                                crate_name, attr, item )? ),
                    "com_impl" =>
                        b.impls.push( ComImpl::from_ast( item )? ),
                    _ => { }
                }
            }
        }

        Ok(())
    }


    pub fn lib( &self ) -> &Option<ComLibrary> { &self.lib }
    pub fn interfaces( &self ) -> &OrderMap<String, ComInterface> { &self.interfaces }
    pub fn structs( &self ) -> &OrderMap<String, ComStruct> { &self.structs }
    pub fn impls( &self ) -> &Vec<ComImpl> { &self.impls }
    pub fn is_incomplete( &self ) -> bool { self.incomplete }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_library() {

        let lib = ComLibrary::parse(
            "library_name".into(),
            r#"( "12345678-1234-1234-1234-567890ABCDEF", Foo, Bar )"# )
                .expect( "com_library attribute parsing failed" );

        assert_eq!( lib.name(), "library_name" );
        assert_eq!( lib.libid(), &GUID {
            data1: 0x12345678,
            data2: 0x1234,
            data3: 0x1234,
            data4: [ 0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF ]
        } );
        assert_eq!( lib.coclasses().len(), 2 );
        assert_eq!( lib.coclasses()[0], "Foo" );
        assert_eq!( lib.coclasses()[1], "Bar" );
    }

    #[test]
    fn parse_com_library_with_auto_guid() {

        // This test derives the GUID from the library name.
        //
        // What the final GUID is isn't important, what _is_ important however
        // is that the final GUID will not change ever as long as the library
        // name stays the same.
        let lib = ComLibrary::parse(
            "another_library".into(),
            "( AUTO_GUID, One, Two )" )
                .expect( "com_library attribute parsing failed" );

        assert_eq!( lib.name(), "another_library" );
        assert_eq!(
                lib.libid(),
                &GUID::parse( "6C6AF0CA-89C3-3467-48F3-37466A58CA22" ).unwrap() );
        assert_eq!( lib.coclasses().len(), 2 );
        assert_eq!( lib.coclasses()[0], "One" );
        assert_eq!( lib.coclasses()[1], "Two" );
    }

    #[test]
    fn parse_com_library_without_coclasses() {

        let lib = ComLibrary::parse( "lib".into(), "( AUTO_GUID )" ).unwrap();
        assert_eq!( lib.coclasses().len(), 0 );
    }

    #[test]
    fn parse_com_library_with_empty_parameters() {

        let result = ComLibrary::parse( "lib".into(), "()" );
        assert!( result.is_err() );
    }

    #[test]
    fn parse_com_class() {
        let cls = ComStruct::parse(
            "not used",
            r#"( "12345678-1234-1234-1234-567890ABCDEF", Foo, Bar )"#,
            "struct S;" )
                .expect( "com_class attribute parsing failed" );

        assert_eq!( cls.name(), "S" );
        assert_eq!( cls.clsid(), &Some(
            GUID::parse( "12345678-1234-1234-1234-567890ABCDEF" ).unwrap() ) );
        assert_eq!( cls.interfaces().len(), 2 );
        assert_eq!( cls.interfaces()[0], "Foo" );
        assert_eq!( cls.interfaces()[1], "Bar" );
    }

    #[test]
    fn parse_com_class_with_auto_guid() {

        // This test derives the GUID from the library name.
        //
        // What the final GUID is isn't important, what _is_ important however
        // is that the final GUID will not change ever as long as the library
        // name stays the same.
        let cls = ComStruct::parse(
            "not used",
            r#"( AUTO_GUID, MyStruct, IThings, IStuff )"#,
            "struct MyStruct { a: u32 }" )
                .expect( "com_class attribute parsing failed" );

        assert_eq!( cls.name(), "MyStruct" );
        assert_eq!( cls.clsid(), &Some(
            GUID::parse( "28F57CBA-6AF4-3D3F-7C55-1CF1394D5C7A" ).unwrap() ) );
        assert_eq!( cls.interfaces().len(), 3 );
        assert_eq!( cls.interfaces()[0], "MyStruct" );
        assert_eq!( cls.interfaces()[1], "IThings" );
        assert_eq!( cls.interfaces()[2], "IStuff" );
    }

    #[test]
    fn parse_com_class_with_no_data() {

        let cls = ComStruct::parse(
            "not used",
            r#"( NO_GUID )"#,
            "struct EmptyType;" )
                .expect( "com_class attribute parsing failed" );

        assert_eq!( cls.name(), "EmptyType" );
        assert_eq!( cls.clsid(), &None );
        assert_eq!( cls.interfaces().len(), 0 );
    }

    #[test]
    fn parse_com_interface() {
        let itf = ComInterface::parse(
            "not used",
            r#"( "12345678-1234-1234-1234-567890ABCDEF" )"#,
            "trait ITrait { fn foo( &self ); fn bar( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "ITrait" );
        assert_eq!( itf.iid(),
            &GUID::parse( "12345678-1234-1234-1234-567890ABCDEF" ).unwrap() );
        assert_eq!( itf.visibility(), &Visibility::Inherited );
        assert_eq!( itf.base_interface().as_ref().unwrap(), "IUnknown" );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "foo" );
        assert_eq!( itf.methods[1].name, "bar" );
    }

    #[test]
    fn parse_com_interface_with_auto_guid() {
        let itf = ComInterface::parse(
            "not used",
            r#"( AUTO_GUID )"#,
            "pub trait IAutoGuid { fn one( &self ); fn two( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "IAutoGuid" );
        assert_eq!( itf.iid(),
            &GUID::parse( "11BA222D-A34B-32BC-4A1F-77157F37803A" ).unwrap() );
        assert_eq!( itf.visibility(), &Visibility::Public );
        assert_eq!( itf.base_interface().as_ref().unwrap(), "IUnknown" );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }


    #[test]
    fn parse_com_interface_with_base_interface() {
        let itf = ComInterface::parse(
            "not used",
            r#"( AUTO_GUID, IBase )"#,
            "pub trait IAutoGuid { fn one( &self ); fn two( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "IAutoGuid" );
        assert_eq!( itf.iid(),
            &GUID::parse( "11BA222D-A34B-32BC-4A1F-77157F37803A" ).unwrap() );
        assert_eq!( itf.visibility(), &Visibility::Public );
        assert_eq!( itf.base_interface().as_ref().unwrap(), "IBase" );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }

    #[test]
    fn parse_com_interface_with_no_base_interface() {
        let itf = ComInterface::parse(
            "not used",
            r#"( AUTO_GUID, NO_BASE )"#,
            "pub trait IAutoGuid { fn one( &self ); fn two( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "IAutoGuid" );
        assert_eq!( itf.iid(),
            &GUID::parse( "11BA222D-A34B-32BC-4A1F-77157F37803A" ).unwrap() );
        assert_eq!( itf.visibility(), &Visibility::Public );
        assert_eq!( itf.base_interface(), &None );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }

    #[test]
    fn parse_com_impl_for_struct() {
        let itf = ComImpl::parse(
            "impl Foo { fn foo( &self ) {} fn bar( &self ) {} }" )
                .expect( "com_impl attribute parsing failed" );

        assert_eq!( itf.struct_name(), "Foo" );
        assert_eq!( itf.interface_name(), "Foo" );
        assert_eq!( itf.is_trait_impl(), false );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "foo" );
        assert_eq!( itf.methods[1].name, "bar" );
    }

    #[test]
    fn parse_com_impl_for_trait() {
        let itf = ComImpl::parse(
            "impl IFoo for Bar { fn one( &self ) {} fn two( &self ) {} }" )
                .expect( "com_impl attribute parsing failed" );

        assert_eq!( itf.struct_name(), "Bar" );
        assert_eq!( itf.interface_name(), "IFoo" );
        assert_eq!( itf.is_trait_impl(), true );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }

    #[test]
    fn parse_crate() {
        let krate = ComCrate::parse( "my_crate", &[
            r#"
                #[com_library( "12345678-1234-1234-1234-567890000000", Foo, Bar )]

                #[com_interface( "12345678-1234-1234-1234-567890000001" )]
                trait IFoo {}

                trait IBar {}
            "#,
            r#"
                #[com_class( "12345678-1234-1234-1234-567890000002", IFoo )]
                struct S;

                #[com_impl]
                impl IFoo for S {}
            "#
        ] ).expect( "Parsing the crate failed" );

        assert!( krate.lib.is_some() );
        assert_eq!( krate.lib.as_ref().unwrap().libid(),
            &GUID::parse( "12345678-1234-1234-1234-567890000000" ).unwrap() );
        assert_eq!( krate.interfaces().len(), 1 );
        assert_eq!( krate.interfaces()[ "IFoo" ].iid(),
            &GUID::parse( "12345678-1234-1234-1234-567890000001" ).unwrap() );

        assert_eq!( krate.structs().len(), 1 );
        assert_eq!( krate.structs()[ "S" ].clsid().as_ref().unwrap(),
            &GUID::parse( "12345678-1234-1234-1234-567890000002" ).unwrap() );

        assert_eq!( krate.impls().len(), 1 );
        assert_eq!( krate.impls()[0].struct_name(), "S" );
        assert_eq!( krate.impls()[0].interface_name(), "IFoo" );
    }

    #[test]
    fn parse_incomplete_crate() {
        let krate = ComCrate::parse( "my_crate", &[
            r#"
                mod foo;
            "#,
        ] ).expect( "Parsing the crate failed" );

        assert!( krate.is_incomplete() );
    }
}
