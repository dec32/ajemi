mod ime;
use std::ffi::c_void;
use windows::{Win32::{Foundation::{HINSTANCE, S_OK, BOOL}, System::{SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}, Com::{IClassFactory, IClassFactory_Impl}}}, core::{GUID, HRESULT, implement, IUnknown, Result}};


//----------------------------------------------------------------------------
//
//  The four exposed functions.
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

// 看起来进程和 dll 交互时会调用这里，并把自己的 ID 传过来
#[allow(non_snake_case, dead_code)]
extern "system" fn DllGetClassObject(_rclsid: *const GUID, _riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    // todo what to do here
    // 把一个 ICClassFactory 对象赋值给 ppv
    // 这个工厂对象负责创建 IME 实例
    // IME 实例需要继承 ITfTextInputProcessorEx
    // 逻辑大概就在这个实现类里面了
    todo!();
    S_OK
}

#[allow(non_snake_case, dead_code)]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    // todo what to do here
    todo!();
    S_OK
}

#[allow(non_snake_case, dead_code)]
extern "system" fn DllRegisterServer() -> HRESULT {
    todo!();
    S_OK
}

#[allow(non_snake_case, dead_code)]
extern "system" fn DllUnregisterServer() -> HRESULT {
    todo!();
    S_OK
}


//----------------------------------------------------------------------------
//
//  ClassFactory. It creates IME instances.
//
//----------------------------------------------------------------------------

#[implement(IClassFactory)]
struct ClassFactory{
    
}

impl ClassFactory {

}

impl IClassFactory_Impl for ClassFactory {
    #[allow(non_snake_case)]
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
