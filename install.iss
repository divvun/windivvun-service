#define MyAppName "WinDivvun"
#define MyAppPublisher "Universitetet i Tromsø - Norges arktiske universitet"
#define MyAppURL "http://divvun.no"

#define WinDivvunUuid "{{41F71B6E-DE82-433D-8659-7E2D7C3B95E2}"
#define Clsid "{{E45885BF-50CB-4F8F-9B19-95767EAF0F5C}"
#define OldNsisWinDivvunUuid "{{FB90E8B8-EBE5-51BC-9BDE-28535417088D}"

[Setup]
AppId={#WinDivvunUuid}
AppName={#MyAppName}
AppVersion={#Version}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={commonpf}\WinDivvun
DisableDirPage=yes
DisableProgramGroupPage=yes
OutputBaseFilename=install
Compression=lzma
SolidCompression=yes
SignedUninstaller=yes
SignTool=signtool
MinVersion=6.3.9200
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x86 x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "armenian"; MessagesFile: "compiler:Languages\Armenian.isl"
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"
Name: "catalan"; MessagesFile: "compiler:Languages\Catalan.isl"
Name: "corsican"; MessagesFile: "compiler:Languages\Corsican.isl"
Name: "czech"; MessagesFile: "compiler:Languages\Czech.isl"
Name: "danish"; MessagesFile: "compiler:Languages\Danish.isl"
Name: "dutch"; MessagesFile: "compiler:Languages\Dutch.isl"
Name: "finnish"; MessagesFile: "compiler:Languages\Finnish.isl"
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "hebrew"; MessagesFile: "compiler:Languages\Hebrew.isl"
Name: "icelandic"; MessagesFile: "compiler:Languages\Icelandic.isl"
Name: "italian"; MessagesFile: "compiler:Languages\Italian.isl"
Name: "japanese"; MessagesFile: "compiler:Languages\Japanese.isl"
Name: "norwegian"; MessagesFile: "compiler:Languages\Norwegian.isl"
Name: "polish"; MessagesFile: "compiler:Languages\Polish.isl"
Name: "portuguese"; MessagesFile: "compiler:Languages\Portuguese.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"
Name: "slovenian"; MessagesFile: "compiler:Languages\Slovenian.isl"
Name: "spanish"; MessagesFile: "compiler:Languages\Spanish.isl"
Name: "turkish"; MessagesFile: "compiler:Languages\Turkish.isl"
Name: "ukrainian"; MessagesFile: "compiler:Languages\Ukrainian.isl"

[Files]
Source: "artifacts\spelli.exe"; DestDir: "{app}\i686\"; Flags: ignoreversion recursesubdirs uninsrestartdelete
Source: "artifacts\divvunspell-mso-i686\divvunspellmso.dll"; DestDir: "{app}\i686\"; Flags: ignoreversion recursesubdirs uninsrestartdelete
Source: "artifacts\divvunspell-mso-x86_64\divvunspellmso.dll"; DestDir: "{app}\x86_64\"; Flags: ignoreversion recursesubdirs uninsrestartdelete
Source: "artifacts\windivvun-i686\windivvun.dll"; DestDir: "{app}\i686\"; Flags: ignoreversion recursesubdirs uninsrestartdelete
Source: "artifacts\windivvun-x86_64\windivvun.dll"; DestDir: "{app}\x86_64\"; Flags: ignoreversion recursesubdirs uninsrestartdelete

[Dirs]
Name: "{app}\Spellers"

[Registry]
Root: HKLM64; Subkey: "SOFTWARE\Microsoft\Spelling\Spellers\Divvun"; ValueName: "CLSID"; ValueType: "string"; ValueData: "{#Clsid}"; Flags: uninsdeletekey; Check: IsWin64
Root: HKLM64; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}"; ValueName: ""; ValueType: "string"; ValueData: "WinDivvun Spell Checking Service"; Flags: uninsdeletekey; Check: IsWin64
Root: HKLM64; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}"; ValueName: "AppId"; ValueType: "string"; ValueData: "{#Clsid}"; Flags: uninsdeletekey; Check: IsWin64
Root: HKLM64; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}\Version"; ValueName: ""; ValueType: "string"; ValueData: "{#Version}"; Flags: uninsdeletekey; Check: IsWin64
Root: HKLM64; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}\InProcServer32"; ValueName: ""; ValueType: "string"; ValueData: "{app}\x86_64\windivvun.dll"; Flags: uninsdeletekey; Check: IsWin64
Root: HKLM64; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}\InProcServer32"; ValueName: "ThreadingModel"; ValueType: "string"; ValueData: "Both"; Flags: uninsdeletekey; Check: IsWin64

Root: HKLM32; Subkey: "SOFTWARE\Microsoft\Spelling\Spellers\Divvun"; ValueName: "CLSID"; ValueType: "string"; ValueData: "{#Clsid}"; Flags: uninsdeletekey
Root: HKLM32; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}"; ValueName: ""; ValueType: "string"; ValueData: "WinDivvun Spell Checking Service"; Flags: uninsdeletekey
Root: HKLM32; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}"; ValueName: "AppId"; ValueType: "string"; ValueData: "{#Clsid}"; Flags: uninsdeletekey
Root: HKLM32; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}\Version"; ValueName: ""; ValueType: "string"; ValueData: "{#Version}"; Flags: uninsdeletekey
Root: HKLM32; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}\InProcServer32"; ValueName: ""; ValueType: "string"; ValueData: "{app}\i686\windivvun.dll"; Flags: uninsdeletekey
Root: HKLM32; Subkey: "SOFTWARE\Classes\CLSID\{#Clsid}\InProcServer32"; ValueName: "ThreadingModel"; ValueType: "string"; ValueData: "Both"; Flags: uninsdeletekey

[Code]
function GetOldNsisUninstallString: String;
var
  sUnInstPath: String;
  sUnInstallString: String;
begin
  sUnInstPath := 'Software\Microsoft\Windows\CurrentVersion\Uninstall\{#OldNsisWinDivvunUuid}';
  sUnInstallString := '';
  if not RegQueryStringValue(HKLM, sUnInstPath, 'UninstallString', sUnInstallString) then
    RegQueryStringValue(HKCU, sUnInstPath, 'UninstallString', sUnInstallString);
  Result := sUnInstallString;
end;

function UninstallOldNsisWinDivvun: String;
var
  iResultCode: Integer;
  sUnInstallString: string;
begin
  sUnInstallString := GetOldNsisUninstallString();
  if sUnInstallString <> '' then
    sUnInstallString := RemoveQuotes(sUnInstallString);
    Exec(ExpandConstant(sUnInstallString), '/S', '', SW_HIDE, ewWaitUntilTerminated, iResultCode);  
    Sleep(250);
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  iResultCode: Integer;
begin
    if CurStep = ssPostInstall then
    begin
        Exec('icacls', ExpandConstant('{app} /grant "ALL APPLICATION PACKAGES":R /T'), '', SW_HIDE, ewWaitUntilTerminated, iResultCode);
        Exec(ExpandConstant('{app}\i686\spelli.exe'), 'refresh', '', SW_HIDE, ewWaitUntilTerminated, iResultCode);
    end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var                       
  V: Integer;
  uninstString: string;    
  iResultCode: Integer;
begin
    if CurUninstallStep = usUninstall then
    begin
        Exec(ExpandConstant('{app}\i686\spelli.exe'), 'nuke', '', SW_HIDE, ewWaitUntilTerminated, iResultCode);
    end;
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ResultCode: Integer;
begin
    // Uninstall old NSIS-based WinDivvun if it exists
    UninstallOldNsisWinDivvun();             
end;