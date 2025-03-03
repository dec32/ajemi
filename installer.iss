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
WizardStyle=modern
WizardSizePercent=100
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
Source: ".\target\release\ajemi.dll"; DestDir: "{app}"; Flags: ignoreversion regserver 64bit
Source: ".\target\i686-pc-windows-msvc\release\ajemi.dll"; DestDir: "{app}"; DestName: "ajemi32.dll"; Flags: ignoreversion regserver 32bit
Source: ".\res\sitelenselikiwenjuniko.ttf"; DestDir: "{autofonts}"; FontInstall: "sitelen seli kiwen juniko"; Flags: onlyifdoesntexist uninsneveruninstall

[Icons]
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"


[Code]   
var
  CustomPage: TWizardPage;
  Guide: TLabel;
  Layout0: TNewRadioButton; 
  Layout1: TNewRadioButton;
  Layout2: TNewRadioButton;
  Layout3: TNewRadioButton;
  
procedure InitializeWizard;
begin
  CustomPage := CreateCustomPage(wpSelectDir, 'Layout', 'Choose your prefered layout.');

  Guide := TLabel.Create(WizardForm);
  Guide.Parent := CustomPage.Surface;
  Guide.Caption := 'Which keyboard layout do you wish to use?';

  Layout0 := TNewRadioButton.Create(WizardForm);
  Layout0.Parent := CustomPage.Surface;
  Layout0.Caption := 'QWERTY'
  Layout0.Top := Guide.Top + Guide.Height + 16;
  Layout0.Checked := True;

  Layout1 := TNewRadioButton.Create(WizardForm);
  Layout1.Parent := CustomPage.Surface;
  Layout1.Caption := 'QWERTY (CFR)'
  Layout1.Top := Layout0.Top + Layout0.Height + 8;
  Layout1.Checked := False;

  Layout2 := TNewRadioButton.Create(WizardForm);
  Layout2.Parent := CustomPage.Surface;
  Layout2.Caption := 'AZERTY'
  Layout2.Top := Layout1.Top + Layout1.Height + 8;
  Layout2.Checked := False;

  Layout3 := TNewRadioButton.Create(WizardForm);
  Layout3.Parent := CustomPage.Surface;
  Layout3.Caption := 'QWERTZ'
  Layout3.Top := Layout2.Top + Layout2.Height + 8;
  Layout3.Checked := False;
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpReady then begin
    ForceDirectories(ExpandConstant('{userappdata}\{#MyAppName}'));
    if Layout0.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\{#MyAppName}\install.toml'), 'layout="Qwerty"', False);
    end;
    if Layout1.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\{#MyAppName}\install.toml'), 'layout="QwertyCFR"', False);
    end;
    if Layout2.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\{#MyAppName}\install.toml'), 'layout="Azerty"', False);
    end;
    if Layout3.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\{#MyAppName}\install.toml'), 'layout="QWERTZ"', False);
    end;
  end;
end;