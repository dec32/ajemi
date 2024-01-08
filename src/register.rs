use std::{mem, ffi::{OsStr, CString}, os::windows::ffi::OsStrExt};
use log::debug;
use windows::core::IntoParam;
use windows::{Win32::{System::{Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, Registry::{KEY_WRITE, REG_OPTION_NON_VOLATILE, HKEY_CLASSES_ROOT, HKEY, REG_CREATE_KEY_DISPOSITION, RegSetValueExA, REG_SZ, RegCloseKey, RegDeleteKeyA, RegOpenKeyA}, LibraryLoader::GetModuleFileNameA}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr, CLSID_TF_CategoryMgr, GUID_TFCAT_CATEGORY_OF_TIP, GUID_TFCAT_TIP_KEYBOARD, GUID_TFCAT_TIPCAP_SECUREMODE, GUID_TFCAT_TIPCAP_UIELEMENTENABLED, GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT, GUID_TFCAT_TIPCAP_COMLESS, GUID_TFCAT_TIPCAP_WOW16, GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT, GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, GUID_TFCAT_PROP_AUDIODATA, GUID_TFCAT_PROP_INKDATA, GUID_TFCAT_PROPSTYLE_STATIC, GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER, GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY}}, core::{Result, GUID, ComInterface, PCSTR, HSTRING}};
use crate::global::*;
use windows::Win32::System::Registry::RegCreateKeyExA;


//----------------------------------------------------------------------------
//
//  Registation for standard COM in-proc servers of any kind.
//  An IME is one of these servers.
//
//----------------------------------------------------------------------------


// FIXME 无法注册到注册表中
pub unsafe fn register_server() -> Result<()> {
    
    // creating key: HKEY_CLASSES_ROOT/CLSID/{IME_ID}
    let mut ime_id: HKEY = mem::zeroed();
    let mut disposition: REG_CREATE_KEY_DISPOSITION = mem::zeroed();

    RegCreateKeyExA(
        HKEY_CLASSES_ROOT, 
        pcstr(&format!("CLSID\\{{{IME_ID}}}")), 
        0, 
        None, 
        REG_OPTION_NON_VOLATILE, 
        KEY_WRITE, 
        None, 
        &mut ime_id as *mut HKEY, 
        Some(&mut disposition as *mut REG_CREATE_KEY_DISPOSITION))?;

    // Register the IME's ASCII name under HKEY_CLASSES_ROOT/CLSID/{IME_ID}
    RegSetValueExA(ime_id, None, 0, REG_SZ, Some(IME_NAME_ASCII.as_bytes()))?;


    // creating key: HKEY_CLASSES_ROOT/CLSID/{IME_ID}/InprocServer32
    let mut inproc_server_32: HKEY = mem::zeroed();
    RegCreateKeyExA(
        ime_id, 
        pcstr("InprocServer32"), 
        0, 
        None, 
        REG_OPTION_NON_VOLATILE, 
        KEY_WRITE, 
        None, 
        &mut inproc_server_32 as *mut HKEY, 
        Some(&mut disposition as *mut REG_CREATE_KEY_DISPOSITION))?;

    // register the dll file under HKEY_CLASSES_ROOT/{IME_ID}/InprocServer32
    let mut file_name: Vec<u8> = Vec::with_capacity(260);
    GetModuleFileNameA(DLL_MOUDLE.unwrap(), &mut file_name);
    RegSetValueExA(inproc_server_32, None, 0, REG_SZ, Some(&file_name))?;
    debug!("Registered dll path: {}", String::from_utf8(file_name.clone()).unwrap());

    // register the thread model under HKEY_CLASSES_ROOT/{IME_ID}/InprocServer32
    RegSetValueExA(inproc_server_32, pcstr("ThreadingModel") , 0, REG_SZ, Some("Apartment".as_bytes()))?;
    RegCloseKey(inproc_server_32)?;
    RegCloseKey(ime_id)?;
    Ok(())
}

