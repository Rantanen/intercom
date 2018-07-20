pushd bin\AMD64\Release
./cpp-raw.exe
$cpp_raw_ok = $?
./cpp-wrapper.exe
$cpp_wrapper_ok = $?
./cpp-dl.exe
$cpp_dl_ok = $?


# Upload test results only if we're running on AppVeyor.
if ( Test-Path env:APPVEYOR_JOB_ID ) {
    ./cpp-raw.exe -r junit > output_raw.xml
    $wc = New-Object 'System.Net.WebClient'
    $wc.UploadFile( "https://ci.appveyor.com/api/testresults/junit/$($env:APPVEYOR_JOB_ID)", (Resolve-Path .\output_raw.xml))

    ./cpp-wrapper.exe -r junit > output_wrapper.xml
    $wc = New-Object 'System.Net.WebClient'
    $wc.UploadFile( "https://ci.appveyor.com/api/testresults/junit/$($env:APPVEYOR_JOB_ID)", (Resolve-Path .\output_wrapper.xml))

    ./cpp-dl.exe -r junit > output_dl.xml
    $wc = New-Object 'System.Net.WebClient'
    $wc.UploadFile( "https://ci.appveyor.com/api/testresults/junit/$($env:APPVEYOR_JOB_ID)", (Resolve-Path .\output_dl.xml))
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

if( -not ( $cpp_raw_ok -and $cpp_wrapper_ok -and $cpp_dl_ok -and $cs_ok ) ) {
    exit -1
}
