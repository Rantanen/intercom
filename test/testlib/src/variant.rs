
use intercom::*;
use std::convert::TryInto;
use std::time::SystemTime;
use chrono::prelude::*;

#[com_class( VariantTests )]
pub struct VariantTests;

#[com_interface]
pub trait IVariantInterface {
    fn do_stuff( &self ) -> ComResult<Variant>;
}

#[com_interface]
#[com_impl]
impl VariantTests
{
    pub fn new() -> VariantTests { VariantTests }

    pub fn variant_parameter( &self, vt : u16, variant : Variant ) -> Result<bool, ComError> {

        if variant.raw_type() != vt {
            return Err( ComError::new_message(
                    E_INVALIDARG,
                    format!( "Expected type {}, got {}", vt, variant.raw_type() ) ) );
        }

        match vt {
            0 => Ok( true ),
            1 => Ok( true ),
            2 => Ok( -1i16 == variant.try_into()? ),
            3 => Ok( -1i32 == variant.try_into()? ),
            4 => Ok( -1.0f32 == variant.try_into()? ),
            5 => Ok( -1.0f64 == variant.try_into()? ),
            6 => Ok( true ),
            7 => Ok( {
                let st : SystemTime = variant.try_into()?;
                if raw::VariantDate::com_epoch() == st {
                    true
                } else {
                    let utc : DateTime<Utc> = st.into();
                    Err( format!( "Not UNIX EPOCH: {:?}", utc ) )?;
                    false
                }
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
        }
    }

    pub fn variant_result( &self ) -> ComResult<Variant> {
        Ok( 123.into() )
    }
}
