
use std::env;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Compiler {
    Msvc,
    Gnu,
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Architecture {
    X86,
    X64,
}

#[allow(dead_code)]
pub struct Host {
    pub arch : Architecture,
    pub compiler : Compiler,
}

#[allow(dead_code)]
pub fn get_host() -> Host {

    let host_triple = env::var( "HOST" ).expect( "HOST not set" );
    let host_triple_parts = host_triple.split( '-' ).collect::<Vec<_>>();

    let arch = host_triple_parts[ 0 ];
    let compiler = host_triple_parts.last().unwrap();

    Host {
        arch: match arch {
            "x86_64" => Architecture::X64,
            "i686" => Architecture::X86,
            _ => panic!( "Unexpected architecture: {}", arch ),
        },
        compiler: match *compiler {
            "msvc" => Compiler::Msvc,
            "gnu" => Compiler::Gnu,
            _ => panic!( "Unexpected compiler: {}", compiler ),
        }
    }
}
