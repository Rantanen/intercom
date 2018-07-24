use ::guid::GUID;
use ::ast_converters::*;
use ::builtin_model;
use ::syn::{Ident};

use super::*;

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

    /// Creates ComLibrary from AST elements.
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
                .map( |coclass| coclass.get_ident() )
                .collect::<Result<_,_>>()
                .map_err( |_| ParseError::ComLibrary( "Bad class name".into() ) )?;

        Ok( ComLibrary {
            name: crate_name.to_owned(),
            libid,
            coclasses,
        } )
    }

    /// Injects built-in types to the library.
    pub fn inject_built_in(
        &mut self,
        built_in_types: &[builtin_model::BuiltinTypeInfo]
    )
    {
        let mut built_in_classes: Vec<Ident> = built_in_types.iter().filter_map( |bti|
                    if bti.class.clsid().is_some() {
                        Some( bti.class.name() )
                    } else {
                        None
                    } ).collect();
        self.coclasses.append( &mut built_in_classes );
    }

    /// Library name.
    pub fn name( &self ) -> &str { &self.name }

    /// Library LIBID.
    pub fn libid( &self ) -> &GUID { &self.libid }

    /// CoClasses exposed by the library.
    pub fn coclasses( &self ) -> &[Ident] { &self.coclasses }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_library() {

        let lib = ComLibrary::parse(
            "library_name".into(),
            r#""12345678-1234-1234-1234-567890ABCDEF", Foo, Bar"# )
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
            "AUTO_GUID, One, Two" )
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

        let lib = ComLibrary::parse( "lib".into(), "AUTO_GUID" ).unwrap();
        assert_eq!( lib.coclasses().len(), 0 );
    }

    #[test]
    fn parse_com_library_with_empty_parameters() {

        let result = ComLibrary::parse( "lib".into(), "()" );
        assert!( result.is_err() );
    }
}
