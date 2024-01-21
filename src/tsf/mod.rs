pub mod text_input_processor;
pub mod candidate_list;
mod key_event_sink;
mod composition;
mod edit_session;
mod langbar_item;

use std::collections::HashSet;
use log::error;
use try_lock::{TryLock, Locked};
use windows::{core::{Result, implement, AsImpl, Error, ComInterface}, Win32::{UI::{TextServices::{ITfTextInputProcessor, ITfTextInputProcessorEx, ITfComposition, ITfThreadMgr, ITfKeyEventSink, ITfCompositionSink, ITfLangBarItem, ITfContext}, WindowsAndMessaging::HICON}, Foundation::E_FAIL}};

use self::candidate_list::CandidateList;

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
    ITfCompositionSink,
    ITfLangBarItem
)]

/// Methods of TSF interfaces don't allow mutation of any kind. Thus all mutable 
/// states are hidden behind a lock. The lock is supposed to be light-weight since
/// inputs from user can be frequent. 
pub struct TextService {
    inner: TryLock<TextServiceInner>
}
struct TextServiceInner {
    // Some basic info about the clinet (the program where user is typing)
    tid: u32,
    thread_mgr: Option<ITfThreadMgr>,
    context: Option<ITfContext>,
    
    // KeyEventSink
    caws: HashSet<usize>, // ctrl, alt, win
    shift: bool,

    // Composition
    composition: Option<ITfComposition>,
    spelling: String,
    output: String,
    groupping: Vec<usize>,
    
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
            caws: HashSet::new(),
            shift: false,
            spelling: String::with_capacity(32),
            output: String::with_capacity(32),
            groupping: Vec::with_capacity(32),
            composition: None,
            icon: HICON::default(),
            candidate_list: None,
            interface: None,
        };
        let text_service = TextService{inner: TryLock::new(inner)};
        // from takes ownership of the object and returns a smart pointer
        let interface = ITfTextInputProcessor::from(text_service);
        // inject the smart pointer back to the object
        let text_service: &TextService = unsafe {interface.as_impl()};
        text_service.inner.try_lock().ok_or(Error::from(E_FAIL))?.interface = Some(interface.clone());
        // cast the interface to desired type
        interface.cast()
    }

    /// Altho inputs can be frequent, data races caused by typing too fast
    /// is very unlikely to happen. So only `try_lock` is used here.
    fn inner(&self) -> Result<Locked<TextServiceInner>> {
        // TODO spin maybe
        self.inner.try_lock().ok_or(Error::from(E_FAIL))
    }
}

impl TextServiceInner {
    fn interface<I: ComInterface>(&self) -> Result<I> {
        // guarenteed to be Some by TextService::create
        self.interface.as_ref().unwrap().cast()
    }

    fn candidate_list(&self) -> Result<&CandidateList> {
        self.candidate_list.as_ref().ok_or_else(||{
            error!("Candidate list is None.");
            Error::from(E_FAIL)
        })
    }

    fn context(&self) -> Result<&ITfContext> {
        self.context.as_ref().ok_or_else(||{
            error!("Context is None.");
            Error::from(E_FAIL)
        })
    }
}