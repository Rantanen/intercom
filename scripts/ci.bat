setlocal enableextensions
FOR /F "tokens=* delims= " %%i in ( 'scripts\vswhere -format value -property installationPath' ) do ( set installdir=%%i )
pushd %installdir%
call .\VC\Auxiliary\Build\vcvars64.bat
popd

echo on

REM Build Intercom
cargo build
if %errorlevel% neq 0 exit /b %errorlevel%

REM Build testlib
pushd test\testlib

cargo build
if %errorlevel% neq 0 exit /b %errorlevel%

popd

REM Build multilib
pushd test\multilib

cargo build
if %errorlevel% neq 0 exit /b %errorlevel%

popd


REM Generate IDL and Manifest for the testlib.
pushd intercom-cli

cargo run -- idl ..\test\testlib > ..\test\testlib\testlib.idl
if %errorlevel% neq 0 exit /b %errorlevel%

cargo run -- manifest ..\test\testlib > ..\test\testlib\TestLib.Assembly.manifest
if %errorlevel% neq 0 exit /b %errorlevel%

popd

REM Build C++ test suite
del /s /q build\x64
mkdir build
mkdir build\x64
pushd build\x64
if %errorlevel% neq 0 exit /b %errorlevel%

cmake "..\.." -DCMAKE_GENERATOR_PLATFORM=x64 -DCMAKE_BUILD_TYPE=Release
if %errorlevel% neq 0 exit /b %errorlevel%

msbuild intercom.sln /p:Platform=x64 /p:Configuration=Release
if %errorlevel% neq 0 exit /b %errorlevel%

popd

REM Build C# test suite
pushd test\cs

tlbimp ..\testlib\target\debug\test_lib.dll /MACHINE:X64 /out:TestLib.Interop.dll

msbuild /p:Platform=x64 /p:Configuration=Release
if %errorlevel% neq 0 exit /b %errorlevel%

popd

