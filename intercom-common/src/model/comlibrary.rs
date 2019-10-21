use super::*;
use crate::prelude::*;

use crate::guid::GUID;
use ::syn::{LitStr, Path};

intercom_attribute!(
    ComLibraryAttr< ComLibraryAttrParam, Path > {
        libid : LitStr,
    }
);

const EMPTY: [Path; 0] = [];

/// COM library details derived from the `com_library` attribute.
#[derive(Debug, PartialEq)]
pub struct ComLibrary
{
    name: String,
    libid: GUID,
    coclasses: Vec<Path>,
}

impl ComLibrary
{
    /// Parses a [com_library] attribute.
    pub fn parse(crate_name: &str, attr_params: TokenStream) -> ParseResult<ComLibrary>
    {
        let attr: ComLibraryAttr = ::syn::parse2(attr_params)
            .map_err(|_| ParseError::ComLibrary("Attribute syntax error".into()))?;

        // The first parameter is the LIBID of the library.
        let libid = match attr.libid().map_err(ParseError::ComLibrary)? {
            Some(libid) => GUID::parse(&libid.value()).map_err(ParseError::ComLibrary)?,
            None => crate::utils::generate_libid(crate_name),
        };

        Ok(ComLibrary {
            name: crate_name.to_owned(),
            coclasses: attr.args().into_iter().cloned().collect(),
            libid,
        })
    }

    /// Library name.
    pub fn name(&self) -> &str
    {
        &self.name
    }

    /// Library LIBID.
    pub fn libid(&self) -> &GUID
    {
        &self.libid
    }

    /// CoClasses exposed by the library.
    pub fn coclasses(&self) -> &[Path]
    {
        &self.coclasses
    }

    /// CoClasses exposed by the library.
    pub fn interfaces(&self) -> &[Path]
    {
        &EMPTY
    }

    /// Adds a coclass.
    pub fn add_coclass(&mut self, clsid: Path)
    {
        self.coclasses.push(clsid)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_library()
    {
        let lib = ComLibrary::parse(
            "library_name".into(),
            quote!(libid = "12345678-1234-1234-1234-567890ABCDEF", Foo, Bar),
        )
        .expect("com_library attribute parsing failed");

        assert_eq!(lib.name(), "library_name");
        assert_eq!(
            lib.libid(),
            &GUID {
                data1: 0x12345678,
                data2: 0x1234,
                data3: 0x1234,
                data4: [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]
            }
        );
        assert_eq!(lib.coclasses().len(), 2);
        assert_eq!(lib.coclasses()[0], parse_quote!(Foo));
        assert_eq!(lib.coclasses()[1], parse_quote!(Bar));
    }

    #[test]
    fn parse_com_library_with_auto_guid()
    {
        // This test derives the GUID from the library name.
        //
        // What the final GUID is isn't important, what _is_ important however
        // is that the final GUID will not change ever as long as the library
        // name stays the same.
        let lib = ComLibrary::parse("another_library".into(), quote!(One, Two))
            .expect("com_library attribute parsing failed");

        assert_eq!(lib.name(), "another_library");
        assert_eq!(
            lib.libid(),
            &GUID::parse("696B2FAE-AC56-3E08-7C2C-ABAA8DB8F6E3").unwrap()
        );
        assert_eq!(lib.coclasses().len(), 2);
        assert_eq!(lib.coclasses()[0], parse_quote!(One));
        assert_eq!(lib.coclasses()[1], parse_quote!(Two));
    }

    #[test]
    fn parse_com_library_with_empty_parameters()
    {
        let lib = ComLibrary::parse("lib".into(), quote!()).unwrap();
        assert_eq!(lib.coclasses().len(), 0);
        assert_eq!(
            lib.libid(),
            &GUID::parse("22EC0095-CD17-3AFD-6C4F-531464178911").unwrap()
        );
    }
}