pub unsafe fn unregister_server() -> Result<()> {

    // is it neccessary to delete the reg key recursively?
    #[allow(dead_code)]
    unsafe fn reg_delete<T>(key: HKEY, subkey: T) -> Result<()> 
    where
        T:IntoParam<PCSTR> 
    {
        let mut out = unsafe{mem::zeroed()};
        RegOpenKeyA(key, subkey, &mut out)
    }

    RegDeleteKeyA(HKEY_CLASSES_ROOT, pcstr(IME_ID))?;
    Ok(())
}

// convert a stand UTF-8 &str into a pointer to a null-terminated C string
#[inline]
fn pcstr(text: &str) -> PCSTR{
    PCSTR::from_raw(CString::new(text).unwrap().as_bytes().as_ptr())
}



//----------------------------------------------------------------------------
//
//  Registration for an IME.
//
//----------------------------------------------------------------------------


// features supported by the IME
const SUPPORTED_CATEGORIES: [GUID;16] = [
    GUID_TFCAT_CATEGORY_OF_TIP,
    GUID_TFCAT_TIP_KEYBOARD,
    GUID_TFCAT_TIPCAP_SECUREMODE,
    GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
    GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
    GUID_TFCAT_TIPCAP_COMLESS,
    GUID_TFCAT_TIPCAP_WOW16,
    GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
    GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
    GUID_TFCAT_PROP_AUDIODATA,
    GUID_TFCAT_PROP_INKDATA,
    GUID_TFCAT_PROPSTYLE_STATIC,
    GUID_TFCAT_PROPSTYLE_STATIC,
    GUID_TFCAT_PROPSTYLE_STATIC,
    GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER,
    GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY
];

pub unsafe fn register_ime() -> Result<()> {
    // some COM nonsense to create the registry objects.
    let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
        &CLSID_TF_InputProcessorProfiles as *const GUID, 
        None, 
        CLSCTX_INPROC_SERVER)?;
    let category_mgr: ITfCategoryMgr = CoCreateInstance(
        &CLSID_TF_CategoryMgr as *const GUID, 
        None, 
        CLSCTX_INPROC_SERVER)?;

    // three things to register:
    // 1. the IME itself
    // 2. language profile
    // 3. categories(the features the IME has)

    let ime_id = &GUID::from(IME_ID) as *const GUID;
    let lang_profile_id = &GUID::from(LANG_PROFILE_ID) as *const GUID;

    input_processor_profiles.Register(ime_id)?;

    // todo the icon cannot be registered
    let ime_name: Vec<u16> = OsStr::new(IME_NAME).encode_wide().chain(Some(0).into_iter()).collect();
    let icon_file: Vec<u16> = OsStr::new(ICON_FILE).encode_wide().chain(Some(0).into_iter()).collect();
    input_processor_profiles.AddLanguageProfile(ime_id, LANG_ID, lang_profile_id, &ime_name, &icon_file, 0)?;

    for rcatid  in SUPPORTED_CATEGORIES {
        let rcatid = &rcatid as *const GUID;
        category_mgr.RegisterCategory(ime_id, rcatid, ime_id)?;
    }
    Ok(())
}

// similar process but re-doing everything
pub unsafe fn unregister_ime() -> Result<()> {
    // todo: it seems able to unregister the dll but alaways exits with 0x80004005
    let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
        &CLSID_TF_InputProcessorProfiles as *const GUID, // using ::IID would cause unregister to fail
        None, 
        CLSCTX_INPROC_SERVER)?;
    let category_mgr: ITfCategoryMgr = CoCreateInstance(
        &CLSID_TF_CategoryMgr as *const GUID, 
        None, 
        CLSCTX_INPROC_SERVER)?;


    let ime_id = &GUID::from(IME_ID) as *const GUID;
    let lang_profile_id = &GUID::from(LANG_PROFILE_ID) as *const GUID;

    input_processor_profiles.Unregister(ime_id)?;
    input_processor_profiles.RemoveLanguageProfile(ime_id, LANG_ID, lang_profile_id)?;
    for rcatid in SUPPORTED_CATEGORIES {
        let rcatid = &rcatid as *const GUID;
        category_mgr.UnregisterCategory(ime_id, rcatid, ime_id)?;
    }
    Ok(())
}


