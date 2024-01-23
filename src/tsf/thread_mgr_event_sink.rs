use windows::Win32::UI::TextServices::{ITfThreadMgrEventSink_Impl, ITfDocumentMgr, ITfContext};
use windows::core::Result;

use super::TextService;

#[allow(non_snake_case, unused)]
impl ITfThreadMgrEventSink_Impl for TextService {
    fn OnInitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }
    fn OnUninitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }
    fn OnSetFocus(&self, focus: Option<&ITfDocumentMgr>, prevfocus: Option<&ITfDocumentMgr>) ->Result<()> {
        self.write()?.abort()
    }
    fn OnPushContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
    fn OnPopContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}