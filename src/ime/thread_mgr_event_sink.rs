use windows::core::implement;
use windows::Win32::UI::TextServices::{ITfThreadMgrEventSink, ITfThreadMgrEventSink_Impl, ITfDocumentMgr, ITfContext};
use windows::core::Result;

#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink;

impl ThreadMgrEventSink {
    pub fn new() -> ThreadMgrEventSink {ThreadMgrEventSink{}}
}

#[allow(non_snake_case, unused)]
impl ITfThreadMgrEventSink_Impl for ThreadMgrEventSink {
    fn OnInitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }
    fn OnUninitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }
    fn OnSetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>, pdimprevfocus: Option<&ITfDocumentMgr>) ->Result<()> {
        Ok(())
    }
    fn OnPushContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
    fn OnPopContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}

