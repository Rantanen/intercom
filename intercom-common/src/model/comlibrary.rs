
use ::prelude::*;
use super::*;
use super::macros::*;

use ::guid::GUID;
use ::ast_converters::*;
use ::syn::{ LitStr };

intercom_attribute!(
    ComLibraryAttr< ComLibraryAttrParam, Ident > {
        libid : LitStr,
    }
);

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
        attr_params : TokenStream,
    ) -> ParseResult<ComLibrary>
    {
        let attr : ComLibraryAttr = ::syn::parse2( attr_params )
            .map_err( |_| ParseError::ComLibrary(
                    "Attribute syntax error".into() ) )?;

        // The first parameter is the LIBID of the library.
        let libid = match attr.libid().map_err( ParseError::ComLibrary )? {
            Some( libid ) => GUID::parse( &libid.value() ) 
                    .map_err( ParseError::ComLibrary )?,
            None => ::utils::generate_libid( crate_name )
        } ;

        Ok( ComLibrary {
            name: crate_name.to_owned(),
            coclasses: attr.args().into_iter().cloned().collect(),
            libid,
        } )
    }

    /// Library name.
    pub fn name( &self ) -> &str { &self.name }

    /// Library LIBID.
    pub fn libid( &self ) -> &GUID { &self.libid }

    /// CoClasses exposed by the library.
    pub fn coclasses( &self ) -> &[Ident] { &self.coclasses }

    /// Adds a coclass.
    pub fn add_coclass( &mut self, clsid : Ident ) { self.coclasses.push( clsid ) }
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
