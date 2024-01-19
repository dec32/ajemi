use windows::Win32::Foundation::{POINT, RECT, BOOL};
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::core::{implement,Result, BSTR};
use windows::Win32::UI::TextServices::{ITfLangBarItem, ITfLangBarItemButton, ITfLangBarItemButton_Impl, ITfMenu, TfLBIClick, ITfLangBarItem_Impl, TF_LANGBARITEMINFO};

#[implement(ITfLangBarItem, ITfLangBarItemButton)]
struct LangbarButton {}
#[allow(non_snake_case, unused)]
impl ITfLangBarItem_Impl for LangbarButton {
    fn GetInfo(&self,pinfo: *mut TF_LANGBARITEMINFO) -> Result<()> {
        todo!()
    }
    fn GetStatus(&self) -> Result<u32> {
        todo!()
    }
    fn Show(&self,fshow:BOOL) -> Result<()> {
        todo!()
    }
    fn GetTooltipString(&self) -> Result<BSTR> {
        todo!()
    }
}
#[allow(non_snake_case, unused)]
impl ITfLangBarItemButton_Impl for LangbarButton {
    fn OnClick(&self,click:TfLBIClick,pt: &POINT, prcarea: *const RECT) -> Result<()> {
        todo!()
    }
    fn InitMenu(&self,pmenu: ::core::option::Option<&ITfMenu>) -> Result<()> {
        todo!()
    }
    fn OnMenuSelect(&self,wid:u32) -> Result<()> {
        todo!()
    }
    fn GetIcon(&self) -> Result<HICON> {
        todo!()
    }
    fn GetText(&self) -> Result<BSTR> {
        todo!()
    }
}