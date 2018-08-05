
/// Type that implements `Synom` but fails parsing.
///
/// Use this for defining attribute parsing that accepts only named arguments.
pub enum NoParams {}
impl ::syn::synom::Synom for NoParams {
    named!(parse -> Self, reject!() );
}

/// Literal string or 'None' if there should be no value.
pub enum StrOption {
    Str( ::syn::LitStr ),
    None
}
impl ::syn::synom::Synom for StrOption {
    named!(parse -> Self, alt!(
        syn!( ::syn::LitStr ) => { |s| StrOption::Str( s ) }
        |
        custom_keyword!( None ) => { |_| StrOption::None }
    ) );
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
        enum $attr_param {
            $( $name ( $type ), )*
            args( $params ),
        }

        impl ::syn::synom::Synom for $attr_param {
            named!( parse -> Self, alt!(
                $(
                    do_parse!(
                        custom_keyword!( $name ) >>
                        punct!(=) >>
                        value : syn!( $type ) >>
                        ( $attr_param::$name( value ) )
                    )
                    |
                )*
                do_parse!( a : syn!( $params ) >> ( $attr_param::args( a ) ) )
            ) );
        }

        struct $attr( Vec<$attr_param> );
        impl ::syn::synom::Synom for $attr {
            named!(parse -> Self, alt!(
                do_parse!(
                    p: parens!( call!( ::syn::punctuated::Punctuated::<$attr_param, ::syn::token::Comma>::parse_terminated ) ) >>
                    ( $attr( p.1.into_iter().collect() ) )
                )
                |
                do_parse!(
                    p: call!( ::syn::punctuated::Punctuated::<$attr_param, ::syn::token::Comma>::parse_terminated ) >>
                    ( $attr( p.into_iter().collect() ) )
                )
                |
                do_parse!( input_end!() >> ( $attr( vec![] ) ) )
            ) );
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

