
use ::prelude::*;
use super::*;

use ::guid::GUID;
use ::ast_converters::*;
use ::syn::{Ident, Visibility};

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
        let item : ::syn::ItemStruct = ::syn::parse_str( item )
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
        item : &::syn::ItemStruct,
    ) -> ParseResult< ComStruct >
    {

        // First attribute parameter is the CLSID. Parse it.
        let mut iter = ::utils::iter_parameters( attr );
        let clsid = ::utils::parameter_to_guid(
                &iter.next()
                    .ok_or_else( || ParseError::ComStruct(
                            item.ident.to_string(),
                            "No CLSID specified".into() ) )?,
                crate_name, item.ident.to_string().as_ref(), "CLSID" )
            .map_err( |_| ParseError::ComStruct(
                    item.ident.to_string(),
                    "Bad CLSID format".into() ) )?;

        // Remaining parameters are coclasses.
        let interfaces : Vec<Ident> = iter
                .map( |itf| itf.get_ident() )
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

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_class() {
        let cls = ComStruct::parse(
            "not used",
            r#""12345678-1234-1234-1234-567890ABCDEF", Foo, Bar"#,
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
            r#"AUTO_GUID, MyStruct, IThings, IStuff"#,
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
            r#"NO_GUID"#,
            "struct EmptyType;" )
                .expect( "com_class attribute parsing failed" );

        assert_eq!( cls.name(), "EmptyType" );
        assert_eq!( cls.clsid(), &None );
        assert_eq!( cls.interfaces().len(), 0 );
    }

    #[test]
    fn parse_com_class_with_no_guid_with_interface() {

        let cls = ComStruct::parse(
            "not used",
            r#"NO_GUID, ITestInterface"#,
            "struct EmptyType;" )
                .expect( "com_class attribute parsing failed" );

        assert_eq!( cls.name(), "EmptyType" );
        assert_eq!( cls.clsid(), &None );
        assert_eq!( cls.interfaces().len(), 1 );
    }
}
