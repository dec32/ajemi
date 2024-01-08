mod text_input_processor_ex;
mod key_event_sink;

use std::ffi::c_void;
use std::{ptr, mem};

use log::{debug, error};
use windows::Win32::Foundation::E_NOINTERFACE;
use windows::Win32::UI::TextServices::{ITfTextInputProcessorEx, ITfKeyEventSink, ITfThreadMgr};
use windows::core::{implement, GUID, Result, ComInterface, Error};

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
    ITfTextInputProcessorEx,
    ITfKeyEventSink
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
            ITfTextInputProcessorEx::IID => {
                debug!("ITfTextInputProcessorEx is required.");
                mem::transmute(ITfTextInputProcessorEx::from(self))
            },
            ITfKeyEventSink::IID => {
                debug!("ITfKeyEventSink is required.");
                mem::transmute(ITfKeyEventSink::from(self))
            },
            _ => {
                error!("The required interface is not implemented.");
                result = Err(Error::from(E_NOINTERFACE));
                ptr::null_mut()
            }
        };
        result
    }
}



