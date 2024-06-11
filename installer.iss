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
  QwertyBtn: TNewRadioButton; 
  QwetzyBtn: TNewRadioButton;
  AterzyBtn: TNewRadioButton;
  DvorakBtn: TNewRadioButton;
  CustomBtn: TNewRadioButton;
  
procedure InitializeWizard;
begin
  CustomPage := CreateCustomPage(wpSelectDir, 'Layout', 'Choose your prefered layout.');

  Guide := TLabel.Create(WizardForm);
  Guide.Parent := CustomPage.Surface;
  Guide.Caption := 'Which keyboard layout do you wish to use?';

  QwertyBtn := TNewRadioButton.Create(WizardForm);
  QwertyBtn.Parent := CustomPage.Surface;
  QwertyBtn.Caption := 'QWERTY'
  QwertyBtn.Top := Guide.Top + Guide.Height + 16;
  QwertyBtn.Checked := True;

  QwetzyBtn := TNewRadioButton.Create(WizardForm);
  QwetzyBtn.Parent := CustomPage.Surface;
  QwetzyBtn.Caption := 'QWERTZ'
  QwetzyBtn.Top := QwertyBtn.Top + QwertyBtn.Height + 8;
  QwetzyBtn.Checked := False;

  AterzyBtn := TNewRadioButton.Create(WizardForm);
  AterzyBtn.Parent := CustomPage.Surface;
  AterzyBtn.Caption := 'ATERZY'
  AterzyBtn.Top := QwetzyBtn.Top + QwetzyBtn.Height + 8;
  AterzyBtn.Checked := False;

  DvorakBtn := TNewRadioButton.Create(WizardForm);
  DvorakBtn.Parent := CustomPage.Surface;
  DvorakBtn.Caption := 'Dvorak'
  DvorakBtn.Top := AterzyBtn.Top + AterzyBtn.Height + 8;
  DvorakBtn.Checked := False;

  CustomBtn := TNewRadioButton.Create(WizardForm);
  CustomBtn.Parent := CustomPage.Surface;
  CustomBtn.Caption := 'Colemak, Workman'
  CustomBtn.Top := DvorakBtn.Top + DvorakBtn.Height + 8;
  CustomBtn.Checked := False;
end;

procedure CurPageChanged(CurPageID: Integer);
begin
  if CurPageID = wpReady then begin
    if QwertyBtn.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\Ajemi\.layout'), 'QWERTY', False);
    end;
    if QwetzyBtn.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\Ajemi\.layout'), 'QWERTZ', False);
    end;
    if AterzyBtn.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\Ajemi\.layout'), 'AZERTY', False);
    end;
    if DvorakBtn.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\Ajemi\.layout'), 'DVORAK', False);
    end;
    if CustomBtn.Checked then begin
      SaveStringToFile(ExpandConstant('{userappdata}\Ajemi\.layout'), 'CUSTOM', False);
    end;
  end;
end;