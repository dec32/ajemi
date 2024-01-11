use log::{debug, trace};
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink_Impl, ITfKeyEventSink, ITfContextComposition, ITfComposition, ITfCompositionSink_Impl, ITfCompositionSink}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::{GUID, ComInterface, implement}};
use windows::core::Result;

use crate::ime::edit_session::{start_composition, end_composition};

//----------------------------------------------------------------------------
//
//  A "sink" for key events. from here on the processing of inputs begins.
//
//----------------------------------------------------------------------------

#[implement(ITfKeyEventSink, ITfCompositionSink)]
pub struct KeyEventSink {
    tid: u32,
    composition: Option<ITfComposition>,
    composing: bool,
    letters: Vec<u8>
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
        Ok(FALSE)
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


//----------------------------------------------------------------------------
//
//  After simplifying the overly-complicated events, 
//  the processing of inputs really begins.
//
//----------------------------------------------------------------------------

enum KeyEvent{
    Letter(u8),
    Punct(u8),
    Space,
    Backspace,
}


impl KeyEventSink {
    pub fn new(tid: u32) -> KeyEventSink {
        KeyEventSink{
            tid: tid,
            composition: None,
            composing: false,
            letters:Vec::new()
        }
    }

    fn append_letter(&mut self, letter: u8) {
        self.letters.push(letter);
        // todo: render
    }
}

impl KeyEventSink {
    fn on_event(&mut self, event: KeyEvent, context: &ITfContext) -> Result<BOOL>  {
        use self::KeyEvent::*;

        match &self.composition {
            None => {
                match event {
                    // only letters can start compositions
                    Letter(letter) => {
                        let composition_sink = unsafe{self.cast()?};
                        self.composition = Some(start_composition(
                            self.tid, context, &composition_sink)?);
                        self.append_letter(letter);
                        Ok(TRUE)
                    },
                    // Punct(punct) => {
                    //     // convert
                    // },
                    _ => {return Ok(FALSE)}
                }
            },

            Some(composition) => {
                match event {
                    // append
                    Letter(letter) => {
                        self.append_letter(letter);
                    },
                    // end composition
                    Space => {
                        // todo select word
                        end_composition(self.tid, context, &composition);
                    },
                    Punct(punct) => {
    
                    },
                    Backspace => {
    
                    }
                };
                Ok(TRUE)


            }
        }
    }
}




impl ITfCompositionSink_Impl for KeyEventSink {
    fn OnCompositionTerminated(&self,ecwrite:u32,pcomposition: ::core::option::Option<&ITfComposition>) -> Result<()> {
        Ok(())
    }
}