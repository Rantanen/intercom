
use syn::parse::{Parse, ParseStream, Result};

/// An empty type that cannot be parsed.
///
/// Used especially with attributes that don't take positional arguments.
#[derive(Debug)]
pub enum NoParams {}
impl Parse for NoParams {
    fn parse( input: ParseStream ) -> Result<Self> {
        Err( input.error( "Attribute does not accept positional parameters" ) )
    }
}

/// Literal string or 'None' if there should be no value.
#[derive(Debug)]
pub enum StrOption {
    Str( ::syn::LitStr ),
    None
}

impl Parse for StrOption {
    fn parse( input: ParseStream ) -> Result<Self> {

        if input.peek( ::syn::LitStr ) {
            return Ok( StrOption::Str( input.parse()? ) );
        }

        let ident : ::syn::Ident = input.parse()?;
        if ident == "None" {
            return Ok( StrOption::None );
        }

        Err( input.error( "Expected string or `None`" ) )
    }
}

/// Defines intercom attribute parameter parsing.
///
/// ```
/// intercom_attribute!(
///     SomeAttr< SomeAttrParam, Ident > {
///         param1: Ident,
///         param2: LitStr,
///         param3: Expr,
///     }
/// );
/// ```
///
/// Will define structures to parse attribute params such as:
///
/// ```
/// #[some_attr( Ident1, Ident2, Ident3 )]
/// #[some_attr( param1 = SomeIdent, List, Idents )]
/// #[some_attr( param2 = "literal", param3 = expression() + 1 )]
/// ```
macro_rules! intercom_attribute {

    ( $attr:ident < $attr_param:ident, $params:ident > { $( $name:ident : $type:ident, )* } ) => {

        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        enum $attr_param {
            $( $name ( $type ), )*
            args( $params ),
        }

        impl ::syn::parse::Parse for $attr_param {
            fn parse( input : ::syn::parse::ParseStream ) -> ::syn::parse::Result<Self> {

                if ! input.peek2( Token![=] ) {
                    return Ok( $attr_param::args( input.parse()? ) );
                }

                let ident : ::syn::Ident = input.parse()?;
                let _punct : Token![=] = input.parse()?;
                Ok( match ident.to_string().as_ref() {
                    $(
                        stringify!( $name ) => $attr_param::$name( input.parse()? ),
                    )*
                    other => return Err( input.error( format!( "Unexpected parameter: `{}`", other ) ) )
                } )
            }
        }

        struct $attr( Vec<$attr_param> );
        impl ::syn::parse::Parse for $attr {
            fn parse( input : ::syn::parse::ParseStream ) -> ::syn::parse::Result<Self> {

                // When parsing #[foo(bar)] attributes with syn, syn will include
                // the parenthess in the tts:
                // > ( bar )
                let params = match ::syn::group::parse_parens( &input ) {
                    Ok( parens ) => parens.content.call(
                        ::syn::punctuated::Punctuated::<$attr_param, ::syn::token::Comma>::parse_terminated )?,
                    Err( _ ) => input.call(
                        ::syn::punctuated::Punctuated::<$attr_param, ::syn::token::Comma>::parse_terminated )?,
                };

                Ok( $attr( params.into_iter().collect() ) )
            }
        }

        #[allow(dead_code)]
        impl $attr {
            $(
                pub fn $name( &self ) -> Result< Option< &$type >, String >
                {
                    let v = self.0.iter().filter_map( |p| match p {
                        $attr_param::$name( v ) => Some( v ),
                        _ => None
                    } ).take( 2 ).collect::<Vec<_>>();

                    match v.len() {
                        0 => Ok( None ),
                        1 => Ok( Some( &v[0] ) ),
                        _ => Err( format!(
                                "Multiple {} arguments", stringify!( $name ) ) )
                    }
                }
            )*

            pub fn args<'a>( &'a self ) -> Vec<&'a $params>
            {
                self.0.iter().filter_map( |p| match p {
                    $attr_param::args( v ) => Some( v ),
                    _ => None
                } ).collect()
            }
        }
    }
}

