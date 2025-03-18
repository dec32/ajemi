use std::ffi::OsStr;
use strum::IntoEnumIterator;
use windows::core::GUID;
use windows::Win32::UI::Input::KeyboardAndMouse::{ActivateKeyboardLayout, GetKeyboardLayoutList, GetKeyboardLayoutNameA, KLF_SETFORPROCESS};
use windows::Win32::UI::TextServices::{self, HKL};
use windows::Win32::{System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr, CLSID_TF_CategoryMgr}};
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;
use crate::install::{Install, Layout};
use crate::{Error, Result};
use crate::extend::{GUIDExt, ResultExt};
use crate::{global::*, extend::OsStrExt2};

//----------------------------------------------------------------------------
//
//  Registation for standard COM in-proc servers of any kind.
//  An IME is one of these servers.
//
//----------------------------------------------------------------------------


pub fn register_server() -> Result<()> {
    // Register the IME's ASCII name under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SOFTWARE\\Classes\\CLSID\\{{{}}}", IME_ID.to_rfc4122());
    let (clsid, _) = hklm.create_subkey(path)?;
    clsid.set_value("", &IME_NAME_ASCII)?;
    // Register the dll's path under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}\InprocServer32 
    let (inproc_server_32, _) = clsid.create_subkey("InprocServer32")?;
    inproc_server_32.set_value("", &dll_path()?)?;
    // Register the threading model under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}\InprocServer32
    inproc_server_32.set_value("ThreadingModel", &"Apartment")?;
    Ok(())
}

pub fn unregister_server() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SOFTWARE\\Classes\\CLSID\\{{{}}}", IME_ID.to_rfc4122());
    hklm.delete_subkey_all(path)?;
    Ok(())
}

//----------------------------------------------------------------------------
//
//  Registration for an IME.
//
//----------------------------------------------------------------------------


// features supported by the IME. there'are 18 of them in total. 
// register all of them expect the speech one and the handwriting one, or 
// your input method won't work in certain applications (for example, MS Word)
const SUPPORTED_CATEGORIES: [GUID; 16] = [
    TextServices::GUID_TFCAT_CATEGORY_OF_TIP,
    TextServices::GUID_TFCAT_TIP_KEYBOARD,
    // TextServices::GUID_TFCAT_TIP_SPEECH,
    // TextServices::GUID_TFCAT_TIP_HANDWRITING,
    TextServices:: GUID_TFCAT_TIPCAP_SECUREMODE,
    TextServices::GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
    TextServices::GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
    TextServices::GUID_TFCAT_TIPCAP_COMLESS,
    TextServices::GUID_TFCAT_TIPCAP_WOW16,
    TextServices::GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
    TextServices::GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
    TextServices::GUID_TFCAT_PROP_AUDIODATA,
    TextServices:: GUID_TFCAT_PROP_INKDATA,
    TextServices::GUID_TFCAT_PROPSTYLE_STATIC,
    GUID::from_u128(0x85F9794B_4D19_40D8_8864_4E747371A66D), // TextServices::GUID_TFCAT_PROPSTYLE_STATICCOMPSCT,
    GUID::from_u128(0x24AF3031_852D_40A2_BC09_8992898CE722), // TextServices::GUID_TFCAT_PROSTYLE_CUSTOM
    TextServices::GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER,
    TextServices::GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY
];

