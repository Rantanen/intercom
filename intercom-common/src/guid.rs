
use super::*;

// <3 winapi
// (Re-defining these here as not to pull the whole winapi as runtime dependency)

/// Binary GUID format as defined for the COM interfaces.
#[repr(C)]
#[derive(Eq, PartialEq, Clone)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [ u8; 8 ],
}

impl std::fmt::Debug for GUID {
    fn fmt( &self, f: &mut std::fmt::Formatter ) -> Result<(), std::fmt::Error> {
        write!( f, "{}", self )
    }
}

impl GUID {

    pub fn zero_guid() -> GUID {
        GUID { data1: 0, data2: 0, data3: 0, data4: [ 0; 8 ] }
    }

    /// Parses the given string as a GUID.
    ///
    /// Supported formats include:
    ///
    /// - Braces and hyphens: {00000000-0000-0000-0000-000000000000}
    /// - Hyphens only: 00000000-0000-0000-0000-000000000000
    /// - Raw hexadecimal: 00000000000000000000000000000000
    pub fn parse( guid : &str ) -> Result< GUID, String >
    {
        // We support the following formats:
        //
        // - {00000000-0000-0000-0000-000000000000} (38 chars)
        // - 00000000-0000-0000-0000-000000000000 (36 chars)
        // - 00000000000000000000000000000000 (32 chars)
        //
        // Use the total length to make the assumption on the format.
        enum GuidFormat { Braces, Hyphens, Raw }
        let guid_format = match guid.len() {
            38 => GuidFormat::Braces,
            36 => GuidFormat::Hyphens,
            32 => GuidFormat::Raw,
            _ => return Err( format!(
                    "Unrecognized GUID format: '{}' ({})",
                    guid, guid.len() ) ),
        };

        // Define the format requirements.
        //
        // Some(_) values correspond to required characters while None values
        // correspond to the hexadecimal digits themselves.
        let format = match guid_format {
            GuidFormat::Braces => vec![
                Some( b'{' ), None, None, None, None, None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None, None, None, None, None, None, None, None, None,
                Some( b'}' )
            ],
            GuidFormat::Hyphens => vec![
                None, None, None, None, None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None,
                Some( b'-' ), None, None, None, None, None, None, None, None, None, None, None, None
            ],
            GuidFormat::Raw => vec![
                None, None, None, None, None, None, None, None,
                None, None, None, None,
                None, None, None, None,
                None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None
            ]
        };

        // Read the hexadecimal values into a buffer.
        let mut buffer = [ 0u8; 16 ];
        let mut digit = 0;
        for ( i_char, chr ) in guid.bytes().enumerate() {

            // If this is a fixed character, ensure we have the correct one.
            // If we had the correct char, continue onto next character,
            // otherwise error out. In any case we'll go past this guard
            // only if we're expeting a hexadecimal-digit.
            if let Some( b ) = format[ i_char ] {
                if chr == b {
                    continue
                } else {
                    return Err( format!( "Unexpected character in GUID: {}", chr ) );
                }
            }

            // Convert the hexadecimal character into a numerical value.
            let value : u8 = match chr {
                b'0'...b'9' => chr - b'0',
                b'a'...b'f' => chr - b'a' + 10,
                b'A'...b'F' => chr - b'A' + 10,
                _ => return Err( format!( "Unrecognized character in GUID: {}", chr ) )
            };

            // Each digit corresponds to one half of a byte in the final [u8]
            // buffer. Resolve which half and which byte this is.
            let half = digit % 2;
            let byte = ( digit - half ) / 2;

            // Finally add the value into the buffer and proceed to the next
            // digit.
            if half == 0 {
                buffer[ byte ] += value * 16;
            } else {
                buffer[ byte ] += value;
            }

            // 'digit' is incremented only when we actually match a hex-digit
            // in the GUID. 'i_char', which is the loop counter, also includes
            // all the GUID formatting characters, such as {} and -.
            digit += 1;
        }

        // We got the whole buffer. Convert it into a GUID.
        //
        // Note: We could probably do this with a memcopy - we might have even
        // been able to do it directly into the GUID by preallocating it and
        // then interpreting it as a *[u8] array.
        //
        // However I can't be bothered to figure out endianness bits here, so
        // we'll just go with raw calculations.
        Ok( GUID {
            data1:
                ( u32::from( buffer[ 0 ] ) << 24 ) +
                ( u32::from( buffer[ 1 ] ) << 16 ) +
                ( u32::from( buffer[ 2 ] ) << 8 ) +
                ( u32::from( buffer[ 3 ] ) ),
            data2:
                ( u16::from( buffer[ 4 ] ) << 8 ) +
                ( u16::from( buffer[ 5 ] ) ),
            data3:
                ( u16::from( buffer[ 6 ] ) << 8 ) +
                ( u16::from( buffer[ 7 ] ) ),
            data4: [
                buffer[ 8 ], buffer[ 9 ], buffer[ 10 ], buffer[ 11 ],
                buffer[ 12 ], buffer[ 13 ], buffer[ 14 ], buffer[ 15 ],
            ]
        } )
    }

    // Get GUID as bytes.
    pub fn as_bytes( &self ) -> &[u8; 16]
    {
        // We know the GUIDs are 128 bits (16 bytes).
        // Do the equivalent of a reinterpret_cast.
        unsafe {
            &*( self as *const _ as *const [u8; 16] )
        }
    }
}

