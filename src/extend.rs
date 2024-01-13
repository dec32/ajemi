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

/// A mutable wide-ecoded OsString
pub struct WChars(Vec<u16>);
impl WChars {
    pub fn with_capacity(capacity: usize) -> Self{
        WChars{0: Vec::with_capacity(capacity)}
    }
    fn from(text: &str) -> Self{
        WChars{0: OsStr::new(text).encode_wide().collect()}
    }

    fn null_terminated(mut self) -> Self{
        self.0.push(0);
        self
    }
}

impl<Idx> std::ops::Index<Idx> for WChars
where
    Idx: std::slice::SliceIndex<[u16]>
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}