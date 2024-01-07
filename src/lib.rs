mod ime;
mod register;
mod global;
mod log;

use std::{ffi::c_void, ptr};
use ::log::{debug, error};
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL}, System::{SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, Com::{IClassFactory, IClassFactory_Impl}}}, core::{GUID, HRESULT, implement, IUnknown, Result, ComInterface}};
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
    debug!("Registering server...");
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
    debug!("Unregistering server...");
    if unregister_ime().is_ok() && unregister_server().is_ok() {
        debug!("Unegistered server successfully.");
        S_OK
    } else {
        error!("Failed to unregister server.");
        E_FAIL
    }
}


#[no_mangle]
#[allow(non_snake_case, dead_code)]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    debug!("DllCanUnloadNow");
    // todo ref count maybe?
    S_OK
}

// Returns the factory object.
#[allow(non_snake_case, dead_code)]
#[no_mangle]
extern "stdcall" fn DllGetClassObject(_rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    debug!("Getting class objects...");
    unsafe {
        if *riid != IClassFactory::IID && *riid != IUnknown::IID {
            error!("Unrecognizable riid.");
            CLASS_E_CLASSNOTAVAILABLE
        } else {
            debug!("Got the class object successfully.");
            *ppv = &mut CLASS_FACTORY  as *mut _ as *mut c_void;
            S_OK
        }
    }
}


//----------------------------------------------------------------------------
//
//  ClassFactory. It creates IME instances.
//
//----------------------------------------------------------------------------

static mut CLASS_FACTORY: ClassFactory= ClassFactory{};
#[implement(IClassFactory)]
struct ClassFactory{
    
}

// for now the Ime struct is completely stateless
static mut IME: Ime = Ime{};
impl IClassFactory_Impl for ClassFactory {
    #[allow(non_snake_case)]
    // Get the IME instance and convert it to a interface pointer
    fn CreateInstance(&self, _punkouter: Option<&IUnknown>, riid: *const GUID, ppvobject: *mut*mut c_void) -> Result<()> {
        debug!("Creating instance...");
        // riid: requested interface id
        // todo: the ime instance will be recycled by rust compiler, creating a dangling ptr
        let mut ime = Ime::new();
        unsafe {
            *ppvobject = ptr::null_mut();
            // *ppvobject = ime.query_interface(riid)?;
            *ppvobject = IME.query_interface(riid)?;
        }
        Ok(())
    }

    #[allow(non_snake_case)]
    fn LockServer(&self, flock: BOOL) -> Result<()> {
        todo!()
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
        todo!()
    }
}
