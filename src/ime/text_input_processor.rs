use log::{trace, error};
use windows::Win32::UI::TextServices::{ITfTextInputProcessor, ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfKeystrokeMgr_Impl, ITfTextInputProcessorEx_Impl, ITfTextInputProcessorEx};
use windows::core::{Result, ComInterface, implement};

use crate::ime::key_event_sink::{KeyEventSink, self};

#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]
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
            error!("Thread manager is null.");
            return Ok(());
        };
        let key_event_sink = KeyEventSink::new();
        
        // call a bunch of AddviceSink methods to subscribe all kinds of event
        unsafe {
            // todo how do you get the pointer of &self without moving it.
            // let sink = ITfKeyEventSink::from(self);
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(tid, &ITfKeyEventSink::from(key_event_sink), true)?;
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