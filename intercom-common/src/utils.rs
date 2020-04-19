use crate::prelude::*;
use crate::tyhandlers::ModelTypeSystem;
use syn::*;

use super::*;
use proc_macro2::Span;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InterfaceType
{
    Trait,
    Struct,
}

pub type InterfaceData<'a> = (
    Path,
    Vec<&'a Signature>,
    InterfaceType,
    Option<Token!(unsafe)>,
);

pub fn get_ident_and_fns(item: &Item) -> Option<InterfaceData>
{
    match *item {
        Item::Impl(ItemImpl {
            ref unsafety,
            ref trait_,
            ref self_ty,
            ref items,
            ..
        }) => {
            let (_, struct_ident, items) = get_impl_data_raw(trait_, self_ty, items);
            Some((struct_ident, items, InterfaceType::Struct, *unsafety))
        }
        Item::Trait(ItemTrait {
            ref ident,
            unsafety,
            ref items,
            ..
        }) => {
            let methods: Option<Vec<&Signature>> =
                items.iter().map(|i| get_trait_method(i)).collect();
            let path = syn::Path::from(ident.clone());

            match methods {
                Some(m) => Some((path, m, InterfaceType::Trait, unsafety)),
                None => None,
            }
        }
        _ => None,
    }
}

pub type ImplData<'a> = (Option<Path>, Path, Vec<&'a Signature>);

fn get_impl_data_raw<'a>(
    trait_ref: &'a Option<(Option<Token!(!)>, Path, Token!(for))>,
    struct_ty: &'a Type,
    items: &'a [ImplItem],
) -> ImplData<'a>
{
    let struct_path = match struct_ty {
        syn::Type::Path(typepath) => {
            if typepath.qself.is_some() {
                panic!("#[com_interface] cannot use associated types");
            }
            typepath.path.clone()
        }
        _ => panic!("#[com_interface] must be defined for Path"),
    };

    let trait_path = trait_ref.as_ref().map(|(_, path, _)| path.clone());

    let methods_opt: Option<Vec<&Signature>> = items.iter().map(|i| get_impl_method(i)).collect();
    let methods = methods_opt.unwrap_or_else(Vec::new);

    (trait_path, struct_path, methods)
}

pub fn get_impl_method(i: &ImplItem) -> Option<&Signature>
{
    match *i {
        ImplItem::Method(ref itm) => Some(&itm.sig),
        _ => None,
    }
}

pub fn get_trait_method(i: &TraitItem) -> Option<&Signature>
{
    match *i {
        TraitItem::Method(ref tim) => Some(&tim.sig),
        _ => None,
    }
}

const AUTO_GUID_BASE: guid::GUID = guid::GUID {
    data1: 0x4449_494C,
    data2: 0xDE1F,
    data3: 0x4525,
    data4: [0xB9, 0x57, 0x89, 0xD6, 0x0C, 0xE9, 0x34, 0x77],
};

pub fn generate_iid(crate_name: &str, item_name: &str, type_system: ModelTypeSystem) -> guid::GUID
{
    generate_guid(
        &[
            "IID",
            crate_name,
            item_name,
            match type_system {
                ModelTypeSystem::Automation => "automation",
                ModelTypeSystem::Raw => "raw",
            },
        ]
        .join(":"),
    )
}

pub fn generate_libid(crate_name: &str) -> guid::GUID
{
    generate_guid(&["LIBID", crate_name].join(":"))
}

pub fn generate_clsid(crate_name: &str, item_name: &str) -> guid::GUID
{
    generate_guid(&["CLSID", crate_name, item_name].join(":"))
}

pub fn generate_guid(key: &str) -> guid::GUID
{
    // Hash the name. The name will be hashed in a form similar to:
    // AUTO_GUID_BASE + "CLSID:random_rust_crate:FooBar"
    let mut hash = sha1::Sha1::new();
    hash.update(AUTO_GUID_BASE.as_bytes());
    hash.update(key.as_bytes());

    let digest = hash.digest();
    let bytes = digest.bytes();

    // Set the GUID bytes according to RFC-4122, section 4.3.
    let time_low: u32 = (u32::from(bytes[0]) << 24)
        + (u32::from(bytes[1]) << 16)
        + (u32::from(bytes[2]) << 8)
        + u32::from(bytes[3]);
    let time_mid: u16 = (u16::from(bytes[4]) << 8) + (u16::from(bytes[5]));
    let time_hi_and_version: u16 =
        (((u16::from(bytes[6]) << 8) + u16::from(bytes[7])) & 0x0fff) | 0x3000;
    let clk_seq_hi_res: u8 = (bytes[8] & 0b0011_1111) | 0b0100_0000;
    let clk_seq_low: u8 = bytes[9];

    guid::GUID {
        data1: time_low,
        data2: time_mid,
        data3: time_hi_and_version,
        data4: [
            clk_seq_hi_res,
            clk_seq_low,
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
        ],
    }
}

pub fn ty_to_string(ty: &syn::Type) -> String
{
    quote!( #ty )
        .to_string()
        .replace(" ", "")
        .replace(",", ", ")
}

pub fn is_unit(tk: &Type) -> bool
{
    if let Type::Tuple(ref t) = *tk {
        t.elems.is_empty()
    } else {
        false
    }
}

pub fn unit_ty(span: Span) -> Type
{
    syn::parse2(quote_spanned!(span => ())).unwrap()
}

pub fn get_guid_tokens(g: &guid::GUID, span: Span) -> TokenStream
{
    let d1 = g.data1;
    let d2 = g.data2;
    let d3 = g.data3;
    let d4_0 = g.data4[0];
    let d4_1 = g.data4[1];
    let d4_2 = g.data4[2];
    let d4_3 = g.data4[3];
    let d4_4 = g.data4[4];
    let d4_5 = g.data4[5];
    let d4_6 = g.data4[6];
    let d4_7 = g.data4[7];
    quote_spanned!(span =>
        intercom::GUID {
            data1: #d1, data2: #d2, data3: #d3,
            data4: [ #d4_0, #d4_1, #d4_2, #d4_3, #d4_4, #d4_5, #d4_6, #d4_7 ]
        }
    )
}

/// Convert the Rust identifier from `snake_case` to `PascalCase`
pub fn pascal_case<T: AsRef<str>>(input: T) -> String
{
    let input = input.as_ref();

    // Allocate the output string. We'll never increase the amount of
    // characters so we can reserve string buffer using the input string length.
    let mut output = String::new();
    output.reserve(input.len());

    // Process each character from the input.
    let mut capitalize = true;
    for c in input.chars() {
        // Check the capitalization requirement.
        if c == '_' {
            // Skip '_' but capitalize the following character.
            capitalize = true;
        } else if capitalize {
            // Capitalize. Add the uppercase characters.
            for c_up in c.to_uppercase() {
                output.push(c_up)
            }

            // No need to capitalize any more.
            capitalize = false;
        } else {
            // No need to capitalize. Just add the character as is.
            output.push(c);
        }
    }
    output
}

#[cfg(test)]
mod test
{
    use super::*;

    /// Tests the `ty_to_string` by converting parameter to Type and back to
    /// String to ensure they equal.
    fn test_ty(ty_str: &str)
    {
        let ty = parse_str(ty_str).unwrap();
        let as_string = ty_to_string(&ty);
        assert_eq!(ty_str, as_string);
    }

    #[test]
    fn path_to_test()
    {
        test_ty("::path::Foo")
    }
    #[test]
    fn generics_to_test()
    {
        test_ty("Result<Foo, Bar>")
    }
    #[test]
    fn unit_to_test()
    {
        test_ty("()")
    }
}
