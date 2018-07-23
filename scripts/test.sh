#! /bin/sh

( cd test/testlib && cargo build )
( mkdir -p build )
( cd build && rm -f bin/cpp-raw )
( cd build && cmake .. )
( cd build && make )
( cd bin/x86_64 && ./cpp-raw )
