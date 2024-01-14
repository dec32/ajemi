use std::{ffi::{OsString, OsStr}, os::windows::ffi::OsStrExt};

use windows::core::GUID;


pub trait GUIDExt {
    fn to_rfc4122(&self) -> String;
}

impl GUIDExt for GUID {
    fn to_rfc4122(&self) -> String {
        format!("{:X}", self.to_u128())
    }
}

pub trait OsStrExt2 {
    fn null_terminated_wchars(&self) -> Vec<u16>;
    fn wchars(&self) -> Vec<u16>;
}

impl OsStrExt2 for OsStr {
    fn null_terminated_wchars(&self) -> Vec<u16>{
        self.encode_wide().chain(Some(0).into_iter()).collect()
    }

    fn wchars(&self) -> Vec<u16>{
        self.encode_wide().collect()
    }
}

impl OsStrExt2 for OsString {
    fn null_terminated_wchars(&self) -> Vec<u16>{
        self.encode_wide().chain(Some(0).into_iter()).collect()
    }

    fn wchars(&self) -> Vec<u16>{
        self.encode_wide().collect()
    }
}