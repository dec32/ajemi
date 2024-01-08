use log::{trace, error};
use windows::Win32::UI::TextServices::{ITfTextInputProcessor, ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfKeystrokeMgr_Impl, ITfTextInputProcessorEx_Impl, ITfTextInputProcessorEx};
use windows::core::{Result, ComInterface, Interface, implement};

use crate::ime::key_event_sink::KeyEventSink;

#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]
pub struct TextInputProcessor;

impl TextInputProcessor {
    fn new() -> ITfTextInputProcessor {
        TextInputProcessor{}.into()
    }
}

// thread manager is an essential component for many tasks.
// tid is the identifier for the client (the program where the user is typing into)
impl ITfTextInputProcessor_Impl for TextInputProcessor {
    #[allow(non_snake_case)]
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        // self.thread_mgr = thread_mgr.map(|it|it as *const ITfThreadMgr).unwrap_or(ptr::null());
        // self.client_id = Some(client_id);
        trace!("Activate");
        
        let Some(thread_mgr) = thread_mgr else {
            error!("Thread manager is null.");
            return Ok(());
        };
        
        // call a bunch of AddviceSink methods to subscribe all kinds of event
        unsafe {
            // todo how do you get the pointer of &self without moving it.
            // let sink = ITfKeyEventSink::from(self);
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(tid, &ITfKeyEventSink::from(KeyEventSink{}), true)?;
            let focus = thread_mgr.GetFocus()?;
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