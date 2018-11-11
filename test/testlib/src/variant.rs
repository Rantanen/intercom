
use intercom::*;
use std::convert::{TryInto, TryFrom};
use std::time::SystemTime;
use chrono::prelude::*;

#[com_class( VariantTests )]
pub struct VariantTests;

#[com_interface]
pub trait IVariantInterface {
    fn do_stuff( &self ) -> Result<Variant, ComError>;
}

#[com_class( IVariantInterface )]
pub struct VariantImpl;

#[com_impl]
impl IVariantInterface for VariantImpl
{
    fn do_stuff( &self ) -> Result<Variant, ComError>
    {
        Ok( Variant::from( 1.0 / 3.0 ) )
    }
}

#[com_interface]
#[com_impl]
impl VariantTests
{
    pub fn new() -> VariantTests { VariantTests }

    pub fn variant_parameter(
        &self,
        vt : u16,
        variant : Variant
    ) -> Result<(), ComError> {

        let vt_type = if vt > 100 { vt / 100 } else { vt };
        if variant.raw_type() != vt_type {
            return Err( ComError::new_message(
                    E_INVALIDARG,
                    format!( "Expected type {}, got {}", vt, variant.raw_type() ) ) );
        }

        // Format the data into string first so we can use it as debug message
        // if the value is wrong.
        //
        // We need to do this before because the match below deconstructs the variant.
        let mut data = format!( "{:?}", variant );
        let r = match vt {
            0 => Ok( true ),
            1 => Ok( true ),
            2 => Ok( -1i16 == variant.try_into()? ),
            3 => Ok( -1i32 == variant.try_into()? ),
            4 => Ok( -1.234f32 == variant.try_into()? ),
            5 => Ok( -1.234f64 == variant.try_into()? ),
            6 => Ok( true ),
            701 => Ok( {
                let st = DateTime::<Utc>::from( SystemTime::try_from( variant )? );
                let expected = DateTime::<Utc>::from( raw::VariantDate::com_epoch() );
                data = st.to_string();
                DateTime::<Utc>::from( st ) == expected
            } ),
            702 => Ok( {
                let st = DateTime::<Utc>::from( SystemTime::try_from( variant )? );
                let expected = DateTime::parse_from_rfc3339( "2000-01-02T03:04:05-00:00" )
                        .unwrap();
                data = st.to_string();
                DateTime::<Utc>::from( st ) == expected
            } ),
            703 => Ok( {
                let st = DateTime::<Utc>::from( SystemTime::try_from( variant )? );
                let expected = DateTime::parse_from_rfc3339( "2000-01-01T00:00:00-00:00" )
                        .unwrap();
                data = st.to_string();
                DateTime::<Utc>::from( st ) == expected
            } ),
            704 => Ok( {
                let st = DateTime::<Utc>::from( SystemTime::try_from( variant )? );
                let expected = DateTime::parse_from_rfc3339( "1800-01-02T03:04:05-00:00" )
                        .unwrap();
                data = st.to_string();
                DateTime::<Utc>::from( st ) == expected
            } ),
            705 => Ok( {
                let st = DateTime::<Utc>::from( SystemTime::try_from( variant )? );
                let expected = DateTime::parse_from_rfc3339( "1800-01-01T00:00:00-00:00" )
                        .unwrap();
                data = st.to_string();
                DateTime::<Utc>::from( st ) == expected
            } ),
            8 => Ok( {
                let bstr : BString = variant.try_into()?;
                let string : String = bstr.com_into()?;
                "text" == string
            } ),
            9 => Ok( true ),
            10 => Ok( true ),
            11 => Ok( true == variant.try_into()? ),
            12 => Ok( true ),
            13 => Ok( true ),
            14 => Ok( true ),  // DECIMAL
            16 => Ok( -1i8 == variant.try_into()? ),
            17 => Ok( 129u8 == variant.try_into()? ),
            18 => Ok( 12929u16 == variant.try_into()? ),
            19 => Ok( 1292929u32 == variant.try_into()? ),
            20 => Ok( -1i64 == variant.try_into()? ),
            21 => Ok( 129292929u64 == variant.try_into()? ),
            _ => Err( E_NOTIMPL )?,
        };

        // Return the result depending on what we got.
        match r {
            Ok( true ) => Ok(()),
            Ok( false ) => 
                    Err( ComError::new_message(
                            E_INVALIDARG,
                            format!( "Bad data: {}", data ) ) ),
            Err( e ) => e
        }
    }

