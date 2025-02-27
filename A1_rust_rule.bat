@echo off
Setlocal ENABLEDELAYEDEXPANSION
set   dp0=%~dp0
set   inpu_1=%1
set   cd_path=%cd%


set  win_rbin=%userprofile%\Desktop\zbin\win_rbin

set  A1_rust_rule=%win_rbin%\A1_rust_rule

echo [ cd_path=%cd_path%]   [ inpu_1=%inpu_1%]    [ dp0=%~dp0%]
echo  [ win_rbin=%win_rbin%]  [ A1_rust_rule=%A1_rust_rule%]

cd  /d  %dp0%/A1_rust_rule


set cargo_path=%HOME%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe

for /f "delims=" %%j in ('where cargo') do (
set  cargo_path=%%j

)

echo cargo_path=%cargo_path%


rem check the cargon.exe file as [ where cargo ]
rem %HOME%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe build 
rem %HOME%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe run 

%cargo_path%  build


echo errorlevel=%ERRORLEVEL%
echo cargo_path=%cargo_path%

if %ERRORLEVEL% LEQ 1 (
echo   _________________ Success Compile and Run  _________________
echo=
%A1_rust_rule%\target\debug\A1_rust_rule.exe %1 %2 %3 %4 %5 %6 %7 %8 %9
echo=
echo   _________________ Success Run Finish  _________________
) else (
echo   _________________ Failed Compile , Please Check  _________________
)
