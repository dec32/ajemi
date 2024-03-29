mod register;
mod global;
mod log;
mod conf;
mod extend;
mod tsf;
mod engine;
mod ui;

use std::{ffi::c_void, ptr, mem};
use ui::candidate_list;
use ::log::{debug, error};
use windows::{core::{implement, IUnknown, Interface, Result, GUID, HRESULT}, Win32::{Foundation::{BOOL, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, HINSTANCE, S_FALSE, S_OK}, System::{Com::{IClassFactory, IClassFactory_Impl}, SystemServices::DLL_PROCESS_ATTACH}, UI::TextServices::{ITfTextInputProcessor, ITfTextInputProcessorEx}}};
use global::*;
use register::*;

use crate::{extend::GUIDExt, tsf::TextService};

//----------------------------------------------------------------------------
//
//  Entry for the DLL
//
//----------------------------------------------------------------------------

#[no_mangle]
#[allow(non_snake_case, dead_code)]
extern "stdcall" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _reserved: *mut()) -> bool {
    if call_reason != DLL_PROCESS_ATTACH {
        return true;
    }
    log::setup();
    global::setup(dll_module);
    conf::setup();
    engine::setup();
    candidate_list::setup().is_ok()
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
    unsafe fn reg() -> Result<()> {
        register_server()?;
        register_ime()
    }
    match reg() {
        Ok(()) => {
            debug!("Registered server successfully.");
            S_OK
        },
        Err(err) => {
            error!("Failed to register server. {:?}", err);
            err.into()
        }
    }
}

// Unregister the IME from the OS. See register.rs.
#[no_mangle]
#[allow(non_snake_case, dead_code)]
unsafe extern "stdcall" fn DllUnregisterServer() -> HRESULT {
    unsafe fn unreg() -> Result<()> {
        unregister_ime()?;
        unregister_server()
    }
    match unreg() {
        Ok(()) => {
            debug!("Unegistered server successfully.");
            S_OK
        },
        Err(err) => {
            error!("Failed to unregister server. {:?}", err);
            err.into()
        }
    }
}

// Returns the required object. For a COM dll like an IME, the required object is always a class factory.
#[allow(non_snake_case, dead_code)]
#[no_mangle]
unsafe extern "stdcall" fn DllGetClassObject(_rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    // SomeInterface::from will move the object, thus we don't need to worry about the object's lifetime and management
    // the return value is a C++ vptr pointing to the moved object under the hood
    // *ppv = mem::transmute(&ClassFactory::new()) is incorrect and cause gray screen.
    debug!("DllGetClassObject({})", (*riid).to_rfc4122());
    let mut result = S_OK;
    *ppv = match *riid {
        IUnknown::IID => mem::transmute(IUnknown::from(ClassFactory::new())),
        IClassFactory::IID => mem::transmute(IClassFactory::from(ClassFactory::new())),
        guid => {
            error!("The required class object {} is not available.", guid.to_rfc4122());
            result = CLASS_E_CLASSNOTAVAILABLE;
            ptr::null_mut()
        }
    };
    result
    
}

#[no_mangle]
#[allow(non_snake_case, dead_code)]
unsafe extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    // todo: add ref count.
    // it seems not that of a important thing to do according to 
    // https://github.com/microsoft/windows-rs/issues/2472 tho
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
    fn new() -> ClassFactory {ClassFactory{}.into()}
}
#[allow(non_snake_case)]
impl IClassFactory_Impl for ClassFactory {
    fn CreateInstance(&self, _punkouter: Option<&IUnknown>, riid: *const GUID, ppvobject: *mut*mut c_void) -> Result<()> {
        debug!("CreateInstance({})", unsafe{ (*riid).to_rfc4122() });
        let mut result = Ok(());
        unsafe {
            *ppvobject = match *riid {
                ITfTextInputProcessor::IID => mem::transmute(
                    TextService::create::<ITfTextInputProcessor>()?),
                ITfTextInputProcessorEx::IID => mem::transmute(
                    TextService::create::<ITfTextInputProcessorEx>()?),
                guid => {
                    error!("The required instance {} is not available.", guid.to_rfc4122());
                    result = Err(E_NOINTERFACE.into());
                    ptr::null_mut()
                }
            };
        }
        result
    }

    fn LockServer(&self, flock: BOOL) -> Result<()> {
        debug!("LockServer({})", flock.as_bool());
        Ok(())
    }
}


//----------------------------------------------------------------------------
//
//  See tsf/mod.rs for the IME's implementation
//
//----------------------------------------------------------------------------