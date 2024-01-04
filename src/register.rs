use std::{mem, ptr};
use windows::{Win32::{System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles}}, core::IUnknown};

use crate::consts::AJEMI_CLSID;


//----------------------------------------------------------------------------
//(
//  Functions for DllRegisterServer() and DllUnregisterServer()
//
//----------------------------------------------------------------------------


pub unsafe fn register_server() -> bool{
    // todo how do you pass NULL into punkouter
    CoCreateInstance(
        mem::transmute(&CLSID_TF_InputProcessorProfiles),
        ptr::null, 
        CLSCTX_INPROC_SERVER)
        .map(|it:ITfInputProcessorProfiles|it.Register(mem::transmute(&AJEMI_CLSID)))
        .is_ok()
}

pub unsafe fn register_profiles() -> bool {
    todo!()
}

pub unsafe fn register_categories() -> bool{
    todo!()
}