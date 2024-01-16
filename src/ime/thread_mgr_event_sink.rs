use windows::core::implement;
use windows::Win32::UI::TextServices::{ITfThreadMgrEventSink, ITfThreadMgrEventSink_Impl, ITfDocumentMgr, ITfContext};
use windows::core::Result;

#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink;

impl ThreadMgrEventSink {
    pub fn new() -> ThreadMgrEventSink {ThreadMgrEventSink{}}
}

impl ITfThreadMgrEventSink_Impl for ThreadMgrEventSink {
    #[allow(non_snake_case, unused)]
    fn OnInitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }

    #[allow(non_snake_case, unused)]
    fn OnUninitDocumentMgr(&self, pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        Ok(())
    }

    #[allow(non_snake_case, unused)]
    fn OnSetFocus(&self, pdimfocus: Option<&ITfDocumentMgr>, pdimprevfocus: Option<&ITfDocumentMgr>) ->Result<()> {
        Ok(())
    }

    #[allow(non_snake_case, unused)]
    fn OnPushContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
    
    #[allow(non_snake_case, unused)]
    fn OnPopContext(&self, pic: Option<&ITfContext>) -> Result<()> {
        Ok(())
    }
}

