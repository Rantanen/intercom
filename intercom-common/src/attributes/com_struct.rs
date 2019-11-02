use super::common::*;
use crate::model::{self, ComStruct};
use crate::prelude::*;
use crate::tyhandlers::ModelTypeSystem;
use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Field};

pub fn expand_com_struct(
    attr_tokens: TokenStreamNightly,
    item_tokens: TokenStreamNightly,
) -> Result<TokenStreamNightly, model::ParseError>
{
    let model = ComStruct::parse(&lib_name(), attr_tokens.into(), item_tokens.clone().into())?;

    let mut output = vec![];
    for ts in [ModelTypeSystem::Automation, ModelTypeSystem::Raw].iter() {
        output.extend(generate_extern_struct(&model, *ts));

        let struct_ident = &model.name;
        let ts_tokens = ts.as_typesystem_type(struct_ident.span());
        let extern_ident = Ident::new(&format!("{}_{:?}", model.name, ts), model.name.span());
        output.push(quote!(
            impl intercom::type_system::ExternType<#ts_tokens> for #struct_ident {
                type ExternInputType = #extern_ident;
                type ExternOutputType = #extern_ident;
                type OwnedExternType = #struct_ident;
                type OwnedNativeType = #struct_ident;
            }
        ));
    }

    // Create runtime type info.
    output.push(
        create_get_typeinfo_function(&model)
            .map_err(|e| model::ParseError::ComStruct(model.name.to_string(), e))?,
    );

    Ok(tokens_to_tokenstream(item_tokens, output))
}

fn generate_extern_struct(s: &ComStruct, ts: ModelTypeSystem) -> Vec<TokenStream>
{
    let vis = &s.vis;
    let original_ident = &s.name;
    let ts_type = ts.as_typesystem_type(original_ident.span());
    let extern_ident = Ident::new(&format!("{}_{:?}", s.name, ts), s.name.span());

    let mut extern_fields = s.fields.clone();
    match extern_fields {
        syn::Fields::Named(ref mut fields) => change_types(&mut fields.named, ts),
        syn::Fields::Unnamed(ref mut fields) => change_types(&mut fields.unnamed, ts),
        _ => {}
    }

    let (field_to_native, field_to_extern) : ( Vec<_>, Vec<_> ) = match s.fields {
        syn::Fields::Named(ref fields) => &fields.named,
        syn::Fields::Unnamed(ref fields) => &fields.unnamed,
        _ => panic!("Unit structs are not supported for [com_struct]"),
    }
    .iter()
    .map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;
        (
            quote!( #field_name: <<#field_ty as intercom::type_system::ExternType<#ts_type>>::OwnedNativeType>::intercom_from(src.#field_name)?.intercom_into()? ),
            quote!( #field_name: <<#field_ty as intercom::type_system::ExternType<#ts_type>>::OwnedExternType>::intercom_from(src.#field_name)?.intercom_into()? ),
        )
    })
    .unzip();

    vec![
        quote!(
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #vis struct #extern_ident #extern_fields
        ),
        quote!(impl intercom::type_system::IntercomFrom<#extern_ident> for
            #original_ident {
                unsafe fn intercom_from(src: #extern_ident) -> intercom::ComResult<Self> {
                    use intercom::type_system::IntercomInto;
                    Ok(Self {
                        #( #field_to_native ),*
                    })
                }
            }
        ),
        quote!(impl intercom::type_system::IntercomFrom<#original_ident> for
            #extern_ident {
                unsafe fn intercom_from(src: #original_ident) -> intercom::ComResult<Self> {
                    use intercom::type_system::IntercomInto;
                    Ok(Self {
                        #( #field_to_extern ),*
                    })
                }
            }
        ),
        quote!(impl intercom::type_system::BidirectionalTypeInfo for #extern_ident {
            fn type_name() -> &'static str {
                stringify!(#original_ident)
            }
        }),
    ]
}

fn change_types(fields: &mut Punctuated<Field, Comma>, ts: ModelTypeSystem)
{
    for field in fields.iter_mut() {
        let ty = &field.ty;
        let ts_type = ts.as_typesystem_type(ty.span());
        field.ty = syn::parse2(
            quote_spanned!(field.ty.span() => <#ty as intercom::type_system::ExternType<#ts_type>>::ExternOutputType),
        ).unwrap();
    }
}

fn create_get_typeinfo_function(model: &model::ComStruct) -> Result<TokenStream, String>
{
    let struct_name = model.name.to_string();
    let mut variant_tokens = vec![];
    for ts in [ModelTypeSystem::Automation, ModelTypeSystem::Raw].iter() {
        variant_tokens.push(create_typeinfo_for_variant(model, *ts)?);
    }

    let struct_ident = &model.name;
    Ok(quote_spanned!(struct_ident.span() =>
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        impl intercom::attributes::StructHasTypeInfo for #struct_ident
        {
            fn gather_type_info() -> Vec<intercom::typelib::TypeInfo>
            {
                let variants = vec![ #( #variant_tokens ),* ];

                vec![ intercom::typelib::TypeInfo::Struct(
                    intercom::ComBox::new( intercom::typelib::Struct {
                        name: #struct_name.into(),
                        variants: variants,
                    })
                ) ]
            }
        }
    ))
}

fn create_typeinfo_for_variant(
    model: &model::ComStruct,
    ts: ModelTypeSystem,
) -> Result<TokenStream, String>
{
    let ts_tokens = ts.as_typesystem_tokens(model.name.span());
    let ts_type = ts.as_typesystem_type(model.name.span());
    let fields = model.fields.iter().enumerate().map(|(i, f)| {
        let field_name = f.ident.clone()
            .map(|f| f.to_string())
            .unwrap_or_else(|| format!("_{}", i));
        let ty = &f.ty;
        let ty = quote!( <#ty as intercom::type_system::ExternType<#ts_type>>::ExternOutputType);
        quote_spanned!(f.ty.span() =>
            intercom::typelib::Arg {
                name: #field_name.into(),
                ty: <#ty as intercom::type_system::OutputTypeInfo>::type_name().into(),
                indirection_level: <#ty as intercom::type_system::OutputTypeInfo>::indirection_level(),
                direction: intercom::typelib::Direction::In,
            })
    }).collect::<Vec<_>>();

    Ok(quote_spanned!(model.name.span() =>
        intercom::ComBox::new( intercom::typelib::StructVariant {
            ts: #ts_tokens,
            fields: vec![ #( #fields ),* ],
        })
    ))
}
