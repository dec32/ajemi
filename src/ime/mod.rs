mod text_input_processor;
mod text_input_processor_ex;
mod key_event_sink;

use std::ffi::c_void;
use std::{ptr, mem};

use log::{debug, error};
use windows::Win32::Foundation::E_NOINTERFACE;
use windows::Win32::UI::TextServices::{ITfTextInputProcessor, ITfTextInputProcessorEx, ITfKeyEventSink, ITfThreadMgr};
use windows::core::{implement, GUID, Result, ComInterface, Error, Interface, AsImpl};

use crate::extend::{GUIDExt};

//----------------------------------------------------------------------------
//
//  The IME struct. It's supposed to implement a ton of interfaces.
//
//----------------------------------------------------------------------------

/*
Some concept

Context: The textarea you handling with.
EditSession: Edit of any form must be in the form of an EditSession object
Range: A selection of texts
Compsition: the ascii texts that is held by the input method waiting to be "composed" into other texts.

*/

/*
// Deactivate 和 EndEdit 的区别在哪里？
ITfTextInputProcessorEx,
ITfTextEditSink
// 用于在终止合成时接收通知
ITfCompositionSink,

ITfThreadMgrEventSink,
ITfTextLayoutSink,
ITfThreadFocusSink,
ITfActiveLanguageProfileNotifySink,
ITfDisplayAttributeProvider*/
#[implement(
    ITfTextInputProcessor,
    ITfTextInputProcessorEx,
    // ITfKeyEventSink
)]

pub struct Ime{
    // thread_mgr: * const ITfThreadMgr,
    // client_id: Option<u32>,
    // composing: bool
}

impl Ime {
    pub fn new() -> Ime {
        Ime{
            // thread_mgr: ptr::null(),
            // client_id: None,
            // composing: false
        }
    }

    pub unsafe fn query_interface(self, riid: *const GUID, out: *mut *mut c_void) -> Result<()>{
        let mut result = Ok(());
        *out = match *riid {
            ITfTextInputProcessor::IID => mem::transmute(ITfTextInputProcessor::from(self)),
            ITfTextInputProcessorEx::IID => mem::transmute(ITfTextInputProcessorEx::from(self)),
            // ITfKeyEventSink::IID => mem::transmute(ITfKeyEventSink::from(self)),
            _guid => {
                error!("The required interface {{{}}} is not implemented.", _guid.to_rfc4122());
                result = Err(Error::from(E_NOINTERFACE));
                ptr::null_mut()
            }
        };
        result
    }

    pub fn to_interface<I: ComInterface>(&self) -> I {
        AsImpl
        // unsafe {I::from(*(self as *const Ime))}
        unsafe {
            // let force_deref = *self;
            // I::from(wtf)
        }

    }
}