    pub fn bad_variant_parameter(
        &self,
        vt : u16,
        variant : Variant
    ) -> ComResult<()> {

        let r = ( || Ok( match vt {
            0 => { <()>::try_from( variant )?; },
            1 => { <()>::try_from( variant )?; },
            2 => { i16::try_from( variant )?; },
            3 => { i32::try_from( variant )?; },
            4 => { f32::try_from( variant )?; },
            5 => { f64::try_from( variant )?; },
            7 => { SystemTime::try_from( variant )?; },
            8 => { BString::try_from( variant )?; },
            11 => { bool::try_from( variant )?; },
            16 => { i8::try_from( variant )?; },
            17 => { u8::try_from( variant )?; },
            18 => { u16::try_from( variant )?; },
            19 => { u32::try_from( variant )?; },
            20 => { i64::try_from( variant )?; },
            21 => { u64::try_from( variant )?; },
            _ => Err( E_NOTIMPL )?,
        } ) )();

        match r {
            Err( ::E_INVALIDARG ) => Ok(()),
            Err( e ) => Err( e ),
            Ok(..) => Err( E_FAIL ),
        }
    }

    pub fn variant_result( &self, vt : u16 ) -> ComResult<Variant> {

        match vt {
            0 => Ok( Variant::None ),
            2 => Ok( Variant::from( -1i16 ) ),
            3 => Ok( Variant::from( -1i32 ) ),
            4 => Ok( Variant::from( -1.234f32 ) ),
            5 => Ok( Variant::from( -1.234f64 ) ),
            701 => Ok( Variant::from( raw::VariantDate::com_epoch() ) ),
            702 => Ok( Variant::from( SystemTime::from(
                        DateTime::parse_from_rfc3339( "2000-01-02T03:04:05-00:00" ).unwrap() ) ) ),
            703 => Ok( Variant::from( SystemTime::from(
                        DateTime::parse_from_rfc3339( "2000-01-01T00:00:00-00:00" ).unwrap() ) ) ),
            704 => Ok( Variant::from( SystemTime::from(
                        DateTime::parse_from_rfc3339( "1800-01-02T03:04:05-00:00" ).unwrap() ) ) ),
            705 => Ok( Variant::from( SystemTime::from(
                        DateTime::parse_from_rfc3339( "1800-01-01T00:00:00-00:00" ).unwrap() ) ) ),
            801 => Ok( Variant::from( BString::from( "text" ) ) ),
            802 => Ok( Variant::from( String::from( "text" ) ) ),
            803 => Ok( Variant::from( CString::new( "text" ).unwrap() ) ),
            11 => Ok( Variant::from( true ) ),
            1301 => Ok( Variant::from( ComStruct::new( VariantImpl ) ) ),
            1302 => Ok( Variant::from( ComRc::<IUnknown>::from( ComStruct::new( VariantImpl ) ) ) ),
            1303 => Ok( Variant::from( ComRc::<IVariantInterface>::from( ComStruct::new( VariantImpl ) ) ) ),
            16 => Ok( Variant::from( -1i8 ) ),
            17 => Ok( Variant::from( 129u8 ) ),
            18 => Ok( Variant::from( 12929u16 ) ),
            19 => Ok( Variant::from( 1292929u32 ) ),
            20 => Ok( Variant::from( -1i64 ) ),
            21 => Ok( Variant::from( 129292929u64 ) ),
            _ => Err( E_NOTIMPL )?,
        }
    }

    pub fn variant_interface(
        &self,
        variant : Variant
    ) -> Result<Variant, ComError> {

        use std::convert::TryFrom;
        match variant {
            Variant::IUnknown( iunk ) => {
                match ComRc::<IVariantInterface>::try_from( &iunk ) {
                    Ok( itf ) => itf.do_stuff(),
                    Err( e ) => Err( ComError::new_message(
                            e,
                            "Interface not supported. IDispatch not supported by tests.".to_string() ) )
                }
            },
            _ => {
                Err( E_INVALIDARG )?
            }
        }
    }
}
