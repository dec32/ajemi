use std::ffi::{OsStr, OsString};
use log::{debug, error};
use windows::{core::{GUID, Interface}, Win32::{Foundation::{GetLastError, HINSTANCE}, System::{Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, LibraryLoader::GetModuleFileNameA}, UI::TextServices::{CLSID_TF_InputProcessorProfiles, ITfInputProcessorProfileSubstituteLayout, ITfInputProcessorProfiles, HKL}}};
use crate::{install::Install, Error, Result};

pub fn setup(dll_module: HINSTANCE) {
    unsafe { DLL_MODULE = Some(dll_module) };
}

// global variables
static mut DLL_MODULE: Option<HINSTANCE> = None;
pub fn dll_module() -> HINSTANCE {
    unsafe{ DLL_MODULE.unwrap() }
}

static mut DLL_PATH: Option<OsString> = None;
pub fn dll_path() -> Result<&'static OsStr> {
    if unsafe { DLL_PATH.as_ref() }.is_none() {
        let mut buf: Vec<u8> = vec![0;512];
        unsafe { GetModuleFileNameA(dll_module(), &mut buf) };
        if buf[0] == 0 {
            let err = unsafe { GetLastError() };
            error!("Failed to find the dll path. {:?}", err);
            return Err(err.into());
        }
        let mut from = 0;
        let mut to = buf.len();
        while to != from + 1 {
            let i = (to + from) / 2;
            if buf[i] == 0 {
                to = i;
            } else {
                from = i;
            }
        }
        buf.truncate(to);
        let path = unsafe{ OsString::from_encoded_bytes_unchecked(buf) };
        debug!("Found dll in {}", path.to_string_lossy());
        unsafe { DLL_PATH = Some(path) };
    }
    let path: &'static OsStr = unsafe{ DLL_PATH.as_ref() }.unwrap();
    Ok(path)
}

pub fn registered_hkl() -> Result<HKL> {
    let install = Install::open()?;
    let langid = install.langid.ok_or(Error::LangIdNotFound)?;
    unsafe {
        // i fucking hate microsoft
        let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
            &CLSID_TF_InputProcessorProfiles, 
            None, 
            CLSCTX_INPROC_SERVER)?;
        let hkl = input_processor_profiles
            .cast::<ITfInputProcessorProfileSubstituteLayout>()?
            .GetSubstituteKeyboardLayout(&IME_ID, langid, &LANG_PROFILE_ID)?;
        Ok(hkl)
    }
}

// registration stuff
pub const IME_NAME: &str = "Ajemi";
pub const IME_NAME_ASCII: &str = "Ajemi";
pub const IME_ID: GUID = GUID::from_u128(0xC93D3D59_2FAC_40E0_ABC6_A3658749E2FA);
pub const LANG_PROFILE_ID: GUID = GUID::from_u128(0xA411A7FC_A082_4B8A_8741_AA4A72613933);
pub const LANGBAR_ITEM_ID: GUID = GUID::from_u128(0x95288B2B_4D3B_4D4A_BF5B_9342E4F75E4D);
pub const DISPLAY_ATTR_ID: GUID = GUID::from_u128(0xE42647FB_4BF0_4570_9013_768487C5CAAE);
pub const LITE_TRAY_ICON_INDEX: u32 = 0;
pub const DARK_TRAY_ICON_INDEX: u32 = 1;
// customization
pub const CANDI_NUM: usize = 5;
pub const CANDI_INDEXES: [&str; CANDI_NUM] = ["1", "2", "3", "4", "5"];
pub const CANDI_INDEX_SUFFIX: &str = ". ";
pub const CANDI_INDEX_SUFFIX_MONO: &str = ".";
pub const PREEDIT_DELIMITER: &str = "'";
// included text
pub const DEFAULT_CONF: &str = include_str!("../res/conf.toml");
pub const SITELEN_SCHEMA: &str = include_str!("../res/schema/sitelen.schema");
pub const EMOJI_SCHEMA: &str = include_str!("../res/schema/emoji.schema");
// Keyboard Indentifiers
// QWERTY
pub const US: u16 = 0x0409;
pub const CANADIAN_FRENCH: u32 = 0x00001009;
// AZERTY
pub const FRENCH: u32 = 0x0000_040C;
pub const BELGIAN_FRENCH: u32 = 0x0000_080C;
pub const BELGIAN_FRENCH_COMMA: u32 = 0x0001_080C;
pub const BELGIAN_FRENCH_PERIOD: u32 = 0x0000_0813;
// QWERTZ
pub const GERMAN: u32 = 0x0000_0407;
pub const GERMAN_IBM: u32 = 0x0001_0407;
pub const SWISS_FRENCH: u32 = 0x0000_100C;

