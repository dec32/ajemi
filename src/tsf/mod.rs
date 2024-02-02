pub mod text_input_processor;
pub mod display_attribute_provider;
mod key_event_sink;
mod thread_mgr_event_sink;
mod composition;
mod edit_session;
mod langbar_item;


use std::time::{Instant, Duration};
use parking_lot::{RwLock, RwLockWriteGuard};
use log::{error, warn};

use windows::{core::{Result, implement, AsImpl, ComInterface}, Win32::{UI::{TextServices::{ITfTextInputProcessor, ITfTextInputProcessorEx, ITfComposition, ITfThreadMgr, ITfKeyEventSink, ITfThreadMgrEventSink, ITfCompositionSink, ITfLangBarItem, ITfContext, ITfDisplayAttributeProvider}, WindowsAndMessaging::HICON}, Foundation::E_FAIL}};
use crate::{engine::Suggestion, ui::candidate_list::CandidateList};

use self::key_event_sink::Modifiers;

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
    ITfKeyEventSink,
    ITfThreadMgrEventSink,
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
    // Some basic info about the clinet (the program where user is typing)
    tid: u32,
    thread_mgr: Option<ITfThreadMgr>,
    context: Option<ITfContext>,
    // ThreadMrgEventSink
    cookie: Option<u32>,
    // KeyEventSink
    modifiers: Modifiers, // ctrl, shift, alt
    char_buf: String,
    // Composition
    composition: Option<ITfComposition>,
    spelling: String,
    selected: String,
    suggestions: Vec<Suggestion>,
    preedit: String,
    // UI
    candidate_list: Option<CandidateList>,
    icon: HICON,
    // An Arc-like smart pointer pointing to TextService
    interface: Option<ITfTextInputProcessor>,
}

impl TextService {
    pub fn create<I: ComInterface>() -> Result<I>{
        let inner = TextServiceInner {
            tid: 0,
            thread_mgr: None,
            context: None,
            modifiers: Modifiers::new(),
            char_buf: String::with_capacity(4),
            cookie: None,
            composition: None,
            spelling: String::with_capacity(32),
            suggestions: Vec::new(),
            selected: String::with_capacity(32),
            preedit: String::with_capacity(32),
            icon: HICON::default(),
            candidate_list: None,
            interface: None,
        };
        let text_service = TextService{
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
    fn interface<I: ComInterface>(&self) -> Result<I> {
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
        self.candidate_list.as_ref().ok_or_else(||{
            // FIXME this very frequent
            // error!("Candidate list is None.");
            E_FAIL.into()
        })
    }
}

//----------------------------------------------------------------------------
//
//  Now see tsf/text_input_processor.rs for the implementation.
//
//----------------------------------------------------------------------------