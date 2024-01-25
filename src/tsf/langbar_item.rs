use log::trace;
use windows::Win32::Foundation::{POINT, RECT, BOOL};
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::core::{Result, BSTR};
use windows::Win32::UI::TextServices::{ITfLangBarItemButton_Impl, ITfMenu, TfLBIClick, ITfLangBarItem_Impl, TF_LANGBARITEMINFO, TF_LBI_STYLE_BTN_BUTTON};
use crate::{IME_ID, LANGBAR_ITEM_ID};

use super::TextService;


#[allow(non_snake_case, unused)]
impl ITfLangBarItem_Impl for TextService {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> Result<()> {
        unsafe {
            (*pinfo).clsidService = IME_ID;
            (*pinfo).guidItem = LANGBAR_ITEM_ID;
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
impl ITfLangBarItemButton_Impl for TextService {
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
        Ok(self.write()?.icon)
    }
    fn GetText(&self) -> Result<BSTR> {
        Ok(BSTR::default())
    }
}