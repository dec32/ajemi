#![allow(unused)]
use std::{env, ffi::{OsStr, OsString}, fs, path::PathBuf};
use log::{debug, error};
use windows::{core::GUID, Win32::{Foundation::{GetLastError, HINSTANCE}, System::{Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, LibraryLoader::GetModuleFileNameA}, UI::TextServices::{ITfInputProcessorProfileSubstituteLayout, HKL}}};

use crate::{Error, Result};
use crate::register::RegInfo;

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
    let reg_info = RegInfo::open()?;
    let lang_id = reg_info.lang_id.ok_or(Error::LangIdNotFound)?;
    unsafe {
        let input_processor_profile_substitute_layout: ITfInputProcessorProfileSubstituteLayout = CoCreateInstance(
            &GUID::from_u128(0x4fd67194_1002_4513_bff2_c0ddf6258552), 
            None, 
            CLSCTX_INPROC_SERVER)?;
        let hkl = input_processor_profile_substitute_layout.GetSubstituteKeyboardLayout(
            &IME_ID, 
            lang_id, 
            &LANG_PROFILE_ID)?;
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

// virtual keycodes
pub const VK_0: u16 = 48;
pub const VK_1: u16 = 49;
pub const VK_2: u16 = 50;
pub const VK_3: u16 = 51;
pub const VK_4: u16 = 52;
pub const VK_5: u16 = 53;
pub const VK_6: u16 = 54;
pub const VK_7: u16 = 55;
pub const VK_8: u16 = 56;
pub const VK_9: u16 = 57;
pub const VK_A: u16 = 65;
pub const VK_ABNT_C1: u16 = 193;
pub const VK_ABNT_C2: u16 = 194;
pub const VK_ACCEPT: u16 = 30;
pub const VK_ADD: u16 = 107;
pub const VK_APPS: u16 = 93;
pub const VK_ATTN: u16 = 246;
pub const VK_B: u16 = 66;
pub const VK_BACK: u16 = 8;
pub const VK_BROWSER_BACK: u16 = 166;
pub const VK_BROWSER_FAVORITES: u16 = 171;
pub const VK_BROWSER_FORWARD: u16 = 167;
pub const VK_BROWSER_HOME: u16 = 172;
pub const VK_BROWSER_REFRESH: u16 = 168;
pub const VK_BROWSER_SEARCH: u16 = 170;
pub const VK_BROWSER_STOP: u16 = 169;
pub const VK_C: u16 = 67;
pub const VK_CANCEL: u16 = 3;
pub const VK_CAPITAL: u16 = 20;
pub const VK_CLEAR: u16 = 12;
pub const VK_CONTROL: u16 = 17;
pub const VK_CONVERT: u16 = 28;
pub const VK_CRSEL: u16 = 247;
pub const VK_D: u16 = 68;
pub const VK_DBE_ALPHANUMERIC: u16 = 240;
pub const VK_DBE_CODEINPUT: u16 = 250;
pub const VK_DBE_DBCSCHAR: u16 = 244;
pub const VK_DBE_DETERMINESTRING: u16 = 252;
pub const VK_DBE_ENTERDLGCONVERSIONMODE: u16 = 253;
pub const VK_DBE_ENTERIMECONFIGMODE: u16 = 248;
pub const VK_DBE_ENTERWORDREGISTERMODE: u16 = 247;
pub const VK_DBE_FLUSHSTRING: u16 = 249;
pub const VK_DBE_HIRAGANA: u16 = 242;
pub const VK_DBE_KATAKANA: u16 = 241;
pub const VK_DBE_NOCODEINPUT: u16 = 251;
pub const VK_DBE_NOROMAN: u16 = 246;
pub const VK_DBE_ROMAN: u16 = 245;
pub const VK_DBE_SBCSCHAR: u16 = 243;
pub const VK_DECIMAL: u16 = 110;
pub const VK_DELETE: u16 = 46;
pub const VK_DIVIDE: u16 = 111;
pub const VK_DOWN: u16 = 40;
pub const VK_E: u16 = 69;
pub const VK_END: u16 = 35;
pub const VK_EREOF: u16 = 249;
pub const VK_ESCAPE: u16 = 27;
pub const VK_EXECUTE: u16 = 43;
pub const VK_EXSEL: u16 = 248;
pub const VK_F: u16 = 70;
pub const VK_F1: u16 = 112;
pub const VK_F10: u16 = 121;
pub const VK_F11: u16 = 122;
pub const VK_F12: u16 = 123;
pub const VK_F13: u16 = 124;
pub const VK_F14: u16 = 125;
pub const VK_F15: u16 = 126;
pub const VK_F16: u16 = 127;
pub const VK_F17: u16 = 128;
pub const VK_F18: u16 = 129;
pub const VK_F19: u16 = 130;
pub const VK_F2: u16 = 113;
pub const VK_F20: u16 = 131;
pub const VK_F21: u16 = 132;
pub const VK_F22: u16 = 133;
pub const VK_F23: u16 = 134;
pub const VK_F24: u16 = 135;
pub const VK_F3: u16 = 114;
pub const VK_F4: u16 = 115;
pub const VK_F5: u16 = 116;
pub const VK_F6: u16 = 117;
pub const VK_F7: u16 = 118;
pub const VK_F8: u16 = 119;
pub const VK_F9: u16 = 120;
pub const VK_FINAL: u16 = 24;
pub const VK_G: u16 = 71;
pub const VK_GAMEPAD_A: u16 = 195;
pub const VK_GAMEPAD_B: u16 = 196;
pub const VK_GAMEPAD_DPAD_DOWN: u16 = 204;
pub const VK_GAMEPAD_DPAD_LEFT: u16 = 205;
pub const VK_GAMEPAD_DPAD_RIGHT: u16 = 206;
pub const VK_GAMEPAD_DPAD_UP: u16 = 203;
pub const VK_GAMEPAD_LEFT_SHOULDER: u16 = 200;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: u16 = 209;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: u16 = 212;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: u16 = 214;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: u16 = 213;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: u16 = 211;
pub const VK_GAMEPAD_LEFT_TRIGGER: u16 = 201;
pub const VK_GAMEPAD_MENU: u16 = 207;
pub const VK_GAMEPAD_RIGHT_SHOULDER: u16 = 199;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: u16 = 210;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: u16 = 216;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: u16 = 218;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: u16 = 217;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: u16 = 215;
pub const VK_GAMEPAD_RIGHT_TRIGGER: u16 = 202;
pub const VK_GAMEPAD_VIEW: u16 = 208;
pub const VK_GAMEPAD_X: u16 = 197;
pub const VK_GAMEPAD_Y: u16 = 198;
pub const VK_H: u16 = 72;
pub const VK_HANGEUL: u16 = 21;
pub const VK_HANGUL: u16 = 21;
pub const VK_HANJA: u16 = 25;
pub const VK_HELP: u16 = 47;
pub const VK_HOME: u16 = 36;
pub const VK_I: u16 = 73;
pub const VK_ICO_00: u16 = 228;
pub const VK_ICO_CLEAR: u16 = 230;
pub const VK_ICO_HELP: u16 = 227;
pub const VK_IME_OFF: u16 = 26;
pub const VK_IME_ON: u16 = 22;
pub const VK_INSERT: u16 = 45;
pub const VK_J: u16 = 74;
pub const VK_JUNJA: u16 = 23;
pub const VK_K: u16 = 75;
pub const VK_KANA: u16 = 21;
pub const VK_KANJI: u16 = 25;
pub const VK_L: u16 = 76;
pub const VK_LAUNCH_APP1: u16 = 182;
pub const VK_LAUNCH_APP2: u16 = 183;
pub const VK_LAUNCH_MAIL: u16 = 180;
pub const VK_LAUNCH_MEDIA_SELECT: u16 = 181;
pub const VK_LBUTTON: u16 = 1;
pub const VK_LCONTROL: u16 = 162;
pub const VK_LEFT: u16 = 37;
pub const VK_LMENU: u16 = 164;
pub const VK_LSHIFT: u16 = 160;
pub const VK_LWIN: u16 = 91;
pub const VK_M: u16 = 77;
pub const VK_MBUTTON: u16 = 4;
pub const VK_MEDIA_NEXT_TRACK: u16 = 176;
pub const VK_MEDIA_PLAY_PAUSE: u16 = 179;
pub const VK_MEDIA_PREV_TRACK: u16 = 177;
pub const VK_MEDIA_STOP: u16 = 178;
pub const VK_MENU: u16 = 18;
pub const VK_MODECHANGE: u16 = 31;
pub const VK_MULTIPLY: u16 = 106;
pub const VK_N: u16 = 78;
pub const VK_NAVIGATION_ACCEPT: u16 = 142;
pub const VK_NAVIGATION_CANCEL: u16 = 143;
pub const VK_NAVIGATION_DOWN: u16 = 139;
pub const VK_NAVIGATION_LEFT: u16 = 140;
pub const VK_NAVIGATION_MENU: u16 = 137;
pub const VK_NAVIGATION_RIGHT: u16 = 141;
pub const VK_NAVIGATION_UP: u16 = 138;
pub const VK_NAVIGATION_VIEW: u16 = 136;
pub const VK_NEXT: u16 = 34;
pub const VK_NONAME: u16 = 252;
pub const VK_NONCONVERT: u16 = 29;
pub const VK_NUMLOCK: u16 = 144;
pub const VK_NUMPAD0: u16 = 96;
pub const VK_NUMPAD1: u16 = 97;
pub const VK_NUMPAD2: u16 = 98;
pub const VK_NUMPAD3: u16 = 99;
pub const VK_NUMPAD4: u16 = 100;
pub const VK_NUMPAD5: u16 = 101;
pub const VK_NUMPAD6: u16 = 102;
pub const VK_NUMPAD7: u16 = 103;
pub const VK_NUMPAD8: u16 = 104;
pub const VK_NUMPAD9: u16 = 105;
pub const VK_O: u16 = 79;
pub const VK_OEM_1: u16 = 186;
pub const VK_OEM_102: u16 = 226;
pub const VK_OEM_2: u16 = 191;
pub const VK_OEM_3: u16 = 192;
pub const VK_OEM_4: u16 = 219;
pub const VK_OEM_5: u16 = 220;
pub const VK_OEM_6: u16 = 221;
pub const VK_OEM_7: u16 = 222;
pub const VK_OEM_8: u16 = 223;
pub const VK_OEM_ATTN: u16 = 240;
pub const VK_OEM_AUTO: u16 = 243;
pub const VK_OEM_AX: u16 = 225;
pub const VK_OEM_BACKTAB: u16 = 245;
pub const VK_OEM_CLEAR: u16 = 254;
pub const VK_OEM_COMMA: u16 = 188;
pub const VK_OEM_COPY: u16 = 242;
pub const VK_OEM_CUSEL: u16 = 239;
pub const VK_OEM_ENLW: u16 = 244;
pub const VK_OEM_FINISH: u16 = 241;
pub const VK_OEM_FJ_JISHO: u16 = 146;
pub const VK_OEM_FJ_LOYA: u16 = 149;
pub const VK_OEM_FJ_MASSHOU: u16 = 147;
pub const VK_OEM_FJ_ROYA: u16 = 150;
pub const VK_OEM_FJ_TOUROKU: u16 = 148;
pub const VK_OEM_JUMP: u16 = 234;
pub const VK_OEM_MINUS: u16 = 189;
pub const VK_OEM_NEC_EQUAL: u16 = 146;
pub const VK_OEM_PA1: u16 = 235;
pub const VK_OEM_PA2: u16 = 236;
pub const VK_OEM_PA3: u16 = 237;
pub const VK_OEM_PERIOD: u16 = 190;
pub const VK_OEM_PLUS: u16 = 187;
pub const VK_OEM_RESET: u16 = 233;
pub const VK_OEM_WSCTRL: u16 = 238;
pub const VK_P: u16 = 80;
pub const VK_PA1: u16 = 253;
pub const VK_PACKET: u16 = 231;
pub const VK_PAUSE: u16 = 19;
pub const VK_PLAY: u16 = 250;
pub const VK_PRINT: u16 = 42;
pub const VK_PRIOR: u16 = 33;
pub const VK_PROCESSKEY: u16 = 229;
pub const VK_Q: u16 = 81;
pub const VK_R: u16 = 82;
pub const VK_RBUTTON: u16 = 2;
pub const VK_RCONTROL: u16 = 163;
pub const VK_RETURN: u16 = 13;
pub const VK_RIGHT: u16 = 39;
pub const VK_RMENU: u16 = 165;
pub const VK_RSHIFT: u16 = 161;
pub const VK_RWIN: u16 = 92;
pub const VK_S: u16 = 83;
pub const VK_SCROLL: u16 = 145;
pub const VK_SELECT: u16 = 41;
pub const VK_SEPARATOR: u16 = 108;
pub const VK_SHIFT: u16 = 16;
pub const VK_SLEEP: u16 = 95;
pub const VK_SNAPSHOT: u16 = 44;
pub const VK_SPACE: u16 = 32;
pub const VK_SUBTRACT: u16 = 109;
pub const VK_T: u16 = 84;
pub const VK_TAB: u16 = 9;
pub const VK_U: u16 = 85;
pub const VK_UP: u16 = 38;
pub const VK_V: u16 = 86;
pub const VK_VOLUME_DOWN: u16 = 174;
pub const VK_VOLUME_MUTE: u16 = 173;
pub const VK_VOLUME_UP: u16 = 175;
pub const VK_W: u16 = 87;
pub const VK_X: u16 = 88;
pub const VK_XBUTTON1: u16 = 5;
pub const VK_XBUTTON2: u16 = 6;
pub const VK_Y: u16 = 89;
pub const VK_Z: u16 = 90;
pub const VK_ZOOM: u16 = 251;


