use log::{debug, trace};
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink_Impl, ITfKeyEventSink, ITfContextComposition}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::{GUID, ComInterface, implement}};
use windows::core::Result;

//----------------------------------------------------------------------------
//
//  A "sink" for key events. from here on the processing of inputs begins.
//
//----------------------------------------------------------------------------

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink;

impl KeyEventSink {
    pub fn new() -> KeyEventSink {
        KeyEventSink{}
    }
}
impl ITfKeyEventSink_Impl for KeyEventSink {
    #[allow(non_snake_case)]
    fn OnSetFocus(&self, fforeground:BOOL) ->  Result<()> {
        Ok(())
    }
    
    // the return value suggests if the given char is "eaten" or not. if eaten the char won't be put onto the textarea
    // key_code indicates the key that is pressed
    // the 0-15 bits of the flag indicates the repeat count
    // see https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown for more info
    #[allow(non_snake_case)]
    fn OnKeyDown(&self, context: Option<&ITfContext>, key_code: WPARAM, flag:LPARAM) -> Result<BOOL> {
        trace!("OnKeyDown");
        let Some(context) = context else {
            // context is needed for editing
            return Ok(FALSE);
        };

        let context_composition = context.cast::<ITfContextComposition>()?;

        // just eat every 'a' for now for testing
        if key_code.0 as i32 == 0x41 {
            Ok(TRUE)
        } else {
            Ok(FALSE)
        }
    }
    

    #[allow(non_snake_case)]
    fn OnKeyUp(&self,context: Option<&ITfContext>,wparam:WPARAM,lparam:LPARAM) -> Result<BOOL> {
        trace!("OnKeyUp");
        Ok(FALSE)
    }

    #[allow(non_snake_case)]
    fn OnTestKeyDown(&self, context: Option<&ITfContext>, wparam:WPARAM,lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyDown");
        Ok(TRUE)
    }

    #[allow(non_snake_case)]
    fn OnTestKeyUp(&self,context: Option<&ITfContext>,wparam:WPARAM,lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyUp");
        Ok(FALSE)
    }

    #[allow(non_snake_case)]
    fn OnPreservedKey(&self,context: Option<&ITfContext>,rguid: *const GUID) -> Result<BOOL> {
        trace!("OnPreservedKey");
        Ok(FALSE)
    }
}