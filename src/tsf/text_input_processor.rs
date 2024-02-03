use log::{trace, debug, warn};
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::TextServices::{ CLSID_TF_CategoryMgr, ITfCategoryMgr, ITfKeyEventSink, ITfKeystrokeMgr, ITfSource, ITfTextInputProcessorEx_Impl, ITfTextInputProcessor_Impl, ITfThreadMgr, ITfThreadMgrEventSink};
use windows::core::{Result, ComInterface};
use crate::extend::VARANTExt;
use crate::{conf, DISPLAY_ATTR_ID};

use super::TextService;

#[allow(non_snake_case)]
impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        trace!("Activate({tid})");
        conf::reload();
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
            debug!("Added thread manager event sink.");
            let _ = inner.create_candidate_list();
            // thread_mgr.cast::<ITfLangBarItemMgr>()?.AddItem(
            //     &inner.interface::<ITfLangBarItem>()?)?;
            // debug!("Added langbar item.");
            if inner.display_attribute.is_none() {
                let category_mgr: ITfCategoryMgr = CoCreateInstance(
                    &CLSID_TF_CategoryMgr, None, CLSCTX_INPROC_SERVER)?;
                let guid_atom = category_mgr.RegisterGUID(&DISPLAY_ATTR_ID)?;
                inner.display_attribute = Some(VARIANT::i4(guid_atom as i32));
            }
            Ok(())
        }
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
            if let Some(candidate_list) = inner.candidate_list.as_ref() {
                candidate_list.destroy()?;
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