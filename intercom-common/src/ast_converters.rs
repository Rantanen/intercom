
use prelude::*;
use syn::*;

/// Extract the underlying Type from various AST types.
pub trait GetType {

    /// Gets the Type from the AST element.
    fn get_ty( &self ) -> Result<Type, String>;
}

impl GetType for FnArg {

    fn get_ty( &self ) -> Result<Type, String>
    {
        Ok( match *self {
            FnArg::Captured( ref c ) => c.ty.clone(),
            FnArg::Ignored( ref ty ) => ty.clone(),
            FnArg::SelfRef( ref s )
                => Type::Reference( TypeReference {
                        and_token: parse_quote!( & ),
                        lifetime: s.lifetime.clone(),
                        mutability: s.mutability,
                        elem: Box::new( parse_quote!( Self ) )
                } ),
            FnArg::SelfValue(_) => self_ty(),
            FnArg::Inferred(_)
                => return Err( "Inferred arguments not supported".to_string() ),
        } )
    }
}

impl GetType for GenericArgument {

    fn get_ty( &self ) -> Result<Type, String>
    {
        match *self {
            GenericArgument::Type( ref ty ) => Ok( ty.clone() ),
            _ => Err( "Expected type parameter".to_string() )
        }
    }
}

pub trait GetIdent {

    /// Gets the Ident from the AST element.
    fn get_ident( &self ) -> Result<Ident, String>;
}

impl GetIdent for FnArg {

    fn get_ident( &self ) -> Result<Ident, String> {

        Ok( match *self {
            FnArg::SelfRef(..) | FnArg::SelfValue(..)
                => Ident::new( "self", Span::call_site() ),
            FnArg::Captured( ref c ) => match c.pat {
                Pat::Ident( ref i ) => i.ident.clone(),
                _ => Err( format!( "Unsupported argument: {:?}", self ) )?,
            },
            FnArg::Ignored(..) => Ident::new( "_", Span::call_site() ),
            FnArg::Inferred(_)
                => return Err( "Inferred arguments not supported".to_string() ),
        } )
    }
}

impl GetIdent for Path {

    fn get_ident( &self ) -> Result<Ident, String> {

        self.segments.last().map( |l| l.value().ident.clone() )
                .ok_or_else( || "Empty path".to_owned() )
    }
}

impl GetIdent for Type {

    fn get_ident( &self ) -> Result<Ident, String> {

        match *self {
            Type::Path( ref p ) => p.path.get_ident(),
            _ => Err( format!( "Cannot get Ident for {:?}", self ) )
        }
    }
}

impl GetIdent for Item {
    fn get_ident( &self ) -> Result<Ident, String>
    {
        Ok( match *self {
            Item::ExternCrate( ref i ) => i.ident.clone(),
            Item::Static( ref i ) => i.ident.clone(),
            Item::Const( ref i ) => i.ident.clone(),
            Item::Fn( ref i ) => i.ident.clone(),
            Item::Mod( ref i ) => i.ident.clone(),
            Item::Type( ref i ) => i.ident.clone(),
            Item::Struct( ref i ) => i.ident.clone(),
            Item::Enum( ref i ) => i.ident.clone(),
            Item::Union( ref i ) => i.ident.clone(),
            Item::Trait( ref i ) => i.ident.clone(),
            Item::Impl( ref i ) => return i.self_ty.get_ident(),
            Item::Macro( ref m ) => return m.mac.path.get_ident(),
            Item::Macro2( ref i ) => i.ident.clone(),
            Item::Existential( ref i ) => i.ident.clone(),
            Item::TraitAlias( ref i ) => i.ident.clone(),

            Item::Use( .. )
                | Item::ForeignMod( .. )
                | Item::Verbatim( .. )
                => return Err( "Item type not supported for Ident".to_string() ),
        } )
    }
}

pub trait GetAttributes {

    /// Gets the Attributes from the AST element.
    fn get_attributes( &self ) -> Result<Vec<Attribute>, String>;
}

impl GetAttributes for Item {

    fn get_attributes( &self ) -> Result<Vec<Attribute>, String>
    {
        Ok( match *self {
            Item::ExternCrate( ref i ) => i.attrs.clone(),
            Item::Static( ref i ) => i.attrs.clone(),
            Item::Const( ref i ) => i.attrs.clone(),
            Item::Fn( ref i ) => i.attrs.clone(),
            Item::Mod( ref i ) => i.attrs.clone(),
            Item::Type( ref i ) => i.attrs.clone(),
            Item::Struct( ref i ) => i.attrs.clone(),
            Item::Enum( ref i ) => i.attrs.clone(),
            Item::Union( ref i ) => i.attrs.clone(),
            Item::Trait( ref i ) => i.attrs.clone(),
            Item::Impl( ref i ) => i.attrs.clone(),
            Item::Macro( ref i ) => i.attrs.clone(),
            Item::Macro2( ref i ) => i.attrs.clone(),
            Item::Use( ref i ) => i.attrs.clone(),
            Item::ForeignMod( ref i ) => i.attrs.clone(),
            Item::Existential( ref i ) => i.attrs.clone(),
            Item::TraitAlias( ref i ) => i.attrs.clone(),
            Item::Verbatim( .. ) => vec![],
        } )
    }
}

fn self_ty() -> Type {
    parse_quote!( Self )
}
