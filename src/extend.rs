use std::{ffi::c_void, ptr, mem};
use atomic_counter::AtomicCounter;
use ::log::{debug, error, trace};
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, S_FALSE}, System::{SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, Com::{IClassFactory, IClassFactory_Impl}}}, core::{GUID, HRESULT, implement, IUnknown, Result, ComInterface, Error}};


// fix the stinky windows APIs 
// pub trait AsInterface {
//     fn as_interface<I:ComInterface>(&self) -> Result<&I>;
// }

// impl <T> AsInterface for T
// where T:ComInterface
// {
//     fn as_interface<I:ComInterface>(&self) -> Result<&I> {
//         unsafe {
//             let mut interface: *mut c_void = mem::zeroed();
//             let res = self.query(&I::IID, &mut interface);
//             if res != S_OK {
//                 return Err(Error::from(res));
//             } else {
//                 Ok(mem::transmute(interface))
//             }
//         }
//     }
// }

pub trait GUIDExt{
    fn to_rfc4122(&self) -> String;
}

impl GUIDExt for GUID {
    fn to_rfc4122(&self) -> String {
        format!("{:X}", self.to_u128())
    }
}