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
use ::syn::{Ident, Visibility};

#[derive(Debug)]
pub struct ParseError( pub String );
type ParseResult<T> = Result< T, ParseError >;

impl ParseError
{
    pub fn new( s: &str ) -> ParseError { ParseError( s.to_owned() ) }
}

/// COM library details derived from the `com_library` attribute.
pub struct ComLibrary {
    name : String,
    libid : GUID,
    coclasses : Vec<Ident>,
}

impl ComLibrary
{
    pub fn new(
        name : String,
        libid : GUID,
        coclasses : Vec<Ident>
    ) -> ComLibrary
    {
        ComLibrary { name, libid, coclasses }
    }

    pub fn parse(
        crate_name : &str,
        attr_params : &str
    ) -> ParseResult<ComLibrary>
    {
        let attr = ::utils::parse_attr_tokens( "com_library", attr_params )
            .map_err( |_| ParseError::new( "Could not parse attribute" ) )?;
        let mut iter = ::utils::iter_parameters( &attr );

        let libid = ::utils::parameter_to_guid(
                &iter.next()
                    .ok_or_else( || ParseError::new( "No LIBID specified" ) )?,
                crate_name, "", "LIBID" )
            .map_err( |_| ParseError::new( "Could not parse CLSID" ) )?
            .ok_or_else( || ParseError::new( "COM library must have a non-zero LIBID" ) )?;

        let coclasses : Vec<Ident> = iter
                .map( |coclass| coclass.get_ident().map( |i| i.clone() ) )
                .collect::<Result<_,_>>()
                .map_err( |_| ParseError::new( "Could not parse coclass" ) )?;

        Ok( ComLibrary {
            name: crate_name.to_owned(),
            libid,
            coclasses
        } )
    }

    pub fn name( &self ) -> &str { &self.name }
    pub fn libid( &self ) -> &GUID { &self.libid }
    pub fn coclasses( &self ) -> &[Ident] { &self.coclasses }
}

pub struct ComStruct
{
    name : Ident,
    clsid : Option<GUID>,
    visibility : Visibility,
    interfaces : Vec<Ident>,
}

impl ComStruct
{
    pub fn new(
        name: Ident,
        clsid: Option<GUID>,
        visibility : Visibility,
        interfaces: Vec<Ident>
    ) -> ComStruct
    {
        ComStruct { name, clsid, visibility, interfaces }
    }

    pub fn parse(
        crate_name : &str,
        attr_params : &str,
        item : &str,
    ) -> ParseResult< ComStruct >
    {
        let ( _, attr, item ) =
                ::utils::parse_inputs( "com_class", attr_params, item )
                    .map_err( |e| ParseError( e.msg ) )?;
        let struct_ident = ::utils::get_struct_ident_from_annotatable( &item );

        let mut iter = ::utils::iter_parameters( &attr );
        let clsid = ::utils::parameter_to_guid(
                &iter.next()
                    .ok_or_else( || ParseError::new( "No CLSID specified" ) )?,
                crate_name, struct_ident.as_ref(), "CLSID" )
            .map_err( |_| ParseError::new( "Could not parse CLSID" ) )?;

        let interfaces : Vec<Ident> = iter
                .map( |itf| itf.get_ident().map( |i| i.clone() ) )
                .collect::<Result<_,_>>()
                .map_err( |_| ParseError::new( "Could not parse interface" ) )?;

        Ok( ComStruct {
            name: struct_ident.clone(),
            visibility: item.vis.clone(),
            clsid,
            interfaces
        } )
    }

    pub fn name( &self ) -> &Ident { &self.name }
    pub fn clsid( &self ) -> &Option<GUID> { &self.clsid }
    pub fn visibility( &self ) -> &::syn::Visibility { &self.visibility }
    pub fn interfaces( &self ) -> &[Ident] { &self.interfaces }
}

pub struct ComInterface
{
    name : Ident,
    iid : GUID,
    visibility : Visibility,
    base_interface : Option<Ident>,
    methods : Vec<ComMethodInfo>,
    item_type: ::utils::InterfaceType,
}

