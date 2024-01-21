use log::{trace, debug};
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::{ ITfThreadMgr, ITfTextInputProcessor_Impl, ITfKeystrokeMgr, ITfKeyEventSink, ITfTextInputProcessorEx_Impl, ITfLangBarItemMgr, ITfLangBarItem};
use windows::core::{Result, ComInterface};
use super::{TextService, candidate_list::CandidateList};

#[allow(non_snake_case)]
impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        trace!("Activate({tid})");
        let mut inner = self.inner()?;
        let thread_mgr = thread_mgr.ok_or(E_FAIL)?;
        inner.tid = tid;
        inner.thread_mgr = Some(thread_mgr.clone());
        inner.candidate_list = {
            let parent_window = unsafe {thread_mgr.GetFocus()?.GetTop()?.GetActiveView()?.GetWnd()}?;
            Some(CandidateList::create(parent_window)?)
        };
        // Creating event sinks to subsucribe to events
        unsafe {
            thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(
                tid, &inner.interface::<ITfKeyEventSink>()? , true)?;
            debug!("Added key event sink.");
            // thread_mgr.cast::<ITfLangBarItemMgr>()?.AddItem(
            //     &inner.interface::<ITfLangBarItem>()?)?;
            // debug!("Added langbar item.");
        }
        Ok(())
    }

    fn Deactivate(&self) -> Result<()> {
        trace!("Deactivate");
        let mut inner = self.inner()?;
        let thread_mgr = inner.thread_mgr.as_ref().ok_or(E_FAIL)?;
        unsafe {
            thread_mgr.cast::<ITfKeystrokeMgr>()?.UnadviseKeyEventSink(inner.tid)?;
            debug!("Removed key event sink.");
            // thread_mgr.cast::<ITfLangBarItemMgr>()?.RemoveItem(&inner.interface::<ITfLangBarItem>()?)?;
        }
        inner.thread_mgr = None;
        Ok(())
    }
}

#[allow(non_snake_case)]
impl ITfTextInputProcessorEx_Impl for TextService {  
    fn ActivateEx(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32, _dwflags: u32) -> Result<()> {
        self.Activate(thread_mgr, tid)
    }
}