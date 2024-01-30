use std::mem;

use windows::{core::GUID, Win32::Foundation::HINSTANCE};

// global variables
static mut DLL_MODULE: HINSTANCE = unsafe{ mem::zeroed() };
pub fn setup(dll_module: HINSTANCE) {
    unsafe {
        DLL_MODULE = dll_module;
    }
}

pub fn dll_module() -> HINSTANCE {
    unsafe{ DLL_MODULE }
}

// todo migrate the constants into a config file
pub const IME_NAME: &str = "Ajemi";
pub const IME_NAME_ASCII: &str = "Ajemi";
pub const IME_ID: GUID = GUID::from_u128(0xC93D3D59_2FAC_40E0_ABC6_A3658749E2FA);
pub const LANG_ID: u16 = 0x409; // en-US
pub const LANG_PROFILE_ID: GUID = GUID::from_u128(0xA411A7FC_A082_4B8A_8741_AA4A72613933);
pub const LANGBAR_ITEM_ID: GUID = GUID::from_u128(0x95288B2B_4D3B_4D4A_BF5B_9342E4F75E4D);
pub const DISPLAY_ATTR_ID: GUID = GUID::from_u128(0xE42647FB_4BF0_4570_9013_768487C5CAAE);
pub const FONT: &str = "Fairfax HD";
pub const FONT_SIZE: i32 = 14;

