mod text_input_processor_ex;
mod key_event_sink;

use std::ffi::c_void;
use std::ptr;

use windows::Win32::UI::TextServices::{ITfTextInputProcessorEx, ITfKeyEventSink, ITfThreadMgr};
use windows::core::{implement, GUID, Result};

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

    pub fn query_interface(&mut self, _interface_id: *const GUID) -> Result<*mut c_void> {
        // todo unsure if this is valid
        Ok(self as *mut _ as *mut c_void)
    }
}



