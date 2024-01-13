use std::ffi::{OsStr, OsString};

use crate::extend::OsStrExt2;


pub fn suggest(letters: &[u16]) -> Vec<u16> {
    if letters == OsStr::new("ike").wchars() {
        OsStr::new("㭗").wchars()
    } else {
        Vec::with_capacity(0)
    }
}