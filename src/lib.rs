mod ime;
mod register;
mod global;
mod log;

use std::{ffi::c_void, ptr, mem, sync::OnceLock};
use ::log::{debug, error};
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, S_FALSE}, System::{SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, Com::{IClassFactory, IClassFactory_Impl}}}, core::{GUID, HRESULT, implement, IUnknown, Result, ComInterface}};
use global::*;
use ime::Ime;
use register::*;

//----------------------------------------------------------------------------
//
//  Entry for the DLL
//
//----------------------------------------------------------------------------

#[no_mangle]
#[allow(non_snake_case, dead_code)]
extern "stdcall" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _reserved: *mut()) -> bool {
    let _ = log::setup();
    // store dll_module for later use
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe { DLL_MOUDLE = Some(dll_module) };
        },
        DLL_PROCESS_DETACH => {
            unsafe { DLL_MOUDLE = None }
        },
        _ => {

        }
    }
    true
}

//----------------------------------------------------------------------------
//
//  The four exposed functions.
//
//----------------------------------------------------------------------------


// Register the IME into the OS. See register.rs.
#[no_mangle]
#[allow(non_snake_case, dead_code)]
unsafe extern "stdcall" fn DllRegisterServer() -> HRESULT {
    debug!("Registering server.");
    if register_server().is_ok() && register_ime().is_ok(){
        debug!("Registered server successfully.");
        S_OK
    } else {
        error!("Failed to register server.");
        let _ = DllUnregisterServer();
        E_FAIL
    }
}

// Unregister the IME from the OS. See register.rs.
#[no_mangle]
#[allow(non_snake_case, dead_code)]
unsafe extern "stdcall" fn DllUnregisterServer() -> HRESULT {
    debug!("Unregistering server.");
    if unregister_ime().is_ok() && unregister_server().is_ok() {
        debug!("Unegistered server successfully.");
        S_OK
    } else {
        error!("Failed to unregister server.");
        E_FAIL
    }
}

// Returns the required object. For a COM dll like an IME, the required object is always a class factory.
#[allow(non_snake_case, dead_code)]
#[no_mangle]
extern "stdcall" fn DllGetClassObject(_rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    debug!("Creating class objects.");
    // SomeInterface::from will move the object, thus we don't need to worry about the object's lifetime and management
    // the return value is a C++ vptr pointing to the moved object under the hood
    unsafe {
        match *riid {
            IUnknown::IID => {
                debug!("IUnknown is required.");
                // *ppv = mem::transmute(&ClassFactory::new()) is incorrect and will crash the system
                *ppv = mem::transmute(IUnknown::from(ClassFactory::new()));
                S_OK
            },
            IClassFactory::IID => {
                debug!("IClassFactory is required.");
                *ppv = mem::transmute(IClassFactory::from(ClassFactory::new()));
                S_OK
            },
            _ => {
                error!("The required interface is not available.");
                *ppv = ptr::null_mut();
                CLASS_E_CLASSNOTAVAILABLE
            }
        }
    }
}

#[no_mangle]
#[allow(non_snake_case, dead_code)]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    debug!("DllCanUnloadNow");
    // todo ref count maybe?
    S_FALSE
}


//----------------------------------------------------------------------------
//
//  ClassFactory. It creates nothing but IME instances.
//
//----------------------------------------------------------------------------

#[implement(IClassFactory)]
struct ClassFactory;

impl ClassFactory {
    fn new() -> ClassFactory {ClassFactory{}}
}

impl IClassFactory_Impl for ClassFactory {
    #[allow(non_snake_case)]
    fn CreateInstance(&self, _punkouter: Option<&IUnknown>, riid: *const GUID, ppvobject: *mut*mut c_void) -> Result<()> {
        debug!("Creating IME instance.");
        unsafe {
            // There're way to may interfaces so we'll leave that for the ime instance itself to handle
            Ime::new().query_interface(riid, ppvobject)
        }
    }

    #[allow(non_snake_case)]
    fn LockServer(&self, _flock: BOOL) -> Result<()> {
        debug!("LockServer");
        Ok(())
    }
}


//----------------------------------------------------------------------------
//
//  See ime/mod.rs for the IME's implementation
//
//----------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}
