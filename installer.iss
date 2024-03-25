#define MyAppName "Ajemi"
#define MyAppVersion "0.1"
#define MyAppPublisher "Dec_32"
#define MyAppURL "https://github.com/dec32/Ajemi"

[Setup]
;; basic info
AppId={{DDEBE3AB-2FAE-426A-9E41-AAAEAE359C72}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultGroupName={#MyAppName}
DefaultDirName={autopf}\{#MyAppName}
;; icon and style
WizardStyle=classic
SetupIconFile=.\res\installer.ico
UninstallIconFile=.\res\installer.ico
;; allow user to disable start menu shorcuts
AllowNoIcons=yes
;; compile
OutputDir=.\target\release
OutputBaseFilename=ajemi-installer_x64
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: ".\target\release\ajemi.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: ".\target\i686-pc-windows-msvc\release\ajemi.dll"; DestDir: "{app}"; DestName: "ajemi32.dll"; Flags: ignoreversion
Source: ".\res\conf.toml"; DestDir: "{userappdata}"; Flags: ignoreversion
Source: ".\res\sitelenselikiwenjuniko.ttf"; DestDir: "{autofonts}"; FontInstall: "sitelen seli kiwen juniko"; Flags: onlyifdoesntexist uninsneveruninstall

[Icons]
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"

[Run]
Filename: "regsvr32"; WorkingDir: "{app}"; Parameters: "/s .\ajemi.dll"
Filename: "regsvr32"; WorkingDir: "{app}"; Parameters: "/s .\ajemi32.dll"

[UninstallRun]
Filename: "regsvr32"; WorkingDir: "{app}"; Parameters: "/s /u .\ajemi.dll"
Filename: "regsvr32"; WorkingDir: "{app}"; Parameters: "/s /u .\ajemi32.dll"

