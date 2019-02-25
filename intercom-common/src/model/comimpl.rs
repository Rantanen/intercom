
use ::prelude::*;
use super::*;

use ::ast_converters::*;
use ::methodinfo::ComMethodInfo;
use ::ordermap::OrderMap;
use ::std::iter::FromIterator;
use ::tyhandlers::{ModelTypeSystem};

#[derive(Debug, PartialEq)]
pub struct ComImpl
{
    struct_name : Ident,
    interface_display_name : Ident,
    is_trait_impl : bool,
    variants : OrderMap<ModelTypeSystem, ComImplVariant>,
}

#[derive(Debug, PartialEq)]
pub struct ComImplVariant
{
    type_system : ModelTypeSystem,
    interface_unique_name : Ident,
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
        // Resolve the idents and functions.
        let ( itf_ident_opt, struct_ident, fns ) =
                ::utils::get_impl_data( item )
                    .ok_or_else( || ParseError::ComImpl(
                            item.get_ident().unwrap().to_string(),
                            "<Unknown>".into(),
                            "Unsupported associated item".into() ) )?;
        let is_trait_impl = itf_ident_opt.is_some();
        let itf_ident = itf_ident_opt.unwrap_or_else( || struct_ident.clone() );

        let variants = OrderMap::from_iter(
            [ ModelTypeSystem::Automation, ModelTypeSystem::Raw ].iter().map( |&ts| {

            let itf_unique_ident = Ident::new(
                    &format!( "{}_{:?}", itf_ident.to_string(), ts ), Span::call_site() );

            // Turn the impl methods into MethodInfo.
            //
            // TODO: Currently we ignore invalid methods. We should probably do
            //       something smarter.
            let methods = fns.iter()
                .map( | sig |
                    ComMethodInfo::new( sig, ts ).map_err( |_| sig.ident.clone() ) )
                .filter_map( Result::ok )
                .collect::<Vec<_>>();

            ( ts, ComImplVariant {
                type_system: ts,
                interface_unique_name: itf_unique_ident,
                methods,
            } )
        } ) );

        Ok( ComImpl {
            struct_name: struct_ident,
            interface_display_name: itf_ident,
            variants,
            is_trait_impl,
        } )
    }

    /// Temp accessor for the automation variant.
    pub fn aut( &self ) -> &ComImplVariant { &self.variants[ &ModelTypeSystem::Automation ] }

    /// Struct name that the trait is implemented for.
    pub fn struct_name( &self ) -> &Ident { &self.struct_name }

    /// Interface variants.
    pub fn variants( &self ) -> &OrderMap<ModelTypeSystem, ComImplVariant> { &self.variants }

    /// Trait name that is implemented. Struct name if this is an implicit impl.
    pub fn interface_name( &self ) -> &Ident { &self.interface_display_name }

    /// True if a valid trait is implemented, false for implicit impls.
    pub fn is_trait_impl( &self ) -> bool { self.is_trait_impl }
}

impl ComImplVariant
{
    /// Implemented methods.
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }

    /// Unique interface name.
    pub fn interface_unique_name( &self ) -> &Ident { &self.interface_unique_name }
}

#[cfg(test)]
mod test
{
    use super::*;
    use tyhandlers::ModelTypeSystem::*;

    #[test]
    fn parse_com_impl_for_struct() {
        let itf = ComImpl::parse(
            "impl Foo { fn foo( &self ) {} fn bar( &self ) {} }" )
                .expect( "com_impl attribute parsing failed" );

        assert_eq!( itf.struct_name(), "Foo" );
        assert_eq!( itf.interface_name(), "Foo" );
        assert_eq!( itf.is_trait_impl(), false );
        assert_eq!( itf.variants[ &Automation ].methods.len(), 2 );
        assert_eq!( itf.variants[ &Automation ].methods[0].display_name, "foo" );
        assert_eq!( itf.variants[ &Automation ].methods[0].unique_name, "foo_Automation" );
        assert_eq!( itf.variants[ &Automation ].methods[1].display_name, "bar" );
        assert_eq!( itf.variants[ &Automation ].methods[1].unique_name, "bar_Automation" );
        assert_eq!( itf.variants[ &Raw ].methods.len(), 2 );
        assert_eq!( itf.variants[ &Raw ].methods[0].display_name, "foo" );
        assert_eq!( itf.variants[ &Raw ].methods[0].unique_name, "foo_Raw" );
        assert_eq!( itf.variants[ &Raw ].methods[1].display_name, "bar" );
        assert_eq!( itf.variants[ &Raw ].methods[1].unique_name, "bar_Raw" );
    }

    #[test]
    fn parse_com_impl_for_trait() {
        let itf = ComImpl::parse(
            "impl IFoo for Bar { fn one( &self ) {} fn two( &self ) {} }" )
                .expect( "com_impl attribute parsing failed" );

        assert_eq!( itf.struct_name(), "Bar" );
        assert_eq!( itf.interface_name(), "IFoo" );
        assert_eq!( itf.is_trait_impl(), true );
        assert_eq!( itf.variants[ &Automation ].methods.len(), 2 );
        assert_eq!( itf.variants[ &Automation ].methods[0].display_name, "one" );
        assert_eq!( itf.variants[ &Automation ].methods[0].unique_name, "one_Automation" );
        assert_eq!( itf.variants[ &Automation ].methods[1].display_name, "two" );
        assert_eq!( itf.variants[ &Automation ].methods[1].unique_name, "two_Automation" );
        assert_eq!( itf.variants[ &Raw ].methods.len(), 2 );
        assert_eq!( itf.variants[ &Raw ].methods[0].display_name, "one" );
        assert_eq!( itf.variants[ &Raw ].methods[0].unique_name, "one_Raw" );
        assert_eq!( itf.variants[ &Raw ].methods[1].display_name, "two" );
        assert_eq!( itf.variants[ &Raw ].methods[1].unique_name, "two_Raw" );
    }
}
