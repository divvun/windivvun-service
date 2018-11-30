; -- Example1.iss --
; Demonstrates copying 3 files and creating an icon.

; SEE THE DOCUMENTATION FOR DETAILS ON CREATING .ISS SCRIPT FILES!

[Setup]
AppName=Divvun Spell Checker
AppVersion=0.1.0
DefaultDirName={pf}\Divvun Spell Checker
DefaultGroupName=Divvun Spell Checker
; UninstallDisplayIcon={app}\MyProg.exe
Compression=lzma2
SolidCompression=yes
OutputDir=output

[Files]
Source: "../target/release/winspellcheck.dll"; DestDir: "{app}"

[Dirs]
Name: "{app}/dicts"


