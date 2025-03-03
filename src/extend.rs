use std::{char::DecodeUtf16Error, ffi::{OsStr, OsString}, os::windows::ffi::OsStrExt};
use toml::{Table, Value};
use windows::{core::GUID, Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY}};

pub trait ResultExt{
    fn watch(self) -> Self;
}

impl<T,E: std::error::Error> ResultExt for std::result::Result<T, E> {
    fn watch(self) -> Self {
        if let Err(e) = self.as_ref() {
            log::error!("{e:#}")
        }
        self
    }
}

pub trait GUIDExt {
    fn to_rfc4122(&self) -> String;
}

impl GUIDExt for GUID {
    fn to_rfc4122(&self) -> String {
        let mut buf = String::new();
        let hex = format!("{:032X}", self.to_u128());
        buf.push_str(&hex[0..8]);
        buf.push('-');
        buf.push_str(&hex[8..12]);
        buf.push('-');
        buf.push_str(&hex[12..16]);
        buf.push('-');
        buf.push_str(&hex[16..20]);
        buf.push('-');
        buf.push_str(&hex[20..32]);
        buf
    }
}

pub trait OsStrExt2 {
    fn wchars(&self) -> Vec<u16>;
    fn null_terminated_wchars(&self) -> Vec<u16>;
}

impl OsStrExt2 for OsStr {
    fn wchars(&self) -> Vec<u16>{
        self.encode_wide().collect()
    }
    fn null_terminated_wchars(&self) -> Vec<u16>{
        self.encode_wide().chain(Some(0).into_iter()).collect()
    }
}

impl OsStrExt2 for OsString {
    fn wchars(&self) -> Vec<u16>{
        self.encode_wide().collect()
    }
    fn null_terminated_wchars(&self) -> Vec<u16>{
        self.encode_wide().chain(Some(0).into_iter()).collect()
    }
}

pub trait StringExt {
    fn push_chars(&mut self, chars: &[char]);
}

impl StringExt for String {
    fn push_chars(&mut self, chars: &[char]) {
        for ch in chars {
            self.push(*ch);
        }
    }
}
pub trait CharExt {
    fn is_joiner(self) -> bool;
    fn try_from_utf16(value: u16) -> Result<char, DecodeUtf16Error>;
}

impl CharExt for char {
    fn is_joiner(self) -> bool {
        matches!(self, '\u{F1995}' | '\u{F1996}' | '\u{200D}')
    }
    
    fn try_from_utf16(value: u16) -> Result<char, DecodeUtf16Error> {
        char::decode_utf16(std::iter::once(value)).next().unwrap()
    }
}

pub trait LoadValue where Self: Sized {
    fn load(&mut self, value: Value);
}

impl LoadValue for bool {
    fn load(&mut self, value: Value) {
        if let Value::Boolean(value) = value {
            *self = value;
        }
    }
}

impl LoadValue for String {
    fn load(&mut self, value: Value) {
        if let Value::String(value) = value {
            *self = value;
        }
    }
}

impl LoadValue for i32 {
    fn load(&mut self, value: Value) {
        if let Value::Integer(value) = value {
            *self = value as i32;
        }
    }
}


pub trait TableExt {
    fn give<T: LoadValue>(&mut self, key: &str, out: &mut T);
}

impl TableExt for Table {
    fn give<T: LoadValue>(&mut self, key: &str, out: &mut T) {
        if let Some(value) = self.remove(key) {
            out.load(value)
        }
    }
    
}

pub trait VKExt {
    fn is_down(self) -> bool;
    fn is_toggled(self) -> bool;
}

impl VKExt for VIRTUAL_KEY {
    fn is_down(self) -> bool {
        unsafe {
            GetKeyState(self.0 as i32) as u16 & 0x8000 != 0
        }
    }

    fn is_toggled(self) -> bool {
        unsafe {
            GetKeyState(self.0 as i32) as u16 & 1 != 0
        }
    }
}