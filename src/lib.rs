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
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, S_FALSE, E_NOINTERFACE}, System::{Com::{IClassFactory, IClassFactory_Impl}, SystemServices::DLL_PROCESS_ATTACH}, UI::TextServices::{ITfTextInputProcessor, ITfTextInputProcessorEx}}, core::{GUID, HRESULT, implement, IUnknown, Result, ComInterface, Error}};
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
    let errors: Vec<Error> = [unregister_ime(), unregister_server()].into_iter()
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .collect();
    if errors.is_empty() {
        debug!("Unegistered server successfully.");
        S_OK
    } else {
        error!("Failed to unregister server. {:?}", errors);
        // FIXME should've been E_FAIL but I don't want to bother the user.
        S_OK
    }
}

// Returns the required object. For a COM dll like an IME, the required object is always a class factory.
#[allow(non_snake_case, dead_code)]
#[no_mangle]
extern "stdcall" fn DllGetClassObject(_rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    // SomeInterface::from will move the object, thus we don't need to worry about the object's lifetime and management
    // the return value is a C++ vptr pointing to the moved object under the hood
    // *ppv = mem::transmute(&ClassFactory::new()) is incorrect and cause gray screen.
    unsafe {
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
}

#[no_mangle]
#[allow(non_snake_case, dead_code)]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
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