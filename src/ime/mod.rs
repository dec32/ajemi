mod text_input_processor_ex;

use windows::Win32::UI::TextServices::{ITfTextInputProcessorEx,ITfThreadMgrEventSink,ITfTextEditSink,ITfTextLayoutSink,ITfKeyEventSink,ITfCompositionSink,ITfThreadFocusSink,ITfActiveLanguageProfileNotifySink,ITfEditSession,ITfDisplayAttributeProvider};
use windows::core::implement;

//----------------------------------------------------------------------------
//
//  The IME struct. It's supposed to implement a ton of interfaces.
//
//----------------------------------------------------------------------------

#[implement(
    ITfTextInputProcessorEx,
    /*
    ITfTextEditSink,
    ITfThreadMgrEventSink,
    ITfTextLayoutSink,
    ITfKeyEventSink,
    ITfCompositionSink,
    ITfThreadFocusSink,
    ITfActiveLanguageProfileNotifySink,
    ITfEditSession,
    ITfDisplayAttributeProvider*/)]
struct IME {

}
