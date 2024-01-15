use std::{ffi::{OsString, OsStr, CString}, os::windows::ffi::OsStrExt};

use windows::core::{GUID, PCSTR};


pub trait GUIDExt {
    fn to_rfc4122(&self) -> String;
}

impl GUIDExt for GUID {
    fn to_rfc4122(&self) -> String {
        format!("{:X}", self.to_u128())
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

pub trait StrExt {
    fn to_bytes_with_nul(&self) -> Vec<u8>;
    fn to_pctr(&self) -> PCSTR;
}

impl StrExt for str {
    fn to_bytes_with_nul(&self) -> Vec<u8> {
        self.as_bytes().into_iter().cloned().chain(Some(0).into_iter()).collect()
    }
    fn to_pctr(&self) -> PCSTR {
        PCSTR::from_raw(CString::new(self).unwrap().as_bytes_with_nul().as_ptr())
    }
}