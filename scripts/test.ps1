
pushd test\cpp\x64\Release
./cpp.exe
./cpp.exe -r junit > output.xml

$wc = New-Object 'System.Net.WebClient'
$wc.UploadFile( "https://ci.appveyor.com/api/testresults/junit/$($env:APPVEYOR_JOB_ID)", (Resolve-Path .\output.xml))

popd

pushd test\cs\bin\x64\Release
vstest.console /logger:Appveyor /platform:x64 cs.dll
popd