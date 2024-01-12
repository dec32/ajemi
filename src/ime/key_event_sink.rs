use std::{sync::RwLock, ffi::OsStr, os::windows::ffi::OsStrExt};

use log::trace;
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink_Impl, ITfKeyEventSink, ITfComposition, ITfCompositionSink_Impl, ITfCompositionSink}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::{GUID, ComInterface, implement}};
use windows::core::Result;

use crate::ime::{edit_session::{start_composition, end_composition, set_text}, composition_sink::CompositionSink};

//----------------------------------------------------------------------------
//
//  A "sink" for key events. from here on the processing of inputs begins.
//
//----------------------------------------------------------------------------

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink (RwLock<Inner>);

impl KeyEventSink {
    pub fn new(tid: u32) -> KeyEventSink {
        let inner = Inner::new(tid);
        KeyEventSink{0: RwLock::new(inner)}
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink {
    #[allow(non_snake_case)]
    fn OnSetFocus(&self, fforeground:BOOL) ->  Result<()> {
        Ok(())
    }

    #[allow(non_snake_case)]
    fn OnPreservedKey(&self, _context: Option<&ITfContext>, _rguid: *const GUID) -> Result<BOOL> {
        trace!("OnPreservedKey");
        Ok(FALSE)
    }

    // OnKeyDown is called only when OnTestKeyDown returns true
    #[allow(non_snake_case)]
    fn OnTestKeyDown(&self, context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyDown");
        Ok(TRUE)
    }

    // the return value suggests if the given char is "eaten" or not.
    // if eaten the char won't be put onto the textarea
    // key_code indicates the key that is pressed
    // the 0-15 bits of the flag indicates the repeat count
    // see https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown for more info
    #[allow(non_snake_case)]
    fn OnKeyDown(&self, context: Option<&ITfContext>, key_code: WPARAM, flag:LPARAM) -> Result<BOOL> {
        trace!("OnKeyDown");
        // TODO detect shift
        let Some(context) = context else {
            // context is needed for editing
            return Ok(FALSE);
        };        

        let key_code = key_code.0;
        let key_event = match key_code {
            // A key ~ Z key, convert them to lowercase letters
            0x41..=0x5A => {
                let key_code:u8 = key_code.try_into().unwrap();
                KeyEvent::Letter(0x61 + (key_code - 0x41))
            }
            // TODO punct
            0x20 => KeyEvent::Space,
            0x08 => KeyEvent::Backspace,
            _ => {
                return Ok(FALSE);
            }
        };
        self.0.write().unwrap().on_event(key_event, context)
    }

    // OnKeyUp is called only when OnTestKeyUp returns true
    #[allow(non_snake_case)]
    fn OnTestKeyUp(&self, _context: Option<&ITfContext>, _wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyUp");
        Ok(FALSE)
    }

    #[allow(non_snake_case)]
    fn OnKeyUp(&self, context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        trace!("OnKeyUp");
        Ok(FALSE)
    }
}

//----------------------------------------------------------------------------
//
//  After simplifying the overly-complicated events, 
//  we can actually implement the processing
//
//----------------------------------------------------------------------------

enum KeyEvent{
    Letter(u8),
    Punct(u8),
    Space,
    Backspace,
}

pub struct Inner {
    tid: u32,
    composition: Option<ITfComposition>,
    letters: Vec<u16> // ANSI
}

impl Inner {
    pub fn new(tid: u32) -> Inner {
        Inner{
            tid: tid,
            composition: None,
            letters: Vec::new()
        }
    }

    fn update_compostition(&self, context: &ITfContext) -> Result<()>{
        set_text(
            self.tid, 
            context, 
            unsafe { &self.composition.as_ref().unwrap().GetRange()? }, 
            &self.letters)
    }

    fn on_event(&mut self, event: KeyEvent, context: &ITfContext) -> Result<BOOL> {
        use self::KeyEvent::*;
        match &self.composition {
            None => {
                match event {
                    // only letters can start compositions
                    Letter(letter) => {
                        let composition_sink = CompositionSink{}.into();
                        self.composition = Some(start_composition(self.tid, context, &composition_sink)?);
                        self.letters.clear();
                        self.letters.push(letter.into());
                        self.update_compostition(context)?;
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
                        self.letters.push(letter.into());
                        self.update_compostition(context)?;
                    },
                    // end composition
                    Space => {
                        // todo select word
                        self.letters = OsStr::new("天杀的微软文档").encode_wide().chain(Some(0).into_iter()).collect();
                        self.update_compostition(context)?;
                        end_composition(self.tid, context, &composition)?;
                        self.composition = None;
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