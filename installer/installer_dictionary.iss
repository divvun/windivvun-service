[Setup]
AppName=Divvun Spellers - Northern Sami Dictionary
AppVersion=0.1.0
DefaultDirName={pf}\Divvun Spellers\dictionaries\se
DefaultGroupName=Divvun Spellers
; UninstallDisplayIcon={app}\MyProg.exe
Compression=lzma2
SolidCompression=yes
OutputDir=output
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=divvun-spellers-dict-se

[Files]
Source: "../dicts/se.zhfst"; DestDir: "{app}"
