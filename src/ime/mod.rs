mod text_input_processor_ex;

use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::core::implement;

//----------------------------------------------------------------------------
//
//  The IME struct. It's supposed to implement a ton of interfaces.
//
//----------------------------------------------------------------------------

#[implement(ITfTextInputProcessorEx)]
struct IME {

}
