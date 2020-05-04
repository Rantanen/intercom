use super::*;
use crate::prelude::*;

use crate::guid::GUID;
use syn::{LitStr, Path};

#[derive(Debug, Clone)]
pub enum LibraryItemType
{
    Module(Path),
    Class(Path),
    Interface(Path),
}

impl syn::parse::Parse for LibraryItemType
{
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self>
    {
        let ident: syn::Ident = input.parse()?;
        match ident.to_string().as_str() {
            "module" => Ok(LibraryItemType::Module(input.parse()?)),
            "class" => Ok(LibraryItemType::Class(input.parse()?)),
            "interface" => Ok(LibraryItemType::Interface(input.parse()?)),
            _ => Err(input.error(&format!(
                "Expected 'class', 'interface' or 'module', found {}",
                ident
            ))),
        }
    }
}

intercom_attribute!(
    ComLibraryAttr< ComLibraryAttrParam, LibraryItemType > {
        libid : LitStr,
        on_load : Path,
    }
);

/// COM library details derived from the `com_library` attribute.
#[derive(Debug, PartialEq)]
pub struct ComLibrary
{
    pub name: String,
    pub libid: GUID,
    pub on_load: Option<Path>,
    pub coclasses: Vec<Path>,
    pub interfaces: Vec<Path>,
    pub submodules: Vec<Path>,
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

        let on_load = attr.on_load().map_err(ParseError::ComLibrary)?.cloned();

        let mut coclasses = vec![];
        let mut interfaces = vec![];
        let mut submodules = vec![];
        for arg in attr.args().into_iter().cloned() {
            match arg {
                LibraryItemType::Class(cls) => coclasses.push(cls),
                LibraryItemType::Interface(cls) => interfaces.push(cls),
                LibraryItemType::Module(cls) => submodules.push(cls),
            }
        }

        Ok(ComLibrary {
            name: crate_name.to_owned(),
            on_load,
            coclasses,
            interfaces,
            submodules,
            libid,
        })
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
            quote!(
                libid = "12345678-1234-1234-1234-567890ABCDEF",
                class Foo,
                class Bar),
        )
        .expect("com_library attribute parsing failed");

        assert_eq!(lib.name, "library_name");
        assert_eq!(
            lib.libid,
            GUID {
                data1: 0x12345678,
                data2: 0x1234,
                data3: 0x1234,
                data4: [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF]
            }
        );
        assert_eq!(lib.coclasses.len(), 2);
        assert_eq!(lib.coclasses[0], parse_quote!(Foo));
        assert_eq!(lib.coclasses[1], parse_quote!(Bar));
    }

    #[test]
    fn parse_com_library_with_auto_guid()
    {
        // This test derives the GUID from the library name.
        //
        // What the final GUID is isn't important, what _is_ important however
        // is that the final GUID will not change ever as long as the library
        // name stays the same.
        let lib = ComLibrary::parse("another_library".into(), quote!(class One, class Two))
            .expect("com_library attribute parsing failed");

        assert_eq!(lib.name, "another_library");
        assert_eq!(
            lib.libid,
            GUID::parse("696B2FAE-AC56-3E08-7C2C-ABAA8DB8F6E3").unwrap()
        );
        assert_eq!(lib.coclasses.len(), 2);
        assert_eq!(lib.coclasses[0], parse_quote!(One));
        assert_eq!(lib.coclasses[1], parse_quote!(Two));
    }

    #[test]
    fn parse_com_library_with_empty_parameters()
    {
        let lib = ComLibrary::parse("lib".into(), quote!()).unwrap();
        assert_eq!(lib.coclasses.len(), 0);
        assert_eq!(
            lib.libid,
            GUID::parse("22EC0095-CD17-3AFD-6C4F-531464178911").unwrap()
        );
    }
}
