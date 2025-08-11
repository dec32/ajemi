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
// --- 1. Type Declaration ---
// The correct method requires defining a static array type with a fixed size.
// 256 is more than enough to hold all possible keyboard layouts.
type
  TLayoutArray = array[0..255] of DWORD;

// --- 2. API Declaration ---
// The function declaration is changed to accept the new static array type by reference (var).
// This is the standard Inno Setup way to pass a buffer to an API function.
function GetKeyboardLayoutList(nBuff: Integer; var lpList: TLayoutArray): Integer;
  external 'GetKeyboardLayoutList@user32.dll stdcall';

// This helper function was correct and remains the same.
function GetLayoutFriendlyName(hkl: DWORD): String;
var
  langID: Word;
  klidString: String;
begin
  langID := hkl and $FFFF;
  klidString := Format('%.8x', [langID]);
  if not RegQueryStringValue(HKLM, 'SYSTEM\CurrentControlSet\Control\Keyboard Layouts\' + klidString, 'Layout Text', Result) then
  begin
    Result := klidString;
  end;
end;

// --- 3. Globals ---
var
  CustomPage: TWizardPage;
  LayoutRadioButtons: array of TNewRadioButton; // This can remain dynamic
  SelectedHKL: DWORD;
  Layouts: TLayoutArray; // The variable for the static array
  LayoutCount: Integer;  // A global to store the count

// --- 4. Wizard UI and Event Handlers ---
procedure InitializeWizard;
var
  i: Integer;
  GuideLabel: TLabel;
  RadioButton: TNewRadioButton;
begin
  CustomPage := CreateCustomPage(wpSelectDir, 'Layout Selection', 'Choose your preferred keyboard layout.');

  GuideLabel := TLabel.Create(WizardForm);
  GuideLabel.Parent := CustomPage.Surface;
  GuideLabel.Caption := 'Which keyboard layout do you wish to use with this application?';
  GuideLabel.AutoSize := True;
  
  // Call the function correctly by passing the static array variable directly.
  // The compiler handles passing its address because of the 'var' keyword.
  LayoutCount := GetKeyboardLayoutList(256, Layouts);

  if LayoutCount > 0 then
  begin
    // This dynamic array for the UI controls is still fine.
    SetArrayLength(LayoutRadioButtons, LayoutCount);
    
    for i := 0 to LayoutCount - 1 do
    begin
      RadioButton := TNewRadioButton.Create(WizardForm);
      RadioButton.Parent := CustomPage.Surface;
      RadioButton.Caption := GetLayoutFriendlyName(Layouts[i]);
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

procedure CurPageChanged(CurPageID: Integer);
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