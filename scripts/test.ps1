# Something wrong with this for now. Not supported currently.
# cargo test
# $cargo_ok = $?
$cargo_ok = $true

pushd test\cpp\x64\Release
./cpp.exe
$cpp_ok = $?

# Upload test results only if we're running on AppVeyor.
if ( Test-Path env:APPVEYOR_JOB_ID ) {
    ./cpp.exe -r junit > output.xml
    $wc = New-Object 'System.Net.WebClient'
    $wc.UploadFile( "https://ci.appveyor.com/api/testresults/junit/$($env:APPVEYOR_JOB_ID)", (Resolve-Path .\output.xml))
}

popd

pushd test\cs\bin\x64\Release

if ( Test-Path env:APPVEYOR_JOB_ID ) {
    vstest.console /logger:Appveyor /platform:x64 cs.dll
    $cs_ok = $?
} else {
    vstest.console /platform:x64 cs.dll
    $cs_ok = $?
}

popd

if( -not ( $cargo_ok -and $cpp_ok -and $cs_ok ) ) {
    exit -1
}
