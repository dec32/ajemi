use windows::core::implement;
use windows::Win32::UI::TextServices::{ITfThreadMgrEventSink, ITfThreadMgrEventSink_Impl, ITfDocumentMgr, ITfContext};
use windows::core::Result;

#[implement(ITfThreadMgrEventSink)]
pub struct ThreadMgrEventSink;

impl ThreadMgrEventSink {
    pub fn new() -> ThreadMgrEventSink {ThreadMgrEventSink{}}
}

impl ITfThreadMgrEventSink_Impl for ThreadMgrEventSink {
    fn OnInitDocumentMgr(&self,pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        todo!()
    }

    fn OnUninitDocumentMgr(&self,pdim: Option<&ITfDocumentMgr>) -> Result<()> {
        todo!()
    }

    fn OnSetFocus(&self,pdimfocus: Option<&ITfDocumentMgr>,pdimprevfocus: Option<&ITfDocumentMgr>) ->Result<()> {
        todo!()
    }

    fn OnPushContext(&self,pic: Option<&ITfContext>) -> Result<()> {
        todo!()
    }

    fn OnPopContext(&self,pic: Option<&ITfContext>) -> Result<()> {
        todo!()
    }
}

