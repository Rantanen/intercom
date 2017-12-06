
use syn::*;

/// Extract the underlying Ty from various AST types.
pub trait GetTy {

    /// Gets the Ty from the AST element.
    fn get_ty( &self ) -> Result<Ty, String>;
}

impl GetTy for FnArg {

    fn get_ty( &self ) -> Result<Ty, String>
    {
        Ok( match self {
            &FnArg::Captured( _, ref ty )
                | &FnArg::Ignored( ref ty )
                => ty.clone(),
            &FnArg::SelfRef( ref life, m )
                => Ty::Rptr( life.clone(), Box::new( MutTy {
                    mutability: m,
                    ty: self_ty()
                } ) ),
            &FnArg::SelfValue(_)
                => self_ty(),
        } )
    }
}

impl GetTy for FunctionRetTy {

    fn get_ty( &self ) -> Result<Ty, String>
    {
        Ok( match self {
            &FunctionRetTy::Ty( ref ty ) => ty.clone(),
            &FunctionRetTy::Default => unit_ty(),
        } )
    }
}

pub trait GetIdent {

    /// Gets the Ident from the AST element.
    fn get_ident( &self ) -> Result<Ident, String>;
}

impl GetIdent for FnArg {

    fn get_ident( &self ) -> Result<Ident, String> {

        Ok( match self {
            &FnArg::SelfRef(..) | &FnArg::SelfValue(..)
                => Ident::from( "self" ),
            &FnArg::Captured( ref pat, _ ) => match pat {
                &Pat::Ident( _, ref i, _ ) => i.clone(),
                _ => Err( format!( "Unsupported argument: {:?}", self ) )?,
            },
            &FnArg::Ignored(..) => Ident::from( "_" ),
        } )
    }
}

impl GetIdent for Path {

    fn get_ident( &self ) -> Result<Ident, String> {

        self.segments.last().map( |l| l.ident.clone() )
                .ok_or( "Empty path".to_owned() )
    }
}

impl GetIdent for NestedMetaItem {

    fn get_ident( &self ) -> Result<Ident, String>
    {
        match self {
            &NestedMetaItem::MetaItem( ref mi ) => Ok( match mi {
                &MetaItem::Word( ref i ) => i,
                &MetaItem::List( ref i, .. ) => i,
                &MetaItem::NameValue( ref i, .. ) => i,
            }.clone() ),
            _ => Err( format!( "Unsupported meta item kind: {:?}", self ) ),
        }
    }
}

fn self_ty() -> Ty {
    Ty::Path(
        None,
        Path::from( PathSegment::from( Ident::from( "Self" ) ) )
    )
}

fn unit_ty() -> Ty {
    Ty::Tup( vec![] )
}
