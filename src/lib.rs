mod ime;
mod register;
mod consts;
use std::{mem, ptr, ffi::c_void};
use register::*;
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL, CLASS_E_CLASSNOTAVAILABLE, E_FAIL}, System::{SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, Com::{IClassFactory, IClassFactory_Impl, CoCreateInstance, CoCreateInstanceEx, CLSCTX_INPROC_SERVER, MULTI_QI}}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles}}, core::{GUID, HRESULT, implement, IUnknown, Result, ComInterface}};


//----------------------------------------------------------------------------
//
//  Entry for the DLL
//
//----------------------------------------------------------------------------

#[allow(non_snake_case, dead_code)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut()) -> bool {
    // 传递句柄、注册窗口等
    match call_reason {
        DLL_PROCESS_ATTACH => {
            todo!()
        },
        DLL_PROCESS_DETACH => {
            todo!()
        },
        _ => {
            todo!()
        }
    }
    true
}

//----------------------------------------------------------------------------
//
//  The four exposed functions.
//
//----------------------------------------------------------------------------

// Returns the factory object.
#[allow(non_snake_case, dead_code)]
extern "system" fn DllGetClassObject(_rclsid: *const GUID, rrid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    unsafe {
        // rrid probably stands for required interface ID, idk
        if *rrid != IClassFactory::IID && *rrid != IUnknown::IID {
            CLASS_E_CLASSNOTAVAILABLE
        } else {
            *ppv = mem::transmute(&CLASS_FACTORY);
            S_OK
        }
    }
}

// Register the IME into the OS. See register.rs.
#[allow(non_snake_case, dead_code)]
unsafe extern "system" fn DllRegisterServer() -> HRESULT {
    if !register_server() || !register_profiles() || !register_categories() {
        DllUnregisterServer();
        E_FAIL
    } else {
        S_OK
    }
}

// Unregister the IME from the OS.
#[allow(non_snake_case, dead_code)]
extern "system" fn DllUnregisterServer() -> HRESULT {
    todo!();
    S_OK
}


#[allow(non_snake_case, dead_code)]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    // todo ref count maybe?
    todo!();
    S_OK
}


//----------------------------------------------------------------------------
//
//  ClassFactory. It creates IME instances.
//
//----------------------------------------------------------------------------

const CLASS_FACTORY: ClassFactory= ClassFactory{};
#[implement(IClassFactory)]
struct ClassFactory{
    
}

impl IClassFactory_Impl for ClassFactory {
    #[allow(non_snake_case)]
    // Get the IME instance and convert it to a interface pointer
    fn CreateInstance(&self, punkouter: Option<&IUnknown>, riid: *const GUID, ppvobject: *mut*mut c_void) -> Result<()> {
        todo!()
        // riid 用来标记所请求的 interface 的类型
        // 根据 interface 获取到对应的对象后，将地址返回给 ppvobject
        // 可以让一个 struct 实现多个 interface 然后强转
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
