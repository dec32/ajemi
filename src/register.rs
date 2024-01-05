use windows::{Win32::{System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, UI::TextServices::{ITfInputProcessorProfiles, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr, CLSID_TF_CategoryMgr, GUID_TFCAT_CATEGORY_OF_TIP, GUID_TFCAT_TIP_KEYBOARD, GUID_TFCAT_TIPCAP_SECUREMODE, GUID_TFCAT_TIPCAP_UIELEMENTENABLED, GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT, GUID_TFCAT_TIPCAP_COMLESS, GUID_TFCAT_TIPCAP_WOW16, GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT, GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, GUID_TFCAT_PROP_AUDIODATA, GUID_TFCAT_PROP_INKDATA, GUID_TFCAT_PROPSTYLE_STATIC, GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER, GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY}}, core::{Result, GUID}};
use crate::consts::{*};


//----------------------------------------------------------------------------
//
//  Implementation for DllRegisterServer() and DllUnregisterServer()
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

pub unsafe fn register() -> Result<()>{
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

    // register
    input_processor_profiles.Register(ime_id())?;
    // todo the icon cannot be registered
    input_processor_profiles.AddLanguageProfile(ime_id(), lang_id(), lang_profile_id(), ime_name(), icon_file(), 0)?;
    for rcatid  in SUPPORTED_CATEGORIES {
        let rcatid = &rcatid as *const GUID;
        category_mgr.RegisterCategory(ime_id(), rcatid, ime_id())?;
    }
    Ok(())
}


// similar process but re-doing everything
pub unsafe fn unregister() -> Result<()> {
    // todo: it seems able to unregister the dll but alaways exits with 0x80004005
    let input_processor_profiles: ITfInputProcessorProfiles = CoCreateInstance(
        &CLSID_TF_InputProcessorProfiles as *const GUID, 
        None, 
        CLSCTX_INPROC_SERVER)?;
    let category_mgr: ITfCategoryMgr = CoCreateInstance(
        &CLSID_TF_CategoryMgr as *const GUID, 
        None, 
        CLSCTX_INPROC_SERVER)?;

    
    input_processor_profiles.Unregister(ime_id())?;
    input_processor_profiles.RemoveLanguageProfile(ime_id(), lang_id(), lang_profile_id())?;
    for rcatid in SUPPORTED_CATEGORIES {
        let rcatid = &rcatid as *const GUID;
        category_mgr.UnregisterCategory(ime_id(), rcatid, ime_id())?;
    }
    Ok(())
}

