use std::sync::atomic::AtomicBool;
use windows::Win32::Foundation::{E_INVALIDARG, E_NOTIMPL};
use windows::Win32::UI::TextServices::{IEnumTfDisplayAttributeInfo, IEnumTfDisplayAttributeInfo_Impl, ITfDisplayAttributeInfo, ITfDisplayAttributeInfo_Impl, ITfDisplayAttributeProvider_Impl, TF_ATTR_INPUT, TF_DA_COLOR, TF_DISPLAYATTRIBUTE, TF_LS_SOLID};
use windows::core::{implement, Result, BSTR, GUID};
use std::sync::atomic::Ordering::*;
use crate::{global, DISPLAY_ATTR_ID};
use super::TextService;

//---------------------------------------------------------------------------------
//
//  To provide display atrributes you need to implement ITfDisplayAttributeProvider
//  and "make it available from the class factory" according to MSDN. They lied.
//  Windows will nerver acquire the instance from class factory. It will only 
//  implicitly cast the ITfTextInputProcessor instance to ITfDisplayAttributeProvider.
//
//---------------------------------------------------------------------------------

#[allow(non_snake_case)]
impl ITfDisplayAttributeProvider_Impl for TextService {
    fn EnumDisplayAttributeInfo(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        Ok(EnumDisplayAttributeInfo::create())
    }
    fn GetDisplayAttributeInfo(&self, guid: *const GUID) -> Result<ITfDisplayAttributeInfo> {
        if unsafe { *guid == global::DISPLAY_ATTR_ID } {
            Ok(DisplayAttributeInfo::create())
        } else {
            Err(E_INVALIDARG.into())
        }
    }
}


//----------------------------------------------------------------------------
//
//  An enumerator that enumerates through all possible display atrributes.
//  The input method has only one display attribute so this is kinda dumb.
//
//----------------------------------------------------------------------------

#[implement(IEnumTfDisplayAttributeInfo)]
struct EnumDisplayAttributeInfo { enumerated: AtomicBool }
impl EnumDisplayAttributeInfo {
    fn create() -> IEnumTfDisplayAttributeInfo {
        IEnumTfDisplayAttributeInfo::from(Self{ enumerated: AtomicBool::new(false) })
    }
}

#[allow(non_snake_case)]
impl IEnumTfDisplayAttributeInfo_Impl for EnumDisplayAttributeInfo {
    fn Clone(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        Err(E_NOTIMPL.into())
    }

    fn Next(&self, _count:u32, info: *mut Option<ITfDisplayAttributeInfo>, fetched: *mut u32) -> Result<()> {
        // Dear MS please fix these raw pointers thanks
        unsafe {
            if self.enumerated.fetch_and(true, Relaxed) {
                *info = Some(DisplayAttributeInfo::create());
                *fetched = 1;
            } else {
                *fetched = 0;
            }
        }
        Ok(())
    }

    fn Reset(&self) -> Result<()> {
        self.enumerated.fetch_and(false, Relaxed);
        Ok(())
    }

    fn Skip(&self, count:u32) ->  Result<()> {
        if count > 0 {
            self.enumerated.fetch_and(true, Relaxed);
        }
        Ok(())
    }
}

//----------------------------------------------------------------------------
//
//  Our one and only display attribute that does nothing but adding underlines
//
//----------------------------------------------------------------------------

#[implement(ITfDisplayAttributeInfo)]
#[derive(Default)]
pub struct DisplayAttributeInfo;
impl DisplayAttributeInfo {
    pub fn create() -> ITfDisplayAttributeInfo {
        ITfDisplayAttributeInfo::from(Self{})
    }
}

#[allow(non_snake_case)]
impl ITfDisplayAttributeInfo_Impl for DisplayAttributeInfo {
    fn GetGUID(&self) -> Result<GUID> {
        Ok(DISPLAY_ATTR_ID)
    }

    fn GetDescription(&self) -> Result<BSTR> {
        Err(E_INVALIDARG.into())
    }

    fn GetAttributeInfo(&self, attr: *mut TF_DISPLAYATTRIBUTE) -> Result<()> {
        unsafe{ 
            *attr = TF_DISPLAYATTRIBUTE {
                crText: TF_DA_COLOR::default(),
                crBk: TF_DA_COLOR::default(),
                crLine: TF_DA_COLOR::default(),
                lsStyle: TF_LS_SOLID,
                fBoldLine: false.into(),
                bAttr: TF_ATTR_INPUT,
            };
        }
        Ok(())
    }

    fn SetAttributeInfo(&self, _attr: *const TF_DISPLAYATTRIBUTE) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn Reset(&self) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}