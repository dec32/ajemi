use std::ffi::{OsStr, OsString};
use std::fs;
use log::{debug, warn, error};
use windows::Win32::Foundation::E_FAIL;
use windows::core::{Result, GUID};
use windows::Win32::UI::TextServices;
use windows::Win32::{System::{Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, LibraryLoader::GetModuleFileNameA}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr, CLSID_TF_CategoryMgr}};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use crate::extend::GUIDExt;
use crate::{global::*, extend::OsStrExt2};


//----------------------------------------------------------------------------
//
//  Registation for standard COM in-proc servers of any kind.
//  An IME is one of these servers.
//
//----------------------------------------------------------------------------


#[cfg(target_pointer_width = "64")]
const POSSIBLE_DLL_PATHS: [&str; 2] = [".\\target\\debug\\ajemi.dll", ".\\ajemi.dll"];
#[cfg(target_pointer_width = "32")]
const POSSIBLE_DLL_PATHS: [&str; 2] = [".\\target\\i686-pc-windows-msvc\\debug\\ajemi.dll", ".\\ajemi32.dll"];


// FIXME these unwrappings...
pub unsafe fn register_server() -> Result<()> {
    // Register the IME's ASCII name under HKLM\SOFTWARE\Classes\CLSID\{IME_ID}
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SOFTWARE\\Classes\\CLSID\\{{{}}}", IME_ID.to_rfc4122());
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
    // FIXME the buf is always empty
    let mut buf: Vec<u8> = Vec::with_capacity(260);
    debug!("Handle to the dll module is {:#0X}", dll_module().0);
    GetModuleFileNameA(dll_module(), &mut buf);
    debug!("Result of GetModuleFileNameA: {:?}", buf);
    if !buf.is_empty() {
        let path = OsString::from_encoded_bytes_unchecked(buf);
        debug!("Found dll in {:?}", path);
        return Ok(path);
    }
    // GetModuleFileNameA tends to fail so try a few more options
    warn!("GetModuleFileNameA did not provide the path of the DLL file.");
    for path in POSSIBLE_DLL_PATHS {
        if let Ok(canonical_path) = fs::canonicalize(path) {
            debug!("Found dll in {path}");
            return Ok(canonical_path.into_os_string())
        }     
    }
    error!("Failed to find the dll path.");
    return Err(E_FAIL.into());
}

pub unsafe fn unregister_server() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SOFTWARE\\Classes\\CLSID\\{{{}}}", IME_ID.to_rfc4122());
    hklm.delete_subkey_all(path).unwrap();
    Ok(())
}

//----------------------------------------------------------------------------
//
//  Registration for an IME.
//
//----------------------------------------------------------------------------


// features supported by the IME. there'are 18 of them in total. 
// register all of them expect the speech one and the handwriting one, or 
// your input method won't work in certain applications (for example, MS Word)
const SUPPORTED_CATEGORIES: [GUID; 16] = [
    TextServices::GUID_TFCAT_CATEGORY_OF_TIP,
    TextServices::GUID_TFCAT_TIP_KEYBOARD,
    // TextServices::GUID_TFCAT_TIP_SPEECH,
    // TextServices::GUID_TFCAT_TIP_HANDWRITING,
    TextServices:: GUID_TFCAT_TIPCAP_SECUREMODE,
    TextServices::GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
    TextServices::GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
    TextServices::GUID_TFCAT_TIPCAP_COMLESS,
    TextServices::GUID_TFCAT_TIPCAP_WOW16,
    TextServices::GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
    TextServices::GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
    TextServices::GUID_TFCAT_PROP_AUDIODATA,
    TextServices:: GUID_TFCAT_PROP_INKDATA,
    TextServices::GUID_TFCAT_PROPSTYLE_STATIC,
    GUID::from_u128(0x85F9794B_4D19_40D8_8864_4E747371A66D), // TextServices::GUID_TFCAT_PROPSTYLE_STATICCOMPSCT,
    GUID::from_u128(0x24AF3031_852D_40A2_BC09_8992898CE722), // TextServices::GUID_TFCAT_PROSTYLE_CUSTOM
    TextServices::GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER,
    TextServices::GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY
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

    input_processor_profiles.Register(&IME_ID)?;
    debug!("Registered the input method.");
    let ime_name: Vec<u16> = OsStr::new(IME_NAME).null_terminated_wchars();
    let icon_file: Vec<u16> = find_dll_path()?.null_terminated_wchars();
    input_processor_profiles.AddLanguageProfile(
        &IME_ID, LANG_ID, &LANG_PROFILE_ID, &ime_name, 
        &icon_file, 0)?;
    debug!("Registered the language profile.");
    for rcatid  in SUPPORTED_CATEGORIES {
        category_mgr.RegisterCategory(&IME_ID, &rcatid, &IME_ID)?;
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

    input_processor_profiles.Unregister(&IME_ID)?;
    debug!("Unregistered the input method.");
    input_processor_profiles.RemoveLanguageProfile(&IME_ID, LANG_ID, &LANG_PROFILE_ID)?;
    debug!("Unregistered the language profile.");
    for rcatid in SUPPORTED_CATEGORIES {
        category_mgr.UnregisterCategory(&IME_ID, &rcatid, &IME_ID)?;
    }
    debug!("Unregistered the categories.");
    Ok(())
}


