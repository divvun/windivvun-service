[Setup]
AppName=Divvun Spell Checker - Northern Sami Dictionary
AppVersion=0.1.0
DefaultDirName={pf}\Divvun Spell Checker
DefaultGroupName=Divvun Spell Checker
; UninstallDisplayIcon={app}\MyProg.exe
Compression=lzma2
SolidCompression=yes
OutputDir=output
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=divvun-dictionary-se

[Files]
Source: "../dicts/se.zhfst"; DestDir: "{app}/dicts"

[Dirs]
Name: "{app}/dicts"
