#define CLSID "{{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}"

[Setup]
AppName=Divvun Spellers - Spell Checker
AppVersion=0.1.0
DefaultDirName={pf}\Divvun Spellers
DefaultGroupName=Divvun Spellers
; UninstallDisplayIcon={app}\MyProg.exe
Compression=lzma2
SolidCompression=yes
OutputDir=output
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=divvun-spellers-checker

[Files]
Source: "../target/release/winspellcheck.dll"; DestDir: "{app}"

[Dirs]
Name: "{app}/dictionaries"

[Registry]
Root: HKLM; Subkey: "SOFTWARE\Microsoft\Spelling\Spellers\divvun"; Flags: uninsdeletekey; ValueType: string; ValueName: "CLSID"; ValueData: "{#CLSID}"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}"; Flags: uninsdeletekey; ValueType: string; ValueData: "Divvun Spell Checking Provider"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}"; Flags: uninsdeletekey; ValueType: string; ValueName: "AppId"; ValueData: "{#CLSID}"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}\InprocServer32"; Flags: uninsdeletekey; ValueType: string; ValueData: "{app}\winspellcheck.dll"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}\InprocServer32"; Flags: uninsdeletekey; ValueType: string; ValueName: "ThreadingModel"; ValueData: "Both"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}\Version"; Flags: uninsdeletekey; ValueType: string; ValueData: "1.0"