pub fn register_ime() -> Result<()> {
    unsafe {
        // some COM nonsense to create the registry objects.
        let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
            &CLSID_TF_InputProcessorProfiles, 
            None, 
            CLSCTX_INPROC_SERVER)?;
        let category_mgr: ITfCategoryMgr = CoCreateInstance(
            &CLSID_TF_CategoryMgr, 
            None, 
            CLSCTX_INPROC_SERVER)?;
        let (langid, layout) = detect_layout()
            .inspect_err_with_log()
            .unwrap_or((LanguageID::US as u16, HKL::default()));

        // three things to register:
        // 1. the IME itself
        // 2. language profile
        // 3. categories(the features the IME has)
        input_processor_profiles.Register(&IME_ID)?;
        log::info!("Registered the input method.");
        let ime_name: Vec<u16> = OsStr::new(IME_NAME).null_terminated_wchars();
        let icon_file: Vec<u16> = dll_path()?.null_terminated_wchars();
        let icon_index = {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
            hkcu.open_subkey(path)
                .and_then(|subkey| subkey.get_value("SystemUsesLightTheme"))
                .map(|light_theme: u32| if light_theme == 1 { LITE_TRAY_ICON_INDEX } else { DARK_TRAY_ICON_INDEX })
                .unwrap_or(LITE_TRAY_ICON_INDEX)
        };
        input_processor_profiles.AddLanguageProfile(
            &IME_ID, langid, &LANG_PROFILE_ID, &ime_name, 
            &icon_file, icon_index)?;
        input_processor_profiles.SubstituteKeyboardLayout(&IME_ID, langid, &LANG_PROFILE_ID, layout)?;
        log::info!("Registered the language profile.");
        for rcatid  in SUPPORTED_CATEGORIES {
            category_mgr.RegisterCategory(&IME_ID, &rcatid, &IME_ID)?;
        }
        log::info!("Registered the categories.");
        Ok(())
    }
}

// similar process but re-doing everything
pub fn unregister_ime() -> Result<()> {
    unsafe {
        let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
            &CLSID_TF_InputProcessorProfiles, // using ::IID would cause unregister to fail
            None, 
            CLSCTX_INPROC_SERVER)?;
        let category_mgr: ITfCategoryMgr = CoCreateInstance(
            &CLSID_TF_CategoryMgr, 
            None, 
            CLSCTX_INPROC_SERVER)?;
        for rcatid in SUPPORTED_CATEGORIES {
            category_mgr.UnregisterCategory(&IME_ID, &rcatid, &IME_ID)?;
        }
        log::info!("Unregistered the categories.");
        if let Some(langid) = Install::open().ok().and_then(|install|install.langid) {
            input_processor_profiles.RemoveLanguageProfile(&IME_ID, langid, &LANG_PROFILE_ID).ok();
        }
        for langid in LanguageID::iter() {
            let langid = langid as u16;
            input_processor_profiles.RemoveLanguageProfile(&IME_ID, langid, &LANG_PROFILE_ID).ok();
        }
        log::info!("Unregistered the language profile.");
        input_processor_profiles.Unregister(&IME_ID)?;
        log::info!("Unregistered the input method.");
        Ok(())
    }
}


//----------------------------------------------------------------------------
//
//  Detection of keyboard layouts
//
//----------------------------------------------------------------------------

/// Detect if there's any preferred keyboard layout.
fn detect_layout() -> Result<(u16, HKL)> {
    let mut install = Install::open()?;
    let prefered_layout = install.layout.ok_or(Error::LayoutMissing)?;
    let mut hkls = [HKL::default(); 16];
    let len = unsafe { GetKeyboardLayoutList(Some(&mut hkls)) } as usize;
    let hkls = &hkls[..len];
    for hkl in hkls.iter().cloned() {
        let id = unsafe {
            let mut buf = [0; 9];
            ActivateKeyboardLayout(hkl, KLF_SETFORPROCESS)?;
            GetKeyboardLayoutNameA(&mut buf)?;
            u32::from_str_radix(std::str::from_utf8_unchecked(&buf[..8]), 16)
                .expect(&format!("`GetKeyboardLayoutNameA` returned malformed data. {buf:?}"))
        };
        let layout = Layout::from_lang_id(id);
        if layout == prefered_layout {
            log::info!("Detected language ID: {id:08x}");
            let lang_id = id as u16;
            install.langid = Some(lang_id);
            install.save()?;
            return Ok((lang_id, hkl));
        }
    }
    Err(Error::LayoutInvalid)
}