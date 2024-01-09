use log::{trace, error, warn};
use windows::Win32::UI::TextServices::{ITfTextInputProcessor, ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfKeystrokeMgr_Impl, ITfTextInputProcessorEx_Impl, ITfTextInputProcessorEx, ITfThreadMgrEventSink, ITfSource};
use windows::core::{Result, ComInterface, implement};

use crate::ime::key_event_sink::{KeyEventSink, self};
use crate::ime::thread_mgr_event_sink::{self, ThreadMgrEventSink};

#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]

/*
TextInputProcessor(Ex) {
    KeyEventSink
    CompositionSink
    ThreadMgrEventSink
    TextEditSink

}
*/
pub struct TextInputProcessor;
impl TextInputProcessor {
    pub fn new() -> TextInputProcessor {
        TextInputProcessor{}
    }
}

impl ITfTextInputProcessor_Impl for TextInputProcessor {
    #[allow(non_snake_case)]
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        trace!("Activate");
        // thread manager is an essential component for many tasks.
        // tid is the identifier for the client (the program where the user is typing into)

        let Some(thread_mgr) = thread_mgr else {
            warn!("Thread manager is null.");
            return Ok(());
        };

        // call a bunch of AddviceSink methods to subscribe all kinds of events
        unsafe {
            let thread_mgr_event_sink = ITfThreadMgrEventSink::from(ThreadMgrEventSink::new());
            let key_event_sink = ITfKeyEventSink::from(KeyEventSink::new());

            // thread_mgr.cast::<ITfSource>()?.AdviseSink(&ITfThreadMgrEventSink::IID, &thread_mgr_event_sink)?;
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(tid, &key_event_sink, true)?;
        }
        Ok(())
    }

    #[allow(non_snake_case)]
    fn Deactivate(&self) -> Result<()> {
        trace!("Deactivate");
        // self.thread_mgr = ptr::null();
        // self.client_id = None;
        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for TextInputProcessor {
    #[allow(non_snake_case)]
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> Result<()> {
        self.Activate(ptim, tid)
    }
}