use std::cell::Cell;

use log::{trace, warn, debug};
use windows::Win32::UI::TextServices::{ITfTextInputProcessor, ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfTextInputProcessorEx_Impl, ITfTextInputProcessorEx, ITfThreadMgrEventSink, ITfSource};
use windows::core::{Result, ComInterface, implement};

use crate::ime::key_event_sink::KeyEventSink;
use crate::ime::thread_mgr_event_sink::ThreadMgrEventSink;


#[implement(ITfTextInputProcessor, ITfTextInputProcessorEx)]
pub struct TextInputProcessor {
    ctx: Cell<Option<Context>>,
}
struct Context {
    tid: u32,
    thread_mgr: ITfThreadMgr,
    cookie: u32,
}

impl TextInputProcessor {
    pub fn new() -> TextInputProcessor {
        TextInputProcessor{
            ctx: Cell::new(None)
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

        let key_event_sink = KeyEventSink::new(tid);
        let thread_mgr_event_sink = ThreadMgrEventSink::new();
        let cookie ;
        unsafe{
            cookie = thread_mgr.cast::<ITfSource>()?.AdviseSink(
                &ITfThreadMgrEventSink::IID, &ITfThreadMgrEventSink::from(thread_mgr_event_sink))?;
            debug!("Added thread manager event sink.");    
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(
                tid, &ITfKeyEventSink::from(key_event_sink) , true)?;
            debug!("Added key event sink.");
        }
        // wow i hate you microsoft why every &self is immutable
        self.ctx.set(Some(Context{
            thread_mgr: thread_mgr.clone(),
            tid,
            cookie
        }));
        Ok(())
    }

    #[allow(non_snake_case)]
    fn Deactivate(&self) -> Result<()> {
        trace!("Deactivate");
        let Some(ctx) = self.ctx.take() else {
            return Ok(());
        };
        unsafe{
            ctx.thread_mgr.cast::<ITfSource>()?.UnadviseSink(ctx.cookie)?;
            debug!("Removed thread manager event sink.");    
            ctx.thread_mgr.cast::<ITfKeystrokeMgr>()?.UnadviseKeyEventSink(ctx.tid)?;
            debug!("Removed key event sink.");
        }
        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for TextInputProcessor {
    #[allow(non_snake_case)]
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, _dwflags: u32) -> Result<()> {
        self.Activate(ptim, tid)
    }
}