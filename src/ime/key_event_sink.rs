use std::{sync::RwLock, collections::HashSet};

use log::{trace, warn, debug};
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl, ITfComposition}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::{GUID, implement}};
use windows::core::Result;

use crate::{ime::{edit_session::{start_composition, end_composition, set_text}, composition_sink::CompositionSink}, extend::{OsStrExt2, GUIDExt}, dict};

//----------------------------------------------------------------------------
//
//  A "sink" for key events. from here on the processing of inputs begins.
//  KeyEvenSink is only a proxy for an inner object because mutation is not 
//  allowed by the interface. 
//
//----------------------------------------------------------------------------

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink (RwLock<KeyEventSinkInner>);

impl KeyEventSink {
    pub fn new(tid: u32) -> KeyEventSink {
        KeyEventSink{0: RwLock::new(KeyEventSinkInner::new(tid))}
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink {
    #[allow(non_snake_case)]
    fn OnTestKeyDown(&self, context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyDown({:#04X})", wparam.0);
        self.0.write().unwrap().on_test_key_down(context, wparam, lparam)
    }
    #[allow(non_snake_case)]
    fn OnKeyDown(&self, context: Option<&ITfContext>, wparam: WPARAM, lparam:LPARAM) -> Result<BOOL> {
        trace!("OnKeyDown({:#04X})", wparam.0);
        self.0.write().unwrap().on_key_down(context, wparam, lparam)
    }
    #[allow(non_snake_case)]
    fn OnTestKeyUp(&self, context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyUp({:#04X})", wparam.0);
        self.0.write().unwrap().on_test_key_up(context, wparam, lparam)
    }
    #[allow(non_snake_case)]
    fn OnKeyUp(&self, context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        trace!("OnKeyUp({:#04X})", wparam.0);
        self.0.write().unwrap().on_key_up(context, wparam, lparam)
    }
    #[allow(non_snake_case)]
    fn OnPreservedKey(&self, context: Option<&ITfContext>, rguid: *const GUID) -> Result<BOOL> {
        trace!("OnPreservedKey({:?})", unsafe{ rguid.as_ref() }.map(GUID::to_rfc4122));
        self.0.read().unwrap().on_preserved_key(context, rguid)
    }
    #[allow(non_snake_case)]
    fn OnSetFocus(&self, fforeground:BOOL) ->  Result<()> {
        trace!("OnSetFocus");
        self.0.read().unwrap().on_set_focus(fforeground)
    }
}


//----------------------------------------------------------------------------
//
//  First thing first is to simplify the overly complicated key events to "inputs"
//
//----------------------------------------------------------------------------

enum Input{
    Letter(u8), Punct(u8), Space, Backspace, Enter, Unknown
}

impl Input {
    fn is_letter(&self) -> bool {
        match self {
            Self::Letter(_) => true,
            _ => false,
        }
    }
}

pub struct KeyEventSinkInner {
    composition: Composition,
    caws: HashSet<usize>, // ctrl, alt, win
    shift: bool,
}

impl KeyEventSinkInner {
    fn new(tid: u32) -> KeyEventSinkInner {
        KeyEventSinkInner {
            composition: Composition::new(tid),
            caws: HashSet::new(),
            shift: false,
        }
    }
    // `wparam` indicates the key that is pressed.
    // The 0-15 bits of lparam indicates the repeat count
    // (See https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown for detail).

    /// The return value suggests if the key event **will be** eaten or not **if** `OnKeyDown` is called.
    /// 
    /// If `true`, the client **may** ignore the actual return value of `OnTestKeyDown` afterwards.
    /// Thus you cannot always return `true` to "capture" every event and expect to "release" them later
    ///  in `OnKeyDown` by returning `false`.
    /// 
    /// If `false`, the clinet **may** not call `OnKeyDown` afterwards.
    /// Thus try to gather any needed infomations and states in `OnTestKeyDown` if possible since it
    /// may be your only chance.
    fn on_test_key_down(&mut self, _context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        if is_caw(wparam) {
            // keep track of the shortcuts
            self.caws.insert(wparam.0);
            return Ok(FALSE);
        }
        if !self.caws.is_empty(){
            // avoid clashing with shortcuts
            return Ok(FALSE);
        }
        if is_shift(wparam) {
            return Ok(TRUE)
        }
        self.test_input(self.convert_event(wparam))
    }

    /// The return value suggests if the key event **is** eaten or not.
    /// The client might call `OnKeyDown` directly without calling `OnTestKeyDown` beforehand.
    fn on_key_down(&mut self, context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        if is_caw(wparam) {
            self.caws.insert(wparam.0);
            return Ok(FALSE);
        }
        if !self.caws.is_empty() {
            return Ok(FALSE);
        }
        // remember the "holding" state but not eat the event since
        // shift is also often a part of a shortcut
        if is_shift(wparam) {
            self.shift = true;
            return Ok(FALSE)
        }
        self.handle_input(self.convert_event(wparam), context)
    }

    /// Ignore all key up events. I know this may cause trouble in the future because there're always
    /// some asshole programs not calling these fuctions properly.
    fn on_test_key_up(&mut self, _context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        if is_caw(wparam) {
            self.caws.remove(&wparam.0);
        }
        if is_shift(wparam) {
            return Ok(TRUE);
        }
        Ok(FALSE)
    }

    fn on_key_up(&mut self, _context: Option<&ITfContext>, wparam:WPARAM, lparam:LPARAM) -> Result<BOOL> {
        if is_caw(wparam) {
            self.caws.remove(&wparam.0);
        }
        if is_shift(wparam) {
            self.shift = false;
            if self.caws.is_empty() {
                // todo ASCII mode toggle
            }
        }
        Ok(FALSE)
    }

    fn on_preserved_key(&self, _context: Option<&ITfContext>, _rguid: *const GUID) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn on_set_focus(&self, fforeground:BOOL) ->  Result<()> {
        // todo abort
        Ok(())
    }

    // see https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    fn convert_event(&self, wparam:WPARAM) -> Input {
        use Input::*;
        let key_code = wparam.0;
        match key_code {
            // A key ~ Z key, convert them to lowercase letters
            0x41..=0x5A => {
                let offset: u8 = (key_code - 0x41).try_into().unwrap();
                if self.shift {
                    Letter(b'A' + offset)
                } else {
                    Letter(b'a' + offset)
                }
            }
            // TODO punct
            0x20 => Space,
            0x0D => Enter,
            0x08 => Backspace,
            _ => Unknown
        }
    } 
}

fn is_shift(wparam:WPARAM) -> bool {
    wparam.0 == 0x10 || wparam.0 == 0xA0 || wparam.0 == 0xA1
}

fn is_caw(wparam:WPARAM) -> bool {
    wparam.0 == 0x11 || wparam.0 == 0xA2 || wparam.0 == 0xA3 || // ctrl
    wparam.0 == 0x12 || wparam.0 == 0xA4 || wparam.0 == 0xA4 || // alt
    wparam.0 == 0x5B || wparam.0 == 0x5C                        // win
}

//----------------------------------------------------------------------------
//
//  After simplifying the overly-complicated events, 
//  we can actually process them
//
//----------------------------------------------------------------------------

impl KeyEventSinkInner {
    fn test_input(&self, event: Input) -> Result<BOOL> {
        trace!("test_input");
        if self.composition.composing() {
            return Ok(TRUE);
        }
        if event.is_letter() {
            return Ok(TRUE);
        }
        // todo eat the punct too if necessary
        Ok(FALSE)
    }

    fn handle_input(&mut self, event: Input, context: Option<&ITfContext>) -> Result<BOOL> {
        trace!("handle_input");
        use self::Input::*;
        let Some(context) = context else {
            warn!("Context is None");
            return Ok(FALSE);
        };
        if !self.composition.composing() {
            match event {
                // only letters can start compositions
                Letter(letter) => self.composition.start(context, letter)?,
                // Punct(punct) => {
                //     // convert
                // },
                _ => {return Ok(FALSE)}
            }
        } else {
            match event {
                // calling these function while not composing would cause the program to crash
                //
                Letter(letter) => self.composition.push(context, letter)?,
                Space => self.composition.accept(context)?,
                Enter => self.composition.release(context)?,
                Punct(punct) => self.composition.release(context)?,
                Backspace => self.composition.pop(context)?,
                Unknown => {
                    self.composition.accept(context)?;
                    return Ok(FALSE);
                }
            }
        }
        return Ok(TRUE);
    }
}

//----------------------------------------------------------------------------
//
//  Composition is the texts held by the input method waiting to be "composed"
//  into proper output. These texts are underscored by default.
//
//----------------------------------------------------------------------------

struct Composition {
    tid: u32,
    composition: Option<ITfComposition>,
    letters: Vec<u16>,
    suggestion: Vec<u16>,
}

impl Composition {
    fn new (tid: u32) -> Composition {
        Composition {
            tid: tid,
            composition: None,
            letters: Vec::new(),
            suggestion: Vec::new(),
        }
    }

    // there are only two states: composing or not
    fn start(&mut self, context: &ITfContext, letter: u8) -> Result<()> {
        self.composition = Some(start_composition(self.tid, context, &CompositionSink{}.into())?);
        self.push(context, letter)
    }

    fn end(&mut self, context: &ITfContext) -> Result<()> {
        end_composition(self.tid, context, self.composition.as_ref().unwrap())?;
        self.composition = None;
        self.letters.clear();
        self.suggestion.clear();
        Ok(())
    }

    // to check the current state
    fn composing(&self) -> bool {
        self.composition.is_some()
    }

    // make things easier
    fn set_text(&self, context: &ITfContext, text:&[u16]) -> Result<()> {
        set_text(self.tid, context, unsafe { self.composition.as_ref().unwrap().GetRange()? }, text)
    }

    // FIXME this function is slow-ass
    fn set_text_as_suggestions_and_letters(&self, context: &ITfContext) -> Result<()> {
        if self.suggestion.is_empty() {
            self.set_text(context, &self.letters)
        } else {
            let mut buf: Vec<u16> = Vec::with_capacity("[]".len() + self.suggestion.len() + self.letters.len());
            buf.push(b'['.into());
            buf.extend_from_slice(&self.suggestion);
            buf.push(b']'.into());
            buf.extend_from_slice(&self.letters);
            self.set_text(context, &buf)
        }
    }

    // handle input and transit state
    fn push(&mut self, context: &ITfContext, letter: u8) -> Result<()>{
        // todo auto-commit
        self.letters.push(letter.into());
        self.suggestion = dict::suggest(&self.letters);
        self.set_text_as_suggestions_and_letters(context)
    }

    fn pop(&mut self, context: &ITfContext) -> Result<()>{
        // todo auto-commit
        self.letters.pop();
        if self.letters.is_empty() {
            self.abort(context)?;
            return Ok(());
        } 
        self.suggestion = dict::suggest(&self.letters);
        self.set_text_as_suggestions_and_letters(context)
    }

    // accept the first suggestion
    fn accept(&mut self, context: &ITfContext) -> Result<()>{
        if self.suggestion.is_empty() {
            self.letters.push(b' '.into());
            self.set_text(context, &self.letters)?;
        } else {
            self.set_text(context, &self.suggestion)?;
        }
        self.end(context)
    }

    // select the desired suggestion by pressing num keys (or maybe tab, enter or any thing else)
    #[allow(dead_code)]
    fn select(&mut self, _context: &ITfContext) -> Result<()> {
        todo!("for v0.1 there's not multiple candidates to select from")
    }

    // release the raw ascii chars
    fn release(&mut self, context: &ITfContext) -> Result<()> {
        self.set_text(context, &self.letters)?;
        self.end(context)
    }

    // interupted. abort everything.
    fn abort(&mut self, context: &ITfContext) -> Result<()> {
        self.set_text(context, &[])?;
        self.end(context)
    }
}