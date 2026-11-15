#define MyAppName "Wziper"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Wziper Project"
#define MyAppExeName "Wziper.exe"

[Setup]
AppId={{C8D42F8C-5E62-4D8D-9F71-1A44C0B22A91}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\Wziper
DefaultGroupName=Wziper
OutputDir=..\dist
OutputBaseFilename=Wziper-Setup-0.1.0
SetupIconFile=..\assets\wziper.ico
UninstallDisplayIcon={app}\Wziper.exe
ArchitecturesInstallIn64BitMode=x64
ArchitecturesAllowed=x64
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
Source: "..\dist\Wziper.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Wziper"; Filename: "{app}\Wziper.exe"
Name: "{autodesktop}\Wziper"; Filename: "{app}\Wziper.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\Wziper.exe"; Description: "Launch Wziper"; Flags: postinstall nowait skipifsilent
