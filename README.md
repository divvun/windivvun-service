# Compile
1) Install Visual Studio with the C++ Compiler and the Windows 10 SDK enabled, or just the Windows build tools.
2) Install rust nightly for Windows https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods


spellcheckprovider.idl is in
C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\spellcheckprovider.idl
after installing the Windows SDK

C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64\midl.exe


cargo build has to be run from a command line with the VC++ paths set. If you're getting errors about trying to build a 64bit binary with a 32bit compiler, run vcvars64.bat in "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build"