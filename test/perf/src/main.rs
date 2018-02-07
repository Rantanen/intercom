#![feature(test)]

extern crate rand;
extern crate test;
extern crate libc;

use rand::distributions::{IndependentSample, Range};

static TEST_CHARACTERS: usize = 5000;

/// Main entry point.
fn main() {


    let buffer = prepare_string_buffer( TEST_CHARACTERS );
    let mut characters_found = 0;
    for c in &buffer {
        if c.clone() == 0 { break; }
        characters_found += 1;
    }
    assert!( characters_found == TEST_CHARACTERS );
    let null_position_simple = find_null_terminator_simple_impl( &buffer );
    let null_position_libc = find_null_terminator_libc_impl( &buffer );
    assert!( null_position_simple == TEST_CHARACTERS );
    assert!( null_position_simple == null_position_libc );
}

/// Measure the time it takes to validate UTF-8 string.
#[bench]
fn from_utf8(b: &mut test::Bencher) {

    // Prepare the buffer.
    let buffer = prepare_string_buffer( TEST_CHARACTERS );

    b.iter(|| {
        test::black_box( convert_to_utf8_impl( &buffer ) );
    } );
}

/// Measures the time to find a  null terminator from a buffer with a simple loop
#[bench]
fn find_null_terminator_simple( b: &mut test::Bencher ) {

    let buffer = prepare_string_buffer( TEST_CHARACTERS );
    b.iter(|| {
        test::black_box( find_null_terminator_simple_impl( &buffer ) );
    } );
}

/// Measures the time to find a  null terminator from a buffer with a loop
/// with 4 unrolled iterations. The array access positions are calculated
/// separately to avoid data dependencies.
#[bench]
fn find_null_terminator_unrolled( b: &mut test::Bencher ) {

    let buffer = prepare_string_buffer( TEST_CHARACTERS );
    b.iter(|| {
        test::black_box( find_null_terminator_unrolled_impl( &buffer ) );
    } );
}

/// Measures the time to find a  null terminator from a buffer with a loop
/// with 4 unrolled iterations.
#[bench]
fn find_null_terminator_unrolled_wd( b: &mut test::Bencher ) {

    // Prepare the buffer.
    let buffer = prepare_string_buffer( TEST_CHARACTERS );
    b.iter(|| {
        test::black_box( find_null_terminator_unrolled_wd_impl( &buffer ) );
    } );
}

/// Measures the time to find a  null terminator from a buffer with a loop
/// with 4 unrolled iterations.
#[bench]
fn find_null_terminator_libc( b: &mut test::Bencher ) {

    // Prepare the buffer.
    let buffer = prepare_string_buffer( TEST_CHARACTERS );

    b.iter(|| {
        test::black_box( find_null_terminator_libc_impl( &buffer ) );
    } );
}

#[allow(dead_code)]
#[inline(never)]
fn convert_to_utf8_impl(
    buffer: &Vec<u8>
) -> usize {
    let as_string: &str = std::str::from_utf8( &buffer ).expect( "Invalid UTF-8 string" );
    as_string.len()
}

#[allow(dead_code)]
#[inline(never)]
fn find_null_terminator_simple_impl(
    buffer: &Vec<u8>
) -> usize {

    let mut position: usize = 0;
    for c in buffer {

        if c.clone() == 0 { break; }
        position += 1;
    }
    return position;
}

#[allow(dead_code)]
#[inline(never)]
fn find_null_terminator_unrolled_impl(
    buffer: &Vec<u8>
) -> usize {

    let mut position: usize = 0;
    while position < buffer.len() - 4 {

        if buffer[ position ] == 0 { return position; }
        if buffer[ position + 1 ] == 0 { return position + 1; }
        if buffer[ position + 2 ] == 0 { return position + 2; }
        if buffer[ position + 3 ] == 0 { return position + 3; }
        position += 4;
    }

    loop {

        if buffer[ position ] == 0 { return position; }
        position += 1;
    }
}

#[allow(dead_code)]
#[inline(never)]
fn find_null_terminator_unrolled_wd_impl(
    buffer: &Vec<u8>
) -> usize {

    let mut position: usize = 0;
    while position < buffer.len() - 4 {

        if buffer[ position ] == 0 { return position; }
        position += 1;
        if buffer[ position ] == 0 { return position; }
        position += 1;
        if buffer[ position ] == 0 { return position; }
        position += 1;
        if buffer[ position ] == 0 { return position; }
        position += 1;
    }

    loop {

        if buffer[ position ] == 0 { return position; }
        position += 1;
    }
}

#[allow(dead_code)]
#[inline(never)]
fn find_null_terminator_libc_impl(
    buffer: &Vec<u8>
) -> usize {

    unsafe { libc::strlen( buffer.as_ptr() as *const i8 ) }
}


/// Prepares an ASCII string buffer for the tests.
fn prepare_string_buffer(
    values: usize
) -> Vec<u8> {

    let mut rng = rand::thread_rng();
    let mut buffer: Vec<u8> = Vec::new();
    let between = Range::new( 65, 90 );
    while buffer.len() < values  {
        let character: u8 = between.ind_sample( &mut rng ) as u8;
        assert!( character > 0 );

        // Uncomment to test with invalid UTF-8 string.
        // if buffer.len() == 500 { buffer.push( 0x80 ); }


        buffer.push( character );
    }
    buffer.push( 0 );
    return buffer;
}
