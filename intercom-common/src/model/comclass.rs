use super::macros::*;
use super::*;
use crate::prelude::*;

use crate::guid::GUID;
use syn::{Generics, Path, Visibility};

intercom_attribute!(
    ComClassAttr<ComClassAttrParam, Path> {
        clsid : StrOption,
    }
);

/// Details of a struct marked with `#[com_class]` attribute.
#[derive(Debug, PartialEq, Eq)]
pub struct ComClass
{
    pub name: Ident,
    pub clsid: Option<GUID>,
    pub visibility: Visibility,
    pub interfaces: Vec<Path>,
    pub generics: Generics,
}

impl ComClass
{
    /// Creates ComClass from AST elements.
    pub fn parse(
        crate_name: &str,
        attr_params: TokenStream,
        item: TokenStream,
    ) -> ParseResult<ComClass>
    {
        // Parse the inputs.
        let item: ::syn::ItemStruct = ::syn::parse2(item)
            .map_err(|_| ParseError::ComClass("<Unknown>".into(), "Item syntax error".into()))?;

        let attr: ComClassAttr = ::syn::parse2(attr_params).map_err(|e| {
            ParseError::ComClass(
                item.ident.to_string(),
                format!("Attribute syntax error: {}", e),
            )
        })?;

        // First attribute parameter is the CLSID. Parse it.
        let clsid_attr = attr
            .clsid()
            .map_err(|msg| ParseError::ComClass(item.ident.to_string(), msg))?;
        let clsid = match clsid_attr {
            None => Some(crate::utils::generate_clsid(
                crate_name,
                &item.ident.to_string(),
            )),
            Some(StrOption::Str(clsid)) => Some(GUID::parse(&clsid.value()).map_err(|_| {
                ParseError::ComClass(item.ident.to_string(), "Bad CLSID format".into())
            })?),
            Some(StrOption::None) => None,
        };

        // Remaining parameters are coclasses.
        let name = item.ident.clone();
        let interfaces = attr
            .args()
            .into_iter()
            .map(|itf| match itf.get_ident() {
                Some(ident) if ident == "Self" => parse_quote!(#name),
                _ => itf.clone(),
            })
            .collect();

        Ok(ComClass {
            visibility: item.vis.clone(),
            generics: item.generics,
            name,
            clsid,
            interfaces,
        })
    }

    /// Figure out whether the path refers to the current struct.
    pub fn is_self_path(&self, path: &Path) -> bool
    {
        // The self type must be specified as ident. We can't resolve
        // the current path of the com_class so we can't figure out whether
        // `foo::bar::ThisStruct` _really_ is `ThisStruct`.
        if let Some(ident) = path.get_ident() {
            // The self type can be specified either with the type name or
            // by using 'Self' as type name.
            if ident == &self.name || ident == "Self" {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_class()
    {
        let cls = ComClass::parse(
            "not used",
            quote!(clsid = "12345678-1234-1234-1234-567890ABCDEF", Foo, Bar),
            quote!(
                struct S;
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "S");
        assert_eq!(
            cls.clsid,
            Some(GUID::parse("12345678-1234-1234-1234-567890ABCDEF").unwrap())
        );
        assert_eq!(cls.interfaces.len(), 2);
        assert_eq!(cls.interfaces[0], parse_quote!(Foo));
        assert_eq!(cls.interfaces[1], parse_quote!(Bar));
    }

    #[test]
    fn parse_com_class_with_auto_guid()
    {
        // This test derives the GUID from the library name.
        //
        // What the final GUID is isn't important, what _is_ important however
        // is that the final GUID will not change ever as long as the library
        // name stays the same.
        let cls = ComClass::parse(
            "not used",
            quote!(MyStruct, IThings, IStuff),
            quote!(
                struct MyStruct
                {
                    a: u32,
                }
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "MyStruct");
        assert_eq!(
            cls.clsid,
            Some(GUID::parse("28F57CBA-6AF4-3D3F-7C55-1CF1394D5C7A").unwrap())
        );
        assert_eq!(cls.interfaces.len(), 3);
        assert_eq!(cls.interfaces[0], parse_quote!(MyStruct));
        assert_eq!(cls.interfaces[1], parse_quote!(IThings));
        assert_eq!(cls.interfaces[2], parse_quote!(IStuff));
    }

    #[test]
    fn parse_com_class_with_no_data()
    {
        let cls = ComClass::parse(
            "not used",
            quote!(clsid = None),
            quote!(
                struct EmptyType;
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "EmptyType");
        assert_eq!(cls.clsid, None);
        assert_eq!(cls.interfaces.len(), 0);
    }

    #[test]
    fn parse_com_class_with_no_guid_with_interface()
    {
        let cls = ComClass::parse(
            "not used",
            quote!(clsid = None, ITestInterface),
            quote!(
                struct EmptyType;
            ),
        )
        .expect("com_class attribute parsing failed");

        assert_eq!(cls.name, "EmptyType");
        assert_eq!(cls.clsid, None);
        assert_eq!(cls.interfaces.len(), 1);
    }
}
