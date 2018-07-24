
use ::guid::GUID;
use ::ast_converters::*;
use ::methodinfo::ComMethodInfo;
use ::syn::{Ident, Visibility};

use super::*;

#[derive(Debug, PartialEq)]
pub struct ComInterface
{
    name : Ident,
    iid : GUID,
    visibility : Visibility,
    base_interface : Option<Ident>,
    methods : Vec<ComMethodInfo>,
    item_type: ::utils::InterfaceType,
    is_unsafe : bool,
}

impl ComInterface
{
    /// Parses a #[com_interface] attribute and the associated item.
    pub fn parse(
        crate_name : &str,
        attr_params : &str,
        item : &str,
    ) -> ParseResult<ComInterface>
    {
        // Parse the input code.
        let item : ::syn::Item = ::syn::parse_str( item )
            .map_err( |_| ParseError::ComInterface(
                    "<Unknown>".into(),
                    "Item syntax error".into() ) )?;
        let attr = ::utils::parse_attr_tokens( "com_interface", attr_params )
            .map_err( |_| ParseError::ComInterface(
                    item.get_ident().unwrap().to_string(),
                    "Attribute syntax error".into() ) )?;

        Self::from_ast( crate_name, &attr, &item )
    }

    /// Creates ComInterface from AST elements.
    pub fn from_ast(
        crate_name : &str,
        attr : &::syn::Attribute,
        item : &::syn::Item,
    ) -> ParseResult< ComInterface >
    {
        // Get the interface details. As [com_interface] can be applied to both
        // impls and traits this handles both of those.
        let ( itf_ident, fns, itf_type, unsafety ) =
                ::utils::get_ident_and_fns( item )
                    .ok_or_else( || ParseError::ComInterface(
                            item.get_ident().unwrap().to_string(),
                            "Unsupported associated item".into() ) )?;

        // The first attribute parameter is the IID. Parse that.
        let mut iter = ::utils::iter_parameters( attr );
        let iid = ::utils::parameter_to_guid(
                    &iter.next()
                        .ok_or_else( || ParseError::ComInterface(
                                item.get_ident().unwrap().to_string(),
                                "IID required".into() ) )?,
                    crate_name, itf_ident.as_ref(), "IID" )
                .map_err( |_| ParseError::ComInterface(
                        item.get_ident().unwrap().to_string(),
                        "Bad IID format".into() ) )?
                .ok_or_else( || ParseError::ComInterface(
                        item.get_ident().unwrap().to_string(),
                        "IID required".into() ) )?;

        // The second argument is the optional base class. If there's no base
        // class defined, use IUnknown as the default. The value of NO_BASE will
        // construct an interface that has no base class.
        //
        // In practice the NO_BASE should be used ONLY for the IUnknown itself.
        let base = iter.next()
                .map( |base| base.get_ident()
                    .map_err( |_| ParseError::ComInterface(
                            item.get_ident().unwrap().to_string(),
                            "Invalid base interface".into() ) ) )
                .map_or( Ok(None), |o| o.map(Some) )?
                .unwrap_or_else( || "IUnknown".into() );
        let base = if base == "NO_BASE" { None } else { Some( base ) };

        // Visibility for trait interfaces is the visibility of the trait.
        //
        // For implicit interfaces (impl Struct) the visibility is always public.
        // These interfaces should only exist for COM types that are meant to be
        // called from external sources as they can't be impl'd for random ComItf.
        //
        // Note this may conflict with visibility of the actual [com_class], but
        // nothing we can do for this really.
        let visibility = if let ::syn::Item::Trait( ref t ) = *item {
                t.vis.clone()
            } else {
                parse_quote!( pub )
            };

        // Read the method details.
        //
        // TODO: Currently we ignore invalid methods. We should probably do
        //       something smarter.
        let methods = fns.into_iter()
            .map( | sig |
                ComMethodInfo::new( sig ) )
            .filter_map( |r| r.ok() )
            .collect::<Vec<_>>();

        Ok( ComInterface {
            name: itf_ident,
            iid,
            visibility,
            base_interface: base,
            methods,
            item_type: itf_type,
            is_unsafe : unsafety.is_some(),
        } )
    }

    /// Interface name.
    pub fn name( &self ) -> Ident { self.name }

    /// Interface IID.
    pub fn iid( &self ) -> &GUID { &self.iid }

    /// Interface visibility.
    pub fn visibility( &self ) -> &Visibility { &self.visibility }

    /// The base interface.
    pub fn base_interface( &self ) -> &Option<Ident> { &self.base_interface }

    /// Interface methods.
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }

    /// The type of the associated item for the #[com_interface] attribute.
    ///
    /// Either an impl or a trait.
    pub fn item_type( &self ) -> ::utils::InterfaceType { self.item_type }

    /// True, if the interface requires unsafe impl.
    pub fn is_unsafe( &self ) -> bool { self.is_unsafe }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_com_interface() {
        let itf = ComInterface::parse(
            "not used",
            r#""12345678-1234-1234-1234-567890ABCDEF""#,
            "trait ITrait { fn foo( &self ); fn bar( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "ITrait" );
        assert_eq!( itf.iid(),
            &GUID::parse( "12345678-1234-1234-1234-567890ABCDEF" ).unwrap() );
        assert_eq!( itf.visibility(), &Visibility::Inherited );
        assert_eq!( itf.base_interface().as_ref().unwrap(), "IUnknown" );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "foo" );
        assert_eq!( itf.methods[1].name, "bar" );
    }

    #[test]
    fn parse_com_interface_with_auto_guid() {
        let itf = ComInterface::parse(
            "not used",
            r#"AUTO_GUID"#,
            "pub trait IAutoGuid { fn one( &self ); fn two( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "IAutoGuid" );
        assert_eq!( itf.iid(),
            &GUID::parse( "11BA222D-A34B-32BC-4A1F-77157F37803A" ).unwrap() );

        let pub_visibility : Visibility = parse_quote!( pub );
        assert_eq!( itf.visibility(), &pub_visibility );
        assert_eq!( itf.base_interface().as_ref().unwrap(), "IUnknown" );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }


    #[test]
    fn parse_com_interface_with_base_interface() {
        let itf = ComInterface::parse(
            "not used",
            r#"AUTO_GUID, IBase"#,
            "pub trait IAutoGuid { fn one( &self ); fn two( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "IAutoGuid" );
        assert_eq!( itf.iid(),
            &GUID::parse( "11BA222D-A34B-32BC-4A1F-77157F37803A" ).unwrap() );

        let pub_visibility : Visibility = parse_quote!( pub );
        assert_eq!( itf.visibility(), &pub_visibility );
        assert_eq!( itf.base_interface().as_ref().unwrap(), "IBase" );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }

    #[test]
    fn parse_com_interface_with_no_base_interface() {
        let itf = ComInterface::parse(
            "not used",
            r#"AUTO_GUID, NO_BASE"#,
            "pub trait IAutoGuid { fn one( &self ); fn two( &self ); }" )
                .expect( "com_interface attribute parsing failed" );

        assert_eq!( itf.name(), "IAutoGuid" );
        assert_eq!( itf.iid(),
            &GUID::parse( "11BA222D-A34B-32BC-4A1F-77157F37803A" ).unwrap() );

        let pub_visibility : Visibility = parse_quote!( pub );
        assert_eq!( itf.visibility(), &pub_visibility );
        assert_eq!( itf.base_interface(), &None );
        assert_eq!( itf.methods.len(), 2 );
        assert_eq!( itf.methods[0].name, "one" );
        assert_eq!( itf.methods[1].name, "two" );
    }
}
