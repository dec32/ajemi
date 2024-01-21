use std::{ffi::{OsString, OsStr}, os::windows::ffi::OsStrExt, fmt::Debug};
use log::error;
use try_lock::{TryLock, Locked};
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

// todo use crate: log_derive
pub trait ResultExt {
    fn log_error(self) -> Self;
    fn ignore(self);
}
impl <T, E:Debug> ResultExt for Result<T, E> {
    fn log_error(self) -> Self {
        if let Err(e) = self.as_ref() {
            error!("\t{:?}", e);
        }
        self
    }

    fn ignore(self) {
        
    }
}


pub trait TryLockExt<T> {
    fn spin(&self, tries: u8) -> Option <Locked<T>>;
}

impl <T> TryLockExt<T> for TryLock<T> {
    fn spin(&self, tries: u8) -> Option<Locked<T>> {
        for _ in 0..tries {
            let locked = self.try_lock();
            if locked.is_some() {
                return locked;
            }
        }
        return None;
    }
}