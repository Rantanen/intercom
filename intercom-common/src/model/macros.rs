
pub enum NoParams {}
impl ::syn::synom::Synom for NoParams {
    named!(parse -> Self, reject!() );
}

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

macro_rules! intercom_attribute {

    ( $attr:ident < $attr_param:ident, $params:ident > { $( $name:ident : $type:ident, )* } ) => {

        enum $attr_param {
            $( $name ( $type ), )*
            args( $params ),
        }

        impl ::syn::synom::Synom for $attr_param {
            named!( parse -> Self, alt!(
                $(
                    do_parse!(
                        custom_keyword!( $name ) >>
                        eq : punct!(=) >>
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
            named!(parse -> Self, do_parse!(
                p: call!( ::syn::punctuated::Punctuated::<$attr_param, ::syn::token::Comma>::parse_separated_nonempty ) >>
                ( $attr( p.into_iter().collect() ) )
            ) );
        }

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