impl Default for GUID {
    fn default() -> GUID {
        GUID::zero_guid()
    }
}

impl std::fmt::Display for GUID {

    /// Formats the GUID in brace-hyphenated format for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt_guid( self, f, &GuidFmtCase::Upper, &GuidFmt::Braces )
    }
}

impl std::fmt::LowerHex for GUID {

    /// Formats the GUID in brace-hyphenated format for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt = if f.sign_minus() {
            GuidFmt::Hyphens
        } else {
            GuidFmt::Raw
        };
        fmt_guid( self, f, &GuidFmtCase::Lower, &fmt )
    }
}

impl std::fmt::UpperHex for GUID {

    /// Formats the GUID in brace-hyphenated format for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt = if f.sign_minus() {
            GuidFmt::Hyphens
        } else {
            GuidFmt::Raw
        };
        fmt_guid( self, f, &GuidFmtCase::Upper, &fmt )
    }
}

enum GuidFmtCase { Lower, Upper }
enum GuidFmt { Braces, Hyphens, Raw }

fn fmt_guid(
    g : &GUID,
    f: &mut std::fmt::Formatter,
    case : &GuidFmtCase,
    fmt : &GuidFmt
) -> std::fmt::Result
{
    match *case {
        GuidFmtCase::Lower => match *fmt {
            GuidFmt::Braces =>
                write!( f,
                    "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
                    g.data1, g.data2, g.data3,
                    g.data4[0], g.data4[1], g.data4[2], g.data4[3],
                    g.data4[4], g.data4[5], g.data4[6], g.data4[7] ),
            GuidFmt::Hyphens =>
                write!( f,
                    "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    g.data1, g.data2, g.data3,
                    g.data4[0], g.data4[1], g.data4[2], g.data4[3],
                    g.data4[4], g.data4[5], g.data4[6], g.data4[7] ),
            GuidFmt::Raw =>
                write!( f,
                    "{:08x}{:04x}{:04x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                    g.data1, g.data2, g.data3,
                    g.data4[0], g.data4[1], g.data4[2], g.data4[3],
                    g.data4[4], g.data4[5], g.data4[6], g.data4[7] ),
        },
        GuidFmtCase::Upper => match *fmt {
            GuidFmt::Braces =>
                write!( f,
                    "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
                    g.data1, g.data2, g.data3,
                    g.data4[0], g.data4[1], g.data4[2], g.data4[3],
                    g.data4[4], g.data4[5], g.data4[6], g.data4[7] ),
            GuidFmt::Hyphens =>
                write!( f,
                    "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                    g.data1, g.data2, g.data3,
                    g.data4[0], g.data4[1], g.data4[2], g.data4[3],
                    g.data4[4], g.data4[5], g.data4[6], g.data4[7] ),
            GuidFmt::Raw =>
                write!( f,
                    "{:08X}{:04X}{:04X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                    g.data1, g.data2, g.data3,
                    g.data4[0], g.data4[1], g.data4[2], g.data4[3],
                    g.data4[4], g.data4[5], g.data4[6], g.data4[7] ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_guid() {
        let guid = GUID::zero_guid();

        assert_eq!( 0, guid.data1 );
        assert_eq!( 0, guid.data2 );
        assert_eq!( 0, guid.data3 );
        assert_eq!( [ 0, 0, 0, 0, 0, 0, 0, 0 ], guid.data4 );
    }

    #[test]
    fn parse_braces() {

        let expected = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        let actual = GUID::parse( "{12345678-90ab-cdef-fedc-ba0987654321}" )
                            .unwrap();

        assert_eq!( expected, actual );
    }

    #[test]
    fn parse_hyphenated() {

        let expected = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        let actual = GUID::parse( "12345678-90ab-cdef-fedc-ba0987654321" )
                            .unwrap();

        assert_eq!( expected, actual );
    }

    #[test]
    fn parse_raw() {

        let expected = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        let actual = GUID::parse( "1234567890abcdeffedcba0987654321" )
                            .unwrap();

        assert_eq!( expected, actual );
    }

    #[test]
    fn format_default() {

        let expected = "{12345678-90AB-CDEF-FEDC-BA0987654321}";
        let guid = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        assert_eq!( expected, format!( "{}", guid ) );
    }

    #[test]
    fn format_lowerhex() {

        let expected = "1234567890abcdeffedcba0987654321";
        let guid = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        assert_eq!( expected, format!( "{:x}", guid ) );
    }

    #[test]
    fn format_lowerhex_hyphens() {

        let expected = "12345678-90ab-cdef-fedc-ba0987654321";
        let guid = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        assert_eq!( expected, format!( "{:-x}", guid ) );
    }

    #[test]
    fn format_upperhex() {

        let expected = "1234567890ABCDEFFEDCBA0987654321";
        let guid = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        assert_eq!( expected, format!( "{:X}", guid ) );
    }

    #[test]
    fn format_upperhex_hyphens() {

        let expected = "12345678-90AB-CDEF-FEDC-BA0987654321";
        let guid = GUID {
            data1: 0x12345678,
            data2: 0x90ab,
            data3: 0xcdef,
            data4: [ 0xfe, 0xdc, 0xba, 0x09, 0x87, 0x65, 0x43, 0x21 ]
        };

        assert_eq!( expected, format!( "{:-X}", guid ) );
    }
}
