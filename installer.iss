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
// --- 1. API & Constant Declarations ---
const
  KL_NAMELENGTH = 9;
  KLF_SETFORPROCESS = $00000100;

type
  HKL = DWORD;

function GetKeyboardLayout(idThread: DWORD): HKL;
  external 'GetKeyboardLayout@user32.dll stdcall';
function ActivateKeyboardLayout(hkl: HKL; Flags: UINT): HKL;
  external 'ActivateKeyboardLayout@user32.dll stdcall';
function GetKeyboardLayoutNameA(pwszKLID: AnsiString): BOOL;
  external 'GetKeyboardLayoutNameA@user32.dll stdcall';
function GetKeyboardLayoutList(nBuff: Integer; var lpList: HKL): Integer;
  external 'GetKeyboardLayoutList@user32.dll stdcall';

// --- 2. Core Functions ---

function GetKlidFromHkl(hkl: HKL): String;
var
  originalHkl: HKL;
  klidBuffer: AnsiString;
begin
  originalHkl := GetKeyboardLayout(0);
  ActivateKeyboardLayout(hkl, KLF_SETFORPROCESS);
  
  klidBuffer := StringOfChar(#0, KL_NAMELENGTH);
  if GetKeyboardLayoutNameA(klidBuffer) then
    Result := klidBuffer
  else
    Result := 'error';
  
  ActivateKeyboardLayout(originalHkl, KLF_SETFORPROCESS);
end;

function GetLayoutFriendlyName(klid: String): String;
begin
  if not RegQueryStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Keyboard Layouts\' + klid, 'Layout Text', Result) then
    Result := klid;
end;

// --- 3. Globals ---
var
  CustomPage: TWizardPage;
  LayoutRadioButtons: array of TNewRadioButton;
  SelectedHKL: DWORD;

// --- 4. Wizard UI and Event Handlers ---
procedure InitializeWizard;
var
  i: Integer;
  LayoutCount: Integer;
  Layouts: array[0..255] of HKL; 
  GuideLabel: TLabel;
  RadioButton: TNewRadioButton;
  klid: String;
begin
  CustomPage := CreateCustomPage(wpSelectDir, 'Layout Selection', 'Choose your preferred keyboard layout.');

  GuideLabel := TLabel.Create(WizardForm);
  GuideLabel.Parent := CustomPage.Surface;
  GuideLabel.Caption := 'Which keyboard layout do you wish to use with this application?';
  GuideLabel.AutoSize := True;
  
  LayoutCount := GetKeyboardLayoutList(256, Layouts[0]);
  
  if LayoutCount > 0 then
  begin
    SetArrayLength(LayoutRadioButtons, LayoutCount);
    for i := 0 to LayoutCount - 1 do
    begin
      RadioButton := TNewRadioButton.Create(WizardForm);
      RadioButton.Parent := CustomPage.Surface;
      
      klid := GetKlidFromHkl(Layouts[i]);
      RadioButton.Caption := GetLayoutFriendlyName(klid);
      RadioButton.Tag := Layouts[i];
      RadioButton.Top := GuideLabel.Top + GuideLabel.Height + 16 + (i * RadioButton.Height);
      
      if i = 0 then
      begin
        RadioButton.Checked := True;
        SelectedHKL := Layouts[i];
      end;
      
      LayoutRadioButtons[i] := RadioButton;
    end;
  end
  else
  begin
    GuideLabel.Caption := 'Error: No keyboard layouts were detected on this system.';
  end;
end;

function NextButtonClick(CurPageID: Integer): Boolean;
var
  i: Integer;
begin
  if CurPageID = CustomPage.ID then
  begin
    for i := 0 to GetArrayLength(LayoutRadioButtons) - 1 do
    begin
      if LayoutRadioButtons[i].Checked then
      begin
        SelectedHKL := LayoutRadioButtons[i].Tag;
        break;
      end;
    end;
  end;
  Result := True;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssInstall then
  begin
    ForceDirectories(ExpandConstant('{localappdata}\{#MyAppName}'));
    SaveStringToFile(ExpandConstant('{localappdata}\{#MyAppName}\install.dat'), Format('%.8x', [SelectedHKL]), False);
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usUninstall then
  begin
    DeleteFile(ExpandConstant('{localappdata}\{#MyAppName}\install.dat'));
    RemoveDir(ExpandConstant('{localappdata}\{#MyAppName}'));
  end;
end;