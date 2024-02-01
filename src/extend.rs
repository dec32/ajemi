use std::{ffi::{OsString, OsStr}, fmt::Debug, mem::ManuallyDrop, os::windows::ffi::OsStrExt};
use toml::{Table, Value};
use windows::{core::GUID, Win32::System::Variant::{VARIANT, VARIANT_0, VARIANT_0_0, VARIANT_0_0_0, VT_I4}};
pub trait GUIDExt {
    fn to_rfc4122(&self) -> String;
}

impl GUIDExt for GUID {
    fn to_rfc4122(&self) -> String {
        let mut buf = String::new();
        let hex = format!("{:X}", self.to_u128());
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
pub trait ResultExt {
    fn ignore(self);
}
impl <T, E:Debug> ResultExt for Result<T, E> {
    fn ignore(self) {}
}
pub trait VARANTExt {
    fn i4(value: i32) -> Self;
}
// modified from variant.rs in https://github.com/microsoft/windows-rs/pull/2786/files
impl VARANTExt for VARIANT {
    fn i4(value: i32) -> Self {
        VARIANT {
            Anonymous: VARIANT_0 {
                Anonymous: ManuallyDrop::new(VARIANT_0_0 {
                    vt: VT_I4, 
                    wReserved1: 0, 
                    wReserved2: 0, 
                    wReserved3: 0, 
                    Anonymous: VARIANT_0_0_0 { 
                        lVal: value
                    } 
                })
            }
        }
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