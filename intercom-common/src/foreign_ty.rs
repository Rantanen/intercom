extern crate std;

use crate::prelude::*;
use crate::type_parser::*;
use syn;

pub trait ForeignTypeHandler {
    /// Gets the name for the 'ty'.
    fn get_name(&self, ty: &Ident) -> String;

    /// Gets the COM type for a Rust type.
    fn get_ty<'a, 'b: 'a>(&self, ty: &'b syn::Type) -> Option<TypeInfo<'a>>;
}

pub struct CTypeHandler;

impl ForeignTypeHandler for CTypeHandler {
    /// Tries to apply renaming to the name.
    fn get_name(&self, ident: &Ident) -> String {
        self.get_name_for_ty(&ident.to_string())
    }

    fn get_ty<'a, 'b: 'a>(&self, ty: &'b syn::Type) -> Option<TypeInfo<'a>> {
        crate::type_parser::parse(ty)
    }
}

impl CTypeHandler {
    fn get_name_for_ty(&self, ty_name: &str) -> String {
        ty_name.to_owned()
    }
}
