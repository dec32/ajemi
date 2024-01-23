use log::{trace, debug, warn};
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::{ ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfTextInputProcessorEx_Impl, ITfSource, ITfThreadMgrEventSink};
use windows::core::{Result, ComInterface};
use super::{TextService, candidate_list::CandidateList};

#[allow(non_snake_case)]
impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        trace!("Activate({tid})");
        let mut inner = self.write()?;
        let thread_mgr = thread_mgr.ok_or(E_FAIL)?;
        inner.tid = tid;
        inner.thread_mgr = Some(thread_mgr.clone());
        unsafe {
            // Use self as event sink to subscribe to events
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(
                tid, &inner.interface::<ITfKeyEventSink>()? , true)?;
            debug!("Added key event sink.");

            inner.cookie = Some(thread_mgr.cast::<ITfSource>()?.AdviseSink(
                &ITfThreadMgrEventSink::IID, &inner.interface::<ITfThreadMgrEventSink>()?)?);
            debug!("Added thread manager event sink.")
            // thread_mgr.cast::<ITfLangBarItemMgr>()?.AddItem(
            //     &inner.interface::<ITfLangBarItem>()?)?;
            // debug!("Added langbar item.");
        }
        // If creation fails, try later.
        inner.candidate_list = CandidateList::create(thread_mgr).ok();
        Ok(())
    }

    fn Deactivate(&self) -> Result<()> {
        trace!("Deactivate");
        let mut inner = self.write()?;
        let thread_mgr = inner.thread_mgr()?;
        unsafe {
            thread_mgr.cast::<ITfKeystrokeMgr>()?.UnadviseKeyEventSink(inner.tid)?;
            debug!("Removed key event sink.");
            
            if let Some(cookie) = inner.cookie {
                thread_mgr.cast::<ITfSource>()?.UnadviseSink(cookie)?;
                inner.cookie = None;
                debug!("Removed thread manager event sink.");
            } else {
                warn!("Cookie for thread manager event sink is None.");
            }
            // thread_mgr.cast::<ITfLangBarItemMgr>()?.RemoveItem(&inner.interface::<ITfLangBarItem>()?)?;
            // debug!("Removed langbar item.")
        }
        inner.thread_mgr = None;
        inner.candidate_list = None;
        Ok(())
    }
}

#[allow(non_snake_case)]
impl ITfTextInputProcessorEx_Impl for TextService {  
    fn ActivateEx(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32, _dwflags: u32) -> Result<()> {
        self.Activate(thread_mgr, tid)
    }
}

//----------------------------------------------------------------------------
//
//  Now see tsf/key_event_sink.rs
//
//----------------------------------------------------------------------------