use std::{ffi::OsStr, os::windows::ffi::OsStrExt, sync::OnceLock};
use windows::core::GUID;


// Randomly generated class ID for the input method.
pub fn ime_id() -> &'static GUID {
    static IME_ID: OnceLock<GUID> = OnceLock::new();
    IME_ID.get_or_init(||GUID::from("C93D3D59-2FAC-40E0-ABC6-A3658749E2FA"))
}

// Use en-US for now until Microsoft recognized tok
pub fn lang_id() -> u16 {
    0x409
}

// Randomly generated GUID for the language profile.
pub fn lang_profile_id() -> &'static GUID {
    static IME_ID: OnceLock<GUID> = OnceLock::new();
    IME_ID.get_or_init(||GUID::from("A411A7FC-A082-4B8A-8741-AA4A72613933"))
}

// The description must be in WCHAR
pub fn ime_name() -> &'static[u16] {
    static IME_NAME:OnceLock<Vec<u16>> = OnceLock::new();
    IME_NAME.get_or_init(||OsStr::new("Ajemi").encode_wide().chain(Some(0).into_iter()).collect())
}

// The path(?) of the file (.ico, .dll or .exe) that contains the icon. Here a .ico file is used for simplicity.
pub fn icon_file() -> &'static[u16] {
    static ICON_FILE:OnceLock<Vec<u16>> = OnceLock::new();
    ICON_FILE.get_or_init(||OsStr::new("./res/icon.ico").encode_wide().chain(Some(0).into_iter()).collect())
} 
