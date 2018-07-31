extern crate std;

use prelude::*;
use model::ComCrate;
use syn;
use type_parser::*;

pub trait ForeignTypeHandler
{
    /// Gets the name for the 'ty'.
    fn get_name( &self, krate : &ComCrate, ty : &Ident ) -> String;

    /// Gets the COM type for a Rust type.
    fn get_ty<'a, 'b: 'a>(
        &self,
        ty : &'b syn::Type,
    ) -> Option< TypeInfo<'a> >;
}

pub struct CTypeHandler;

impl ForeignTypeHandler for CTypeHandler
{
    /// Tries to apply renaming to the name.
    fn get_name(
        &self,
        krate: &ComCrate,
        ident: &Ident,
    ) -> String
    {
        self.get_name_for_ty( krate, &ident.to_string() )
    }

    fn get_ty<'a, 'b: 'a>(
        &self,
        ty: &'b syn::Type,
    ) -> Option<TypeInfo<'a>>
    {
        ::type_parser::parse( ty )
    }
}

impl CTypeHandler
{
     fn get_name_for_ty(
        &self,
        krate : &ComCrate,
        ty_name : &str
    ) -> String
    {
        let itf = if let Some( itf ) = krate.interfaces().get( ty_name ) {
            itf
        } else {
            return ty_name.to_owned()
        };

        if itf.item_type() == ::utils::InterfaceType::Struct {
            format!( "I{}", itf.name() )
        } else {
            ty_name.to_owned()
        }
    }
}
