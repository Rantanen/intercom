
use super::*;

// <3 winapi
// (Re-defining these here as not to pull the whole winapi as runtime dependency)

/// Binary GUID format as defined for the COM interfaces.
#[repr(C)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [ u8; 8 ],
}

impl GUID {

    /// Parses the given string as a GUID.
    ///
    /// Supported formats include:
    /// - Braces and hyphens: {00000000-0000-0000-0000-000000000000}
    /// - Hyphens only: 00000000-0000-0000-0000-000000000000
    /// - Raw hexadecimal: 00000000000000000000000000000000
    pub fn parse( guid : &str ) -> Result< GUID, String >
    {
        // We support the following formats:
        // {00000000-0000-0000-0000-000000000000} (38 chars)
        // 00000000-0000-0000-0000-000000000000 (36 chars)
        // 00000000000000000000000000000000 (32 chars)
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
                ( ( buffer[ 0 ] as u32 ) << 24 ) +
                ( ( buffer[ 1 ] as u32 ) << 16 ) +
                ( ( buffer[ 2 ] as u32 ) << 8 ) +
                ( ( buffer[ 3 ] as u32 ) << 0 ),
            data2:
                ( ( buffer[ 4 ] as u16 ) << 8 ) +
                ( ( buffer[ 5 ] as u16 ) << 0 ),
            data3:
                ( ( buffer[ 6 ] as u16 ) << 8 ) +
                ( ( buffer[ 7 ] as u16 ) << 0 ),
            data4: [
                buffer[ 8 ], buffer[ 9 ], buffer[ 10 ], buffer[ 11 ],
                buffer[ 12 ], buffer[ 13 ], buffer[ 14 ], buffer[ 15 ],
            ]
        } )
    }
}

impl std::fmt::Display for GUID {

    /// Formats the GUID in brace-hyphenated format for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!( f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7] )
    }
}

impl std::fmt::UpperHex for GUID {

    /// Formats the GUID in brace-hyphenated format for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!( f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7] )
    }
}
