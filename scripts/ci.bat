setlocal enableextensions
FOR /F "tokens=* delims= " %%i in ( 'vswhere -format value -property installationPath' ) do ( set installdir=%%i )
pushd %installdir%
call .\VC\Auxiliary\Build\vcvars64.bat
popd

REM Build stuff
cd ..
cargo build

pushd intercom_utils

pushd ..\test\

pushd testlib
cargo build

cd ..\..\intercom_utils
cargo run -- idl ..\test\testlib > ..\test\testlib\testlib.idl
cargo run -- manifest ..\test\testlib > ..\test\testlib\TestLib.Assembly.manifest
popd

pushd cpp
devenv cpp.sln /Build
cd x64\Debug
cpp.exe
popd
pwd

