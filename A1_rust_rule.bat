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



%HOME%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\cargo.exe build 
%A1_rust_rule%\target\debug\A1_rust_rule.exe %1 %2 %3 %4 %5 %6 %7 %8 %9