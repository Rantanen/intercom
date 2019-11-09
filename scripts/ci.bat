setlocal enableextensions
FOR /F "tokens=* delims= " %%i in ( 'scripts\vswhere -format value -property installationPath -version "[15.0,16.0)"' ) do ( set installdir=%%i )
pushd %installdir%
call .\VC\Auxiliary\Build\vcvars64.bat
popd

echo on

REM Build Intercom and the C++ test suite
mkdir build
pushd build
if %errorlevel% neq 0 exit /b %errorlevel%

cmake ".." -DCMAKE_GENERATOR_PLATFORM=x64 -DCMAKE_BUILD_TYPE=Debug
if %errorlevel% neq 0 exit /b %errorlevel%

cmake --build . --config Debug
if %errorlevel% neq 0 exit /b %errorlevel%

popd

REM REM Build C# test suite
pushd test\cs

tlbimp ..\target\debug\test_lib.dll /MACHINE:X64 /out:TestLib.Interop.dll

nuget restore
msbuild /p:Platform=x64 /p:Configuration=Debug
if %errorlevel% neq 0 exit /b %errorlevel%

popd

