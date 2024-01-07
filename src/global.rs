use windows::Win32::Foundation::HINSTANCE;

pub static mut dll_module: Option<HINSTANCE> = None;