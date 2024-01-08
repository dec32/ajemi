mod ime;
mod register;
mod global;
mod log;
mod extend;

use std::{ffi::c_void, ptr, mem};
use atomic_counter::AtomicCounter;
use ::log::{debug, error, trace};
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, S_FALSE}, System::{SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, Com::{IClassFactory, IClassFactory_Impl}}, UI::TextServices::ITfTextInputProcessor}, core::{GUID, HRESULT, implement, IUnknown, Result, ComInterface, Error}};
use global::*;
use ime::Ime;
use register::*;

use crate::extend::GUIDExt;

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
    trace!("DllRegisterServer");
    if register_server().is_ok() && register_ime().is_ok(){
        debug!("Registered server successfully.");
        S_OK
    } else {
        // TODO print the error
        error!("Failed to register server. Trying to unregister now.");
        let _ = DllUnregisterServer();
        E_FAIL
    }
}

// Unregister the IME from the OS. See register.rs.
#[no_mangle]
#[allow(non_snake_case, dead_code)]
unsafe extern "stdcall" fn DllUnregisterServer() -> HRESULT {
    trace!("DllUnregisterServer");
    let errors: Vec<Error> = [unregister_ime(), unregister_server()].into_iter()
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .collect();
    if errors.is_empty() {
        debug!("Unegistered server successfully.");
        S_OK
    } else {
        error!("Failed to unregister server.");
        for error in errors {
            error!("\t{}", error)
        }
        E_FAIL
    }
}

// Returns the required object. For a COM dll like an IME, the required object is always a class factory.
#[allow(non_snake_case, dead_code)]
#[no_mangle]
extern "stdcall" fn DllGetClassObject(_rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    trace!("DllGetClassObject");
    // SomeInterface::from will move the object, thus we don't need to worry about the object's lifetime and management
    // the return value is a C++ vptr pointing to the moved object under the hood
    // *ppv = mem::transmute(&ClassFactory::new()) is incorrect and cause gray screen.
    unsafe {
        let mut result = S_OK;
        *ppv = match *riid {
            IUnknown::IID => mem::transmute(IUnknown::from(ClassFactory::new())),
            IClassFactory::IID => mem::transmute(IClassFactory::from(ClassFactory::new())),
            _guid => {
                error!("The required interface {{{}}} is not available.", _guid.to_rfc4122());
                result = CLASS_E_CLASSNOTAVAILABLE;
                ptr::null_mut()
            }
        };
        result
    }
}

#[no_mangle]
#[allow(non_snake_case, dead_code)]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    // todo: add ref count.
    // it seems not that of a important thing to do according to https://github.com/microsoft/windows-rs/issues/2472 tho
    trace!("DllCanUnloadNow");
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
        trace!("CreateInstance");
        unsafe {
            *ppv = match *riid {
                ITfTextInputProcessor::IID =>
                ITfTextInputProcessorEx::IID =>
            }
            // There're way too may interfaces so we'll leave that for the IME object itself to handle
            Ime::new().query_interface(riid, ppvobject)
        }
    }

    #[allow(non_snake_case)]
    fn LockServer(&self, _flock: BOOL) -> Result<()> {
        trace!("LockServer");
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
