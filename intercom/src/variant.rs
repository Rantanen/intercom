
use bstr::BStr;
use std;
use std::convert::TryFrom;

#[repr(C)]
#[derive(Copy, Clone)]
struct VariantBool(u16);

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
union VariantData {
    llVal : i64,
    lVal : i32,
    bVal : i8,
    iVal : i16,
    fltVal : f32,
    dblVal : f64,
    boolVal : VariantBool,
    //bool : _VARIANT_BOOL,
    scode : ::HRESULT,
    //cyVal : CY,
    //date : DATE,
    bstrVal : BStr,
    punkVal : ::RawComPtr,
    //*pdispVal : ComItf<IDispatch>,
    //parray : SafeArray,
    pbVal : *mut i8,
    piVal : *mut i16,
    plVal : *mut i32,
    pllVal : *mut i64,
    pfltVal : *mut f32,
    pdblVal : *mut f64,
    pboolVal : *mut VariantBool,
    //*pbool : _VARIANT_BOOL,
    //*pscode : SCODE,
    //*pcyVal : CY,
    //*pdate : DATE,
    pbstrVal : *mut BStr,
    ppunkVal : *mut ::RawComPtr,
    //ppdispVal : *mut ComItf<IDispatch>,
    //pparray : *mut SafeArray,
    pvarVal : *mut Variant,
    byref : *mut std::os::raw::c_void,
    cVal : u8,
    uiVal : u16,
    ulVal : u32,
    ullVal : u64,
    intVal : i32,
    uintVal : u32,
    //*pdecVal : DECIMAL,
    pcVal : *mut u8,
    puiVal : *mut u16,
    pulVal : *mut u32,
    pullVal : *mut u64,
    pintVal : *mut i32,
    puintVal : *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Variant {
    vt : VariantType,
    reserved1 : u16,
    reserved2 : u16,
    reserved3 : u16,
    data : VariantData,
}

impl Variant {
    fn new( vt : VariantType, data : VariantData ) -> Variant {
        Variant {
            vt: vt,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            data: data
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VariantType(u16);

impl VariantType {
    fn new( vt : u16 ) -> VariantType {
        VariantType( vt as u16 )
    }
}

#[allow(unused)]
mod var_type {
    pub const EMPTY : u16 = 0;
    pub const NULL : u16 = 1;
    pub const I2 : u16 = 2;
    pub const I4 : u16 = 3;
    pub const R4 : u16 = 4;
    pub const R8 : u16 = 5;
    pub const CY : u16 = 6;
    pub const DATE : u16 = 7;
    pub const BSTR : u16 = 8;
    pub const DISPATCH : u16 = 9;
    pub const ERROR : u16 = 10;
    pub const BOOL : u16 = 11;
    pub const VARIANT : u16 = 12;
    pub const UNKNOWN : u16 = 13;
    pub const DECIMAL : u16 = 14;
    pub const I1 : u16 = 16;
    pub const UI1 : u16 = 17;
    pub const UI2 : u16 = 18;
    pub const UI4 : u16 = 19;
    pub const I8 : u16 = 20;
    pub const UI8 : u16 = 21;
    pub const INT : u16 = 22;
    pub const UINT : u16 = 23;
    pub const VOID : u16 = 24;
    pub const HRESULT : u16 = 25;
    pub const PTR : u16 = 26;
    pub const SAFEARRAY : u16 = 27;
    pub const CARRAY : u16 = 28;
    pub const USERDEFINED : u16 = 29;
    pub const LPSTR : u16 = 30;
    pub const LPWSTR : u16 = 31;
    pub const RECORD : u16 = 36;
    pub const INTPTR : u16 = 37;
    pub const UINTPTR : u16 = 38;
    pub const FILETIME : u16 = 64;
    pub const BLOB : u16 = 65;
    pub const STREAM : u16 = 66;
    pub const STORAGE : u16 = 67;
    pub const STREAMEDOBJECT : u16 = 68;
    pub const STOREDOBJECT : u16 = 69;
    pub const BLOBOBJECT : u16 = 70;
    pub const CF : u16 = 71;
    pub const CLSID : u16 = 72;
    pub const VERSIONEDSTREAM : u16 = 73;
    pub const BSTRBLOB : u16 = 0xFFF;

    pub const VECTOR : u16 = 0x1000;
    pub const ARRAY : u16 = 0x2000;
    pub const BYREF : u16 = 0x4000;
    pub const RESERVED : u16 = 0x8000;
    pub const ILLEGAL : u16 = 0xffff;
    pub const ILLEGALMASKED : u16 = 0xfff;
    pub const TYPEMASK : u16 = 0xfff;
}

struct VariantError( VariantType );

use self::var_type::*;

macro_rules! convert {
    ( $var_type:ident => $var_field:ident : $rust_type:ty ) => {

        impl From< $rust_type > for Variant {
            fn from( src : $rust_type ) -> Variant {
                Variant::new(
                    VariantType::new( $var_type ),
                    VariantData { $var_field : src } )
            }
        }

        impl TryFrom<Variant> for $rust_type {
            type Error = VariantError;
            fn try_from( src : Variant ) -> Result< $rust_type, Self::Error> {
                Ok( match src.vt.0 {
                    $var_type => unsafe { src.data.$var_field },
                    _ => return Err( VariantError( src.vt ) )
                } )
            }
        }

    }
}

// convert!( EMPTY => Z : u16 );
// convert!( NULL => Z : u16 );
convert!( I2 => iVal : i16 );
convert!( I4 => lVal : i32 );
convert!( R4 => fltVal : f32 );
convert!( R8 => dblVal : f64 );
// convert!( CY => Z : u16 );
// convert!( DATE => Z : u16 );
convert!( BSTR => bstrVal : BStr );
// convert!( DISPATCH => Z : u16 );
// convert!( ERROR => Z : u16 );
convert!( BOOL => boolVal : VariantBool );
convert!( VARIANT => pvarVal : *mut Variant );
// convert!( UNKNOWN => Z : u16 );
// convert!( DECIMAL => Z : u16 );
convert!( I1 => bVal : i8 );
convert!( UI1 => cVal : u8 );
convert!( UI2 => uiVal : u16 );
convert!( UI4 => ulVal : u32 );
convert!( I8 => llVal : i64 );
convert!( UI8 => ullVal : u64 );
// convert!( INT => intVal : u16 );
// convert!( UINT => uintVal : u16 );
// convert!( VOID => Z : u16 );
convert!( HRESULT => scode : ::HRESULT );
// convert!( PTR => Z : u16 );
// convert!( SAFEARRAY => Z : u16 );
// convert!( CARRAY => Z : u16 );
// convert!( USERDEFINED => Z : u16 );
// convert!( LPSTR => Z : u16 );
// convert!( LPWSTR => Z : u16 );
// convert!( RECORD => Z : u16 );
// convert!( INTPTR => Z : u16 );
// convert!( UINTPTR => Z : u16 );
// convert!( FILETIME => Z : u16 );
// convert!( BLOB => Z : u16 );
// convert!( STREAM => Z : u16 );
// convert!( STORAGE => Z : u16 );
// convert!( STREAMEDOBJECT => Z : u16 );
// convert!( STOREDOBJECT => Z : u16 );
// convert!( BLOBOBJECT => Z : u16 );
// convert!( CF => Z : u16 );
// convert!( CLSID => Z : u16 );
// convert!( VERSIONEDSTREAM => Z : u16 );
// convert!( BSTRBLOB => Z : u16 );