impl ComInterface
{
    pub fn new(
        name: Ident,
        iid: GUID,
        visibility: Visibility,
        base_interface: Option<Ident>,
        methods: Vec<ComMethodInfo>,
        item_type: ::utils::InterfaceType,
    ) -> ComInterface
    {
        ComInterface { name, visibility, base_interface, iid, methods, item_type }
    }

    pub fn parse(
        crate_name : &str,
        attr_params : &str,
        item : &str,
    ) -> ParseResult<ComInterface>
    {
        let ( _, attr, item ) =
                ::utils::parse_inputs( "com_class", attr_params, item )
                    .map_err( |e| ParseError( e.msg ) )?;
        let ( itf_ident, fns, itf_type ) =
                ::utils::get_ident_and_fns( &item )
                    .ok_or_else( || ParseError::new(
                            "[com_interface(IID:&str)] must be applied to trait \
                            or struct impl" ) )?;

        let mut iter = ::utils::iter_parameters( &attr );
        let iid = ::utils::parameter_to_guid(
                    &iter.next()
                        .ok_or_else( || ParseError::new( "No IID specified" ) )?,
                    crate_name, itf_ident.as_ref(), "IID" )
                .map_err( |_| ParseError::new( "Could not parse IID" ) )?
                .ok_or_else( || ParseError::new( "COM interfaces must have non-zero IID" ) )?;

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

        let base = iter.next()
                .map( |base| base.get_ident()
                    .map( |i| i.clone() )
                    .map_err( |_| ParseError::new( "Invalid base interface" ) ) )
                .map_or( Ok(None), |o| o.map(Some) )?
                .unwrap_or_else( || "IUnknown".into() );
        let base = if base == "NO_BASE" { None } else { Some( base ) };

        // TODO: This bit should filter the methods 
        let methods = fns.into_iter()
            .map( |( ident, sig )|
                ComMethodInfo::new( ident, &sig.decl ).map_err( |_| ident ) )
            .filter_map( |r| r.ok() )
            .collect::<Vec<_>>();

        Ok( ComInterface {
            name: itf_ident.clone(),
            iid: iid,
            visibility: visibility,
            base_interface: base,
            methods: methods,
            item_type: itf_type,
        } )
    }

    pub fn name( &self ) -> &Ident { &self.name }
    pub fn iid( &self ) -> &GUID { &self.iid }
    pub fn visibility( &self ) -> &Visibility { &self.visibility }
    pub fn base_interface( &self ) -> &Option<Ident> { &self.base_interface }
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }
    pub fn item_type( &self ) -> ::utils::InterfaceType { self.item_type }
}

pub struct ComImpl
{
    struct_name : Ident,
    interface_name : Ident,
    is_trait_impl : bool,
    methods : Vec<ComMethodInfo>,
}

impl ComImpl
{
    pub fn new(
        struct_name: Ident,
        interface_name: Ident,
        is_trait_impl: bool,
        methods: Vec<ComMethodInfo>,
    ) -> ComImpl
    {
        ComImpl { struct_name, interface_name, is_trait_impl, methods }
    }

    pub fn parse(
        item : &str,
    ) -> ParseResult<ComImpl>
    {
        let item = ::syn::parse_item( item )
                .map_err( |_| ParseError::new( "Could not parse [com_impl]" ) )?;
        let ( itf_ident_opt, struct_ident, fns ) =
                ::utils::get_impl_data( &item )
                    .ok_or_else( || ParseError::new(
                        "[com_impl] must be applied to an impl" ) )?;

        // TODO: This bit should filter the methods 
        let methods = fns.into_iter()
            .map( |( ident, sig )|
                ComMethodInfo::new( ident, &sig.decl ).map_err( |_| ident ) )
            .filter_map( |r| r.ok() )
            .collect::<Vec<_>>();

        Ok( ComImpl {
            struct_name: struct_ident.clone(),
            interface_name: itf_ident_opt
                    .map_or( struct_ident.clone(), |i| i.clone() ),
            is_trait_impl: itf_ident_opt.is_some(),
            methods: methods,
        } )
    }

    pub fn struct_name( &self ) -> &Ident { &self.struct_name }
    pub fn interface_name( &self ) -> &Ident { &self.interface_name }
    pub fn is_trait_impl( &self ) -> bool { self.is_trait_impl }
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }
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
}
