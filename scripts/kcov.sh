!/bin/bash -eo pipefail
# Set up the environment. We need LD_LIBRARY_PATH to include rust libs for
# the rust tests to execute without cargo.
# export LD_LIBRARY_PATH=/usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:${PWD}/build/bin/lib

export KCOV=/usr/bin/kcov
export KCOV_OUT=${PWD}/target/cov
export KCOV_INCLUDE=${PWD}
export KCOV_EXCLUDE=${PWD}/test
export KCOV_FLAGS=--verify
export RUN_KCOV="$KCOV --include-pattern=$KCOV_INCLUDE --exclude-pattern=$KCOV_EXCLUDE $KCOV_FLAGS $KCOV_OUT"
mkdir -p $KCOV_OUT

# Rust tests
for file in target/debug/*-[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f];
do
    ${RUN_KCOV} "$file"
done

for file in intercom-attributes/tests/data/*.source.rs;
do
    CARGO_PKG_NAME="source" ${RUN_KCOV} rustc --crate-name source --crate-type lib "$file" --out-dir target/test_out -L dependency=target/debug/deps --extern intercom=target/debug/libintercom.rlib --pretty=expanded -Z unstable-options > /dev/null;
done

# C++ tests
( cd build/bin && ${RUN_KCOV} cpp-raw )

bash <( curl -s https://codecov.io/bash )
