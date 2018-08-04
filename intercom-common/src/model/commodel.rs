

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
                .map( |coclass| coclass.get_ident() )
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
