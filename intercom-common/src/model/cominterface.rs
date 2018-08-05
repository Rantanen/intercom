
use ::prelude::*;
use super::*;
use super::macros::*;

use ::guid::GUID;
use ::ast_converters::*;
use ::methodinfo::ComMethodInfo;
use ::syn::{ Ident, Visibility, LitStr };
use ::std::collections::HashMap;
use ::std::iter::FromIterator;
use ::tyhandlers::{TypeSystem};

intercom_attribute!(
    ComInterfaceAttr< ComInterfaceAttrParam, NoParams > {
        com_iid : LitStr,
        raw_iid : LitStr,
        base : Ident,
    }
);

impl ComInterfaceAttr {

    pub fn iid( &self, ts : TypeSystem ) -> Result< Option< &LitStr >, String > {

        match ts {
            TypeSystem::Raw => self.raw_iid(),
            TypeSystem::Automation => self.com_iid(),
            TypeSystem::Invariant => self.com_iid(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ComInterface
{
    display_name : Ident,
    visibility : Visibility,
    base_interface : Option<Ident>,
    variants : HashMap<TypeSystem, ComInterfaceVariant>,
    item_type: ::utils::InterfaceType,
    is_unsafe : bool,
}

#[derive(Debug, PartialEq)]
pub struct ComInterfaceVariant
{
    display_name : Ident,
    unique_name : Ident,
    unique_base_interface : Option<Ident>,
    type_system : TypeSystem,
    iid : GUID,
    methods : Vec<ComMethodInfo>,
}

impl ComInterface
{
    /// Parses a #[com_interface] attribute and the associated item.
    pub fn parse(
        crate_name : &str,
        attr_params : TokenStream,
        item : &str,
    ) -> ParseResult<ComInterface>
    {
        // Parse the input code.
        let item : ::syn::Item = ::syn::parse_str( item )
            .map_err( |_| ParseError::ComInterface(
                    "<Unknown>".into(),
                    "Item syntax error".into() ) )?;

        Self::from_ast( crate_name, attr_params, &item )
    }

    /// Creates ComInterface from AST elements.
    pub fn from_ast(
        crate_name : &str,
        attr : TokenStream,
        item : &::syn::Item,
    ) -> ParseResult< ComInterface >
    {
        let attr : ComInterfaceAttr = ::syn::parse2( attr )
            .map_err( |_| ParseError::ComInterface(
                    item.get_ident().unwrap().to_string(),
                    "Attribute syntax error".into() ) )?;

        // Get the interface details. As [com_interface] can be applied to both
        // impls and traits this handles both of those.
        let ( itf_ident, fns, itf_type, unsafety ) =
                ::utils::get_ident_and_fns( item )
                    .ok_or_else( || ParseError::ComInterface(
                            item.get_ident().unwrap().to_string(),
                            "Unsupported associated item".into() ) )?;

        // The second argument is the optional base class. If there's no base
        // class defined, use IUnknown as the default. The value of NO_BASE will
        // construct an interface that has no base class.
        //
        // In practice the NO_BASE should be used ONLY for the IUnknown itself.
        let base = attr.base()
                .map_err( |msg| ParseError::ComInterface(
                    item.get_ident().unwrap().to_string(), msg ) )?;
        let base = match base {
            Some( b ) => if b == "NO_BASE" { None } else { Some( b.to_owned() ) },
            None => Some( Ident::new( "IUnknown", Span::call_site() ) ),
        };

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

        let variants = HashMap::from_iter(
            [ TypeSystem::Automation, TypeSystem::Raw ].into_iter().map( |&ts| {

            let itf_unique_ident = Ident::new( 
                    &format!( "{}_{:?}", itf_ident.to_string(), ts ), Span::call_site() );

                // IUnknown interfaces do not have type system variants.
                let unique_base = match base {
                    Some( ref iunk ) if iunk == "IUnknown" => base.clone(),
                    ref b => b.as_ref().map( |b| Ident::new( &format!( "{}_{:?}", b, ts ), Span::call_site() ) )
                };

                let iid_attr = attr.iid( ts )
                        .map_err( |msg| ParseError::ComInterface(
                            item.get_ident().unwrap().to_string(), msg ) )?;
                let iid = match iid_attr {
                    Some( iid ) => GUID::parse( &iid.value() )
                        .map_err( |_| ParseError::ComInterface(
                                item.get_ident().unwrap().to_string(),
                                "Bad IID format".into() ) )?,
                    None => ::utils::generate_iid(
                            crate_name, &itf_unique_ident.to_string(), ts )
                };

                // Read the method details.
                //
                // TODO: Currently we ignore invalid methods. We should probably do
                //       something smarter.
                let methods = fns.iter()
                        .map( | sig |
                            ComMethodInfo::new( sig, ts ) )
                        .filter_map( |r| r.ok() )
                        .collect::<Vec<_>>();

                Ok( ( ts, ComInterfaceVariant {
                    display_name: itf_ident.clone(),
                    unique_name : itf_unique_ident,
                    unique_base_interface : unique_base,
                    type_system : ts,
                    iid : iid,
                    methods : methods
                } ) )
            } ).collect::<Result<Vec<_>,_>>()? );

        Ok( ComInterface {
            display_name: itf_ident,
            visibility,
            base_interface: base,
            item_type: itf_type,
            is_unsafe : unsafety.is_some(),
            variants : variants,
        } )
    }

    /// Temp accessor for the automation variant.
    pub fn aut( &self ) -> &ComInterfaceVariant {
        &self.variants[ &TypeSystem::Automation ]
    }

    /// Interface name.
    pub fn name( &self ) -> &Ident { &self.display_name }

    /// Interface visibility.
    pub fn visibility( &self ) -> &Visibility { &self.visibility }

    /// The base interface.
    pub fn base_interface( &self ) -> &Option<Ident> { &self.base_interface }

    /// Interface variants.
    pub fn variants( &self ) -> &HashMap<TypeSystem, ComInterfaceVariant> { &self.variants }

    /// The type of the associated item for the #[com_interface] attribute.
    ///
    /// Either an impl or a trait.
    pub fn item_type( &self ) -> ::utils::InterfaceType { self.item_type }

    /// True, if the interface requires unsafe impl.
    pub fn is_unsafe( &self ) -> bool { self.is_unsafe }
}

impl ComInterfaceVariant {

    /// Interface unique name.
    pub fn unique_name( &self ) -> &Ident { &self.unique_name }

    /// Interface base interface variant unique name.
    pub fn unique_base_interface( &self ) -> &Option<Ident> { &self.unique_base_interface }

    /// Implemented methods.
    pub fn methods( &self ) -> &Vec<ComMethodInfo> { &self.methods }

    /// Interface IID.
    pub fn iid( &self ) -> &GUID { &self.iid }

    /// Gets the type system this interface variant represents.
    pub fn type_system( &self ) -> TypeSystem { self.type_system }
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

