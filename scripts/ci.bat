setlocal enableextensions
FOR /F "tokens=* delims= " %%i in ( 'vswhere -format value -property installationPath' ) do ( set installdir=%%i )
pushd %installdir%
call .\VC\Auxiliary\Build\vcvars64.bat
popd

REM Build stuff
cd ..
cargo build

cd test/calculator
../../com_utils/target/debug/com_utils.exe idl .