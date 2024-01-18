use std::ffi::{OsStr, OsString};
use std::fs;
use log::{debug, warn, error};
use windows::Win32::Foundation::E_FAIL;
use windows::core::{Result, GUID, Error};
use windows::Win32::{System::{Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, LibraryLoader::GetModuleFileNameA}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr, CLSID_TF_CategoryMgr, GUID_TFCAT_CATEGORY_OF_TIP, GUID_TFCAT_TIP_KEYBOARD, GUID_TFCAT_TIPCAP_SECUREMODE, GUID_TFCAT_TIPCAP_UIELEMENTENABLED, GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT, GUID_TFCAT_TIPCAP_COMLESS, GUID_TFCAT_TIPCAP_WOW16, GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT, GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, GUID_TFCAT_PROP_AUDIODATA, GUID_TFCAT_PROP_INKDATA, GUID_TFCAT_PROPSTYLE_STATIC, GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER, GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY}};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use crate::{global::*, extend::OsStrExt2};


//----------------------------------------------------------------------------
//
//  Registation for standard COM in-proc servers of any kind.
//  An IME is one of these servers.
//
//----------------------------------------------------------------------------


// FIXME these unwrappings...
pub unsafe fn register_server() -> Result<()> {
    // Register the IME's ASCII name under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SOFTWARE\\Classes\\CLSID\\{{{}}}", IME_ID);
    let (clsid, _) = hklm.create_subkey(path).unwrap();
    clsid.set_value("", &IME_NAME_ASCII).unwrap();
    // Register the dll's path under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}\InprocServer32 
    let (inproc_server_32, _) = clsid.create_subkey("InprocServer32").unwrap();
    let dll_path = find_dll_path()?;
    inproc_server_32.set_value("", &dll_path).unwrap();
    // Register the threading model under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}\InprocServer32
    inproc_server_32.set_value("ThreadingModel", &"Apartment").unwrap();
    Ok(())
}

unsafe fn find_dll_path() -> Result<OsString> {
    let mut buf: Vec<u8> = Vec::with_capacity(260);
    // FIXME the buf is always empty
    let handle = DLL_MOUDLE.map(|it|format!("Some({:#0X})", it.0)).unwrap_or("None".to_string());
    debug!("Handle to the dll module is {handle}");
    GetModuleFileNameA(DLL_MOUDLE.unwrap(), &mut buf);
    debug!("Result of GetModuleFileNameA: {:?}", buf);
    if !buf.is_empty() {
        let path = OsString::from_encoded_bytes_unchecked(buf);
        debug!("Found dll in {:?}", path);
        return Ok(path);
    }
    // GetModuleFileNameA tends to fail so try a few more options
    warn!("GetModuleFileNameA did not provide the path of the DLL file. Stupid M$.");
    for path in [".\\target\\debug\\ajemi.dll", ".\\ajemi.dll"] {
        if let Ok(canonical_path) = fs::canonicalize(path) {
            debug!("Found dll in {path}");
            return Ok(canonical_path.into_os_string())
        }     
    }
    error!("Failed to find the dll path.");
    return Err(Error::from(E_FAIL));
}

pub unsafe fn unregister_server() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SOFTWARE\\Classes\\CLSID\\{{{}}}", IME_ID);
    hklm.delete_subkey_all(path).unwrap();
    Ok(())
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
        &CLSID_TF_InputProcessorProfiles, 
        None, 
        CLSCTX_INPROC_SERVER)?;
    let category_mgr: ITfCategoryMgr = CoCreateInstance(
        &CLSID_TF_CategoryMgr, 
        None, 
        CLSCTX_INPROC_SERVER)?;

    // three things to register:
    // 1. the IME itself
    // 2. language profile
    // 3. categories(the features the IME has)

    let ime_id = &GUID::from(IME_ID);
    let lang_profile_id = &GUID::from(LANG_PROFILE_ID);

    input_processor_profiles.Register(ime_id)?;
    debug!("Registered the input method.");
    // todo the icon cannot be registered
    let ime_name: Vec<u16> = OsStr::new(IME_NAME).null_terminated_wchars();
    let icon_file: Vec<u16> = OsStr::new(ICON_FILE).null_terminated_wchars();
    input_processor_profiles.AddLanguageProfile(ime_id, LANG_ID, lang_profile_id, &ime_name, &icon_file, 0)?;
    debug!("Registered the language profile.");
    for rcatid  in SUPPORTED_CATEGORIES {
        category_mgr.RegisterCategory(ime_id, &rcatid, ime_id)?;
    }
    debug!("Registered the categories.");
    Ok(())
}

// similar process but re-doing everything
pub unsafe fn unregister_ime() -> Result<()> {
    // todo: it seems able to unregister the dll but alaways exits with 0x80004005
    let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
        &CLSID_TF_InputProcessorProfiles, // using ::IID would cause unregister to fail
        None, 
        CLSCTX_INPROC_SERVER)?;
    let category_mgr: ITfCategoryMgr = CoCreateInstance(
        &CLSID_TF_CategoryMgr, 
        None, 
        CLSCTX_INPROC_SERVER)?;


    let ime_id = &GUID::from(IME_ID);
    let lang_profile_id = &GUID::from(LANG_PROFILE_ID);

    input_processor_profiles.Unregister(ime_id)?;
    debug!("Unregistered the input method.");
    input_processor_profiles.RemoveLanguageProfile(ime_id, LANG_ID, lang_profile_id)?;
    debug!("Unregistered the language profile.");
    for rcatid in SUPPORTED_CATEGORIES {
        category_mgr.UnregisterCategory(ime_id, &rcatid, ime_id)?;
    }
    debug!("Unregistered the categories.");
    Ok(())
}


