#define CLSID "{{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}"

[Setup]
AppName=Divvun Spell Checker
AppVersion=0.1.0
DefaultDirName={pf}\Divvun Spell Checker
DefaultGroupName=Divvun Spell Checker
; UninstallDisplayIcon={app}\MyProg.exe
Compression=lzma2
SolidCompression=yes
OutputDir=output
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "../target/release/winspellcheck.dll"; DestDir: "{app}"

[Dirs]
Name: "{app}/dicts"

[Registry]
Root: HKLM; Subkey: "SOFTWARE\Microsoft\Spelling\Spellers\divvun"; Flags: uninsdeletekey; ValueType: string; ValueName: "CLSID"; ValueData: "{#CLSID}"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}"; Flags: uninsdeletekey; ValueType: string; ValueData: "Divvun Spell Checking Provider"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}"; Flags: uninsdeletekey; ValueType: string; ValueName: "AppId"; ValueData: "{#CLSID}"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}\InprocServer32"; Flags: uninsdeletekey; ValueType: string; ValueData: "{app}\winspellcheck.dll"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}\InprocServer32"; Flags: uninsdeletekey; ValueType: string; ValueName: "ThreadingModel"; ValueData: "Both"
Root: HKLM; Subkey: "SOFTWARE\Classes\CLSID\{#CLSID}\Version"; Flags: uninsdeletekey; ValueType: string; ValueData: "1.0"




; Windows Registry Editor Version 5.00

; [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Spelling\Spellers\divvun]
; "CLSID"="{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}"

; [HKEY_LOCAL_MACHINE\SOFTWARE\Classes\CLSID\{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}]
; @="Divvun Spell Checking Provider"
; "AppId"="{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}"

; [HKEY_LOCAL_MACHINE\SOFTWARE\Classes\CLSID\{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}\InprocServer32]
; @="C:\\Program Files\\SpellCheckTest\\winspellcheck.dll"
; "ThreadingModel"="Both"

; [HKEY_LOCAL_MACHINE\SOFTWARE\Classes\CLSID\{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}\Version]
; @="1.0"

