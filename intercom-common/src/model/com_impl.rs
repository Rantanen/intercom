use ::ast_converters::*;
use ::methodinfo::ComMethodInfo;
use ::syn::{Ident};

use super::*;

#[derive(Debug, PartialEq)]
pub struct ComImpl
{
    struct_name : Ident,
    interface_name : Ident,
    is_trait_impl : bool,
    methods : Vec<ComMethodInfo>,
}

impl ComImpl
{
    /// Parses the associated item of the #[com_impl] attribute.
    pub fn parse(
        item : &str,
    ) -> ParseResult<ComImpl>
    {
        // Get the item details from the associated item.
        let item : ::syn::Item = ::syn::parse_str( item )
                .map_err( |_| ParseError::ComImpl(
                        "<Unknown>".into(),
                        "<Unknown>".into(),
                        "Could not parse [com_impl]".into() ) )?;

        Self::from_ast( &item )
    }

    /// Creates ComImpl from AST elements.
    pub fn from_ast(
        item : &::syn::Item,
    ) -> ParseResult< ComImpl >
    {
        let ( itf_ident_opt, struct_ident, fns ) =
                ::utils::get_impl_data( item )
                    .ok_or_else( || ParseError::ComImpl(
                            item.get_ident().unwrap().to_string(),
                            "<Unknown>".into(),
                            "Unsupported associated item".into() ) )?;

        // Turn the impl methods into MethodInfo.
        //
        // TODO: Currently we ignore invalid methods. We should probably do
        //       something smarter.
        let methods = fns.into_iter()
            .map( | sig |
                ComMethodInfo::new( sig ).map_err( |_| sig.ident ) )
            .filter_map( |r| r.ok() )
            .collect::<Vec<_>>();

        Ok( ComImpl {
            struct_name: struct_ident,
            is_trait_impl: itf_ident_opt.is_some(),
            interface_name: itf_ident_opt
                    .unwrap_or_else( || struct_ident ),
            methods,
        } )
    }

    /// Struct name that the trait is implemented for.
    pub fn struct_name( &self ) -> Ident { self.struct_name }

    /// Trait name that is implemented. Struct name if this is an implicit impl.
    pub fn interface_name( &self ) -> Ident { self.interface_name }

    /// True if a valid trait is implemented, false for implicit impls.
    pub fn is_trait_impl( &self ) -> bool { self.is_trait_impl }

    /// Implemented methods.
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_impl_for_struct() {
        let itf = ComImpl::parse(
            "impl Foo { fn foo( &self ) {} fn bar( &self ) {} }" )
                .expect( "com_impl attribute parsing failed" );

        assert_eq!( itf.struct_name(), "Foo" );
        assert_eq!( itf.interface_name(), "Foo" );
        assert_eq!( itf.is_trait_impl(), false );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "foo" );
        assert_eq!( itf.methods[1].name, "bar" );
    }

    #[test]
    fn parse_com_impl_for_trait() {
        let itf = ComImpl::parse(
            "impl IFoo for Bar { fn one( &self ) {} fn two( &self ) {} }" )
                .expect( "com_impl attribute parsing failed" );

        assert_eq!( itf.struct_name(), "Bar" );
        assert_eq!( itf.interface_name(), "IFoo" );
        assert_eq!( itf.is_trait_impl(), true );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }

}