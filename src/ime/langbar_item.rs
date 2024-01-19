use log::trace;
use windows::Win32::Foundation::{POINT, RECT, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{HICON, LoadIconA};
use windows::core::{implement,Result, BSTR, s, GUID};
use windows::Win32::UI::TextServices::{ITfLangBarItem, ITfLangBarItemButton, ITfLangBarItemButton_Impl, ITfMenu, TfLBIClick, ITfLangBarItem_Impl, TF_LANGBARITEMINFO, TF_LBI_STATUS_BTN_TOGGLED, ITfContext, TF_LBI_STYLE_BTN_BUTTON, TF_LBI_STYLE_BTN_MENU};

use crate::{DLL_MOUDLE, IME_ID, LANGBAR_ITEM_ID};
#[implement(ITfLangBarItem, ITfLangBarItemButton)]
pub struct LangbarItem {
    icon: HICON
}

impl LangbarItem {
    // fixme there's no need to new at all
    pub fn new() -> LangbarItem {
        let hinstance = unsafe{ DLL_MOUDLE.unwrap() };
        let icon = unsafe {LoadIconA(hinstance, s!("ICON"))}.unwrap();
        LangbarItem {icon}
    }
}

#[allow(non_snake_case, unused)]
impl ITfLangBarItem_Impl for LangbarItem {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> Result<()> {
        unsafe {
            (*pinfo).clsidService = GUID::from(IME_ID);
            (*pinfo).guidItem = GUID::from(LANGBAR_ITEM_ID);
            (*pinfo).dwStyle = TF_LBI_STYLE_BTN_BUTTON;
            (*pinfo).ulSort = 0;
        }
        Ok(())
    }
    fn GetStatus(&self) -> Result<u32> {
        Ok(0)
    }
    fn Show(&self,fshow:BOOL) -> Result<()> {
        Ok(())
    }
    fn GetTooltipString(&self) -> Result<BSTR> {
        Ok(BSTR::default())
    }
}
#[allow(non_snake_case, unused)]
impl ITfLangBarItemButton_Impl for LangbarItem {
    fn OnClick(&self, click:TfLBIClick, pt: &POINT, prcarea: *const RECT) -> Result<()> {
        trace!("OnClick");
        Ok(())
    }
    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> Result<()> {
        let Some(menu) = pmenu else {
            return Ok(());
        };
        // todo add menu item
        Ok(())
    }
    fn OnMenuSelect(&self, wid:u32) -> Result<()> {
        Ok(())
    }
    fn GetIcon(&self) -> Result<HICON> {
        Ok(self.icon)
    }
    fn GetText(&self) -> Result<BSTR> {
        Ok(BSTR::default())
    }
}