extern crate intercom;
extern crate intercom_common;
mod os;
mod host;

use std::io::Write;

#[derive(Debug)]
pub enum BuildError {
    ParseError( String ),
    IoError( String, std::io::Error ),
    CommandError( String, String, String ),
}

impl BuildError {
    fn print_as_warning( &self, w : &mut Write ) -> Result<(), std::io::Error> {

        let msg : std::borrow::Cow<str> = match *self {
            BuildError::ParseError( ref m, .. ) => m.into(),
            BuildError::IoError( ref m, ref e ) => format!( "{}: {}", m, e ).into(),
            BuildError::CommandError( ref m, .. ) => m.into(),
        };

        writeln!( w, "cargo:warning={}", msg )?;

        match *self {
            BuildError::CommandError( _, ref stdout, ref stderr ) => {

                if stdout.len() > 0 {
                    writeln!( w, "cargo:warning=" )?;
                    writeln!( w, "cargo:warning=Program stdout:" )?;
                    writeln!( w, "cargo:warning=---------------" )?;
                    for line in stdout.split( '\n' ) {
                        writeln!( w, "cargo:warning={}", line )?;
                    }
                }

                if stderr.len() > 0 {
                    writeln!( w, "cargo:warning=" )?;
                    writeln!( w, "cargo:warning=Program stderr:" )?;
                    writeln!( w, "cargo:warning=---------------" )?;
                    for line in stderr.split( '\n' ) {
                        writeln!( w, "cargo:warning={}", line )?;
                    }
                }

                if stdout.len() == 0 && stderr.len() == 0 {
                    writeln!( w, "cargo:warning=(Command produced no output)" )?;
                }
            },
            _ => {}  // No extra lines.
        }

        Ok(())
    }
}

pub fn build( all_type_systems : bool ) {
    match os::build( all_type_systems ) {
        Ok(..) => {}
        Err( e ) => {
            e.print_as_warning( &mut std::io::stdout() )
                    .expect( "Cannot write to stdout" );
            println!( "cargo:warning=Some Intercom functionality might not be available" );
        }
    }
}
