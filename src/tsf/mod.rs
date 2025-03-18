pub mod text_input_processor;
pub mod display_attribute_provider;
mod key_event_sink;
mod thread_mgr_event_sink;
mod composition;
mod edit_session;
mod langbar_item;

use std::time::{Duration, Instant};
use log_derive::logfn;
use parking_lot::{RwLock, RwLockWriteGuard};
use log::{debug, error, warn};

use windows::{core::{implement, AsImpl, Interface, Result, VARIANT}, Win32::{Foundation::E_FAIL, UI::{TextServices::{ITfComposition, ITfCompositionSink, ITfContext, ITfDisplayAttributeProvider, ITfKeyEventSink, ITfLangBarItem, ITfTextInputProcessor, ITfTextInputProcessorEx, ITfThreadMgr, ITfThreadMgrEventSink, HKL}, WindowsAndMessaging::HICON}}};
use crate::{engine::{Engine, Suggestion}, global::registered_hkl, ui::candidate_list::CandidateList};

//----------------------------------------------------------------------------
//
//  A text service is required to implement ITfTextInputProcessor and provide
//  a few other interfaces in ITfTextInputProcessor::Activate. The common
//  approach is to let the text service implement every interfaces needed and
//  return self whenever required.
//
//----------------------------------------------------------------------------

#[implement(
    ITfTextInputProcessor,
    ITfTextInputProcessorEx,
    ITfThreadMgrEventSink,
    ITfKeyEventSink,
    ITfCompositionSink,
    ITfLangBarItem,
    ITfDisplayAttributeProvider
)]

/// Methods of TSF interfaces don't allow mutation of any kind. Thus all mutable 
/// states are hidden behind a lock. The lock is supposed to be light-weight since
/// inputs from users can be frequent. 
pub struct TextService {
    inner: RwLock<TextServiceInner>,
}
struct TextServiceInner {
    // engine
    engine: Engine,
    // Some basic info about the clinet (the program where user is typing)
    tid: u32,
    thread_mgr: Option<ITfThreadMgr>,
    context: Option<ITfContext>,
    // ThreadMrgEventSink
    cookie: Option<u32>,
    // KeyEventSink
    hkl: HKL,
    char_buf: String,
    // Composition
    composition: Option<ITfComposition>,
    spelling: String,
    selected: String,
    suggestions: Vec<Suggestion>,
    preedit: String,
    // display attribute provider
    display_attribute: Option<VARIANT>,
    // UI
    candidate_list: Option<CandidateList>,
    icon: HICON,
    // An Arc-like smart pointer pointing to TextService
    interface: Option<ITfTextInputProcessor>,
}

impl TextService {
    #[logfn(err = "Error")]
    pub fn create() -> Result<ITfTextInputProcessor> {
        let inner = TextServiceInner {
            engine: Engine::build_or_default(),
            tid: 0,
            thread_mgr: None,
            context: None,
            hkl: registered_hkl()?,
            char_buf: String::with_capacity(4),
            cookie: None,
            composition: None,
            spelling: String::with_capacity(32),
            suggestions: Vec::new(),
            selected: String::with_capacity(32),
            preedit: String::with_capacity(32),
            icon: HICON::default(),
            candidate_list: None,
            display_attribute: None,
            interface: None,
        };
        let text_service = TextService {
            inner: RwLock::new(inner)
        };
        // from takes ownership of the object and returns a smart pointer
        let interface = ITfTextInputProcessor::from(text_service);
        // inject the smart pointer back to the object
        let text_service: &TextService = unsafe {interface.as_impl()};
        text_service.write()?.interface = Some(interface.clone());
        // cast the interface to desired type
        interface.cast()
    }

    fn write(&self) -> Result<RwLockWriteGuard<TextServiceInner>> {
        self.inner.try_write().or_else(||{
            warn!("RwLock::try_write returned None.");
            let timeout = Instant::now() + Duration::from_millis(50);
            self.inner.try_write_until(timeout)
        }).ok_or_else(||{
            error!("Failed to obtain write lock.");
            E_FAIL.into()
        })
    }

    fn try_write(&self) -> Result<RwLockWriteGuard<TextServiceInner>> {
        self.inner.try_write().ok_or_else(||E_FAIL.into())
    }
}

impl TextServiceInner {
    fn interface<I: Interface>(&self) -> Result<I> {
        // guarenteed to be Some by TextService::create
        self.interface.as_ref().unwrap().cast()
    }

    fn thread_mgr(&self) -> Result<&ITfThreadMgr> {
        self.thread_mgr.as_ref().ok_or_else(||{
            error!("Thread manager is None.");
            E_FAIL.into()
        })
    }

    fn context(&self) -> Result<&ITfContext> {
        self.context.as_ref().ok_or_else(||{
            error!("Context is None.");
            E_FAIL.into()
        })
    }

    fn candidate_list(&self) -> Result<&CandidateList> {
        self.candidate_list.as_ref().ok_or(E_FAIL.into())
    }

    fn create_candidate_list(&mut self) -> Result<()> {
        let parent_window = unsafe{ 
            self.thread_mgr()?.GetFocus()?.GetTop()?.GetActiveView()?.GetWnd()? 
        };
        self.candidate_list = Some(CandidateList::create(parent_window)?);
        Ok(())
    }

    fn assure_candidate_list(&mut self) -> Result<()>{
        if self.candidate_list.is_some() {
            return Ok(());
        } else {
            debug!("Previous creation of candidate list failed. Recreating now.");
            self.create_candidate_list()
        }
    }

}

//----------------------------------------------------------------------------
//
//  Now see tsf/text_input_processor.rs for the implementation.
//
//----------------------------------------------------------------------------