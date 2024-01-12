use log::{trace, warn, debug};
use windows::Win32::UI::TextServices::{ITfTextInputProcessor, ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfKeystrokeMgr_Impl, ITfTextInputProcessorEx_Impl, ITfTextInputProcessorEx, ITfThreadMgrEventSink, ITfSource};
use windows::core::{Result, ComInterface, implement};

use crate::ime::key_event_sink::KeyEventSink;
use crate::ime::thread_mgr_event_sink::ThreadMgrEventSink;


#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]

/*
TextInputProcessor(Ex) {
    KeyEventSink
    CompositionSink
    ThreadMgrEventSink
    TextEditSink
}
*/
pub struct TextInputProcessor {

}
impl TextInputProcessor {
    pub fn new() -> TextInputProcessor {
        TextInputProcessor{

        }
    }
}

impl ITfTextInputProcessor_Impl for TextInputProcessor {
    #[allow(non_snake_case)]
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        trace!("Activate");
        // tid is the identifier for the client (the program where the user is typing into)

        let Some(thread_mgr) = thread_mgr else {
            warn!("Thread manager is null.");
            return Ok(());
        };

        // Creating event sinks to subsucribe to events

        // TODO: Manage the dependencies of event sinks.
        // For now its only
        // KeyEventSink ---> CompositionEventSink

        let key_event_sink = KeyEventSink::new(tid);
        let thread_mgr_event_sink = ThreadMgrEventSink::new();
        
        unsafe{
            thread_mgr.cast::<ITfSource>()?.AdviseSink(
                &ITfThreadMgrEventSink::IID, &ITfThreadMgrEventSink::from(thread_mgr_event_sink))?;
            debug!("Added thread manager event sink.");    
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(
                tid, &ITfKeyEventSink::from(key_event_sink) , true)?;
            debug!("Added key event sink.");
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
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, _dwflags: u32) -> Result<()> {
        self.Activate(ptim, tid)
    }
}