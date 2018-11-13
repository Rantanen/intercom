extern crate intercom;
extern crate intercom_common;
mod os;
mod host;

#[derive(Debug)]
pub enum BuildError {
    ParseError( String ),
    IoError( String, std::io::Error ),
    CommandError( String ),
}

pub fn build( all_type_systems : bool ) {
    match os::build( all_type_systems ) {
        Ok(..) => {}
        Err( e ) => {
            println!( "cargo:warning=Error during Intercom build step: {:?}", e );
            println!( "cargo:warning=Some Intercom functionality might not be available" );
        }
    }
}
