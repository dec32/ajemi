use std::{sync::RwLock, collections::HashSet, ffi::{OsStr, OsString}, os::windows::ffi::OsStrExt};

use log::{trace, warn, debug};
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl, ITfComposition}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::{GUID, implement}};
use windows::core::Result;

use crate::{ime::{edit_session, composition_sink::CompositionSink}, extend::{GUIDExt, OsStrExt2}, engine};

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

pub struct KeyEventSinkInner {
    tid: u32,
    composition: Composition,
    caws: HashSet<usize>, // ctrl, alt, win
    shift: bool,
}

impl KeyEventSinkInner {
    fn new(tid: u32) -> KeyEventSinkInner {
        KeyEventSinkInner {
            tid: tid,
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
        self.test_input(Input::from(wparam.0, self.shift))
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
        self.handle_input(Input::from(wparam.0, self.shift), context)
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
}

fn is_shift(wparam:WPARAM) -> bool {
    wparam.0 == 0x10 || wparam.0 == 0xA0 || wparam.0 == 0xA1
}

fn is_caw(wparam:WPARAM) -> bool {
    wparam.0 == 0x11 || wparam.0 == 0xA2 || wparam.0 == 0xA3 || // ctrl
    wparam.0 == 0x12 || wparam.0 == 0xA4 || wparam.0 == 0xA4 || // alt
    wparam.0 == 0x5B || wparam.0 == 0x5C                        // win
}

/// see https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
enum Input{
    Letter(char), Number(char), Punct(char),
    Space, Backspace, Enter,
    Left, Up, Right, Down,
    Unknown
}

impl Input {
    fn is_letter(&self) -> bool {
        match self {
            Self::Letter(_) => true,
            _ => false,
        }
    }

    fn from (key_code: usize, shift: bool) -> Input {
        use Input::*;
        fn offset(key_code: usize, from: usize ) -> u8 {
            (key_code - from).try_into().unwrap()
        }
        fn add(char: char, offset: u8) -> char {
            let char: u8 = char.try_into().unwrap();
            let sum: u8 = char + offset;
            sum.try_into().unwrap()
        }
        match (key_code, shift) {
            // A key ~ Z key, convert them to lowercase letters
            (0x41..=0x5A, false) => Letter(add('a', offset(key_code, 0x41))),
            (0x41..=0x5A, true ) => Letter(add('A', offset(key_code, 0x41))),
            // Numbers
            (0x30..=0x39, false) => Number(add('0', offset(key_code, 0x30))),
            // Punctuators, the keycodes has nothing to do with ASCII values
            (0x31, true ) => Punct('!'),
            (0x32, true ) => Punct('@'),
            (0x33, true ) => Punct('#'),
            (0x34, true ) => Punct('$'),
            (0x35, true ) => Punct('%'),
            (0x36, true ) => Punct('^'),
            (0x37, true ) => Punct('&'),
            (0x38, true ) => Punct('*'),
            (0x39, true ) => Punct('('),
            (0x30, true ) => Punct(')'),
            // Punctuators
            (0xBA, false) => Punct(';'),
            (0xBA, true ) => Punct(':'),
            (0xBB, false) => Punct('='),
            (0xBB, true ) => Punct('+'),
            (0xBC, false) => Punct(','),
            (0xBC, true ) => Punct('<'),
            (0xBD, false) => Punct('-'),
            (0xBD, true ) => Punct('_'),
            (0xBE, false) => Punct('.'),
            (0xBE, true ) => Punct('>'),
            (0xBF, false) => Punct('/'),
            (0xBF, true ) => Punct('?'),
            (0xC0, false) => Punct('`'),
            (0xC0, true ) => Punct('~'),
            (0xDB, false) => Punct('['),
            (0xDB, true ) => Punct('{'),
            (0xDC, false) => Punct('\\'),
            (0xDC, true ) => Punct('|'),
            (0xDD, false) => Punct(']'),
            (0xDD, true ) => Punct('}'),
            (0xDE, false) => Punct('\''),
            (0xDE, true ) => Punct('"'),
            // The special ones. They are for editing and operations.
            (0x20, _) => Space,
            (0x0D, _) => Enter,
            (0x08, _) => Backspace,
            (0x25, _) => Left,
            (0x26, _) => Up,
            (0x27, _) => Right,
            (0x28, _) => Down,
            _ => Unknown
        }
    }
}
//----------------------------------------------------------------------------
//
//  After simplifying the overly-complicated events, 
//  we can actually process them
//
//----------------------------------------------------------------------------

impl KeyEventSinkInner {
    fn test_input(&self, event: Input) -> Result<BOOL> {
        use Input::*;
        trace!("test_input");
        if !self.composition.composing() {
            match event {
                Letter(_) | Punct(_) => Ok(TRUE),
                _ => Ok(FALSE),
            }
        } else {
            Ok(TRUE)
        }
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
                // letters start compositions. punctuators need to be re-mapped.
                Letter(letter) => self.composition.start(context, letter)?,
                Punct(punct) => self.insert_text(context, &engine::remap_punct(punct))?,
                _ => {return Ok(FALSE)}
            }
        } else {
            match event {
                Letter(letter) => self.composition.push(context, letter)?,
                Number(number) => {
                    // todo numbers can be used to select from candidate list
                    self.composition.commit_release(context)?;
                    self.insert_char(context, number)?;
                },
                Punct(punct) => {
                    // todo punctuator can be regarded as one-character auto commit
                    // but to support auto commit the searching algorithm needs to re-designed
                    self.composition.push(context, punct)?;
                    self.composition.commit_release(context)?;
                    // self.insert_text(context, &engine::remap_punct(punct))?;
                },
                Space => self.composition.commit(context)?,
                Enter => self.composition.release(context)?,
                Backspace => self.composition.pop(context)?,
                // disable cursor movement because I am lazy
                Left|Up|Right|Down => (),
                Unknown => {
                    self.composition.abort(context)?;
                    return Ok(FALSE);
                }
            }
        }
        return Ok(TRUE);
    }

    fn insert_text(&self, context: &ITfContext, text: &str) -> Result<()> {
        edit_session::insert_text(self.tid, context, &OsStr::new(text).wchars())
    }

    fn insert_char(&self, context: &ITfContext, char: char) -> Result<()> {
        edit_session::insert_text(self.tid, context, &[char.try_into().unwrap()])
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
    letters: String,
    suggestion: String,
}

impl Composition {
    fn new (tid: u32) -> Composition {
        Composition {
            tid: tid,
            composition: None,
            letters: String::new(),
            suggestion: String::new(),
        }
    }

    // there are only two states: composing or not
    fn start(&mut self, context: &ITfContext, letter: char) -> Result<()> {
        self.composition = Some(edit_session::start_composition(self.tid, context, &CompositionSink{}.into())?);
        self.push(context, letter)
    }

    fn end(&mut self, context: &ITfContext) -> Result<()> {
        edit_session::end_composition(self.tid, context, self.composition.as_ref().unwrap())?;
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
    fn set_text(&self, context: &ITfContext, text: &str) -> Result<()> {
        let text = OsString::from(text).wchars();
        let range = unsafe { self.composition.as_ref().unwrap().GetRange()? };
        edit_session::set_text(self.tid, context, range, &text)
    }

    // FIXME this function is slow-ass
    fn set_text_as_suggestions_and_letters(&self, context: &ITfContext) -> Result<()> {
        if self.suggestion.is_empty() {
            self.set_text(context, &self.letters)
        } else {
            let mut buf = String::with_capacity("[]".len() + self.suggestion.len() + self.letters.len());
            buf.push('[');
            buf += &self.suggestion;
            buf.push(']');
            buf += &self.letters;
            self.set_text(context, &buf)
        }
    }
}

// handle input and transit state
// calling these function while not composing would cause the program to crash
impl Composition {
    fn push(&mut self, context: &ITfContext, letter: char) -> Result<()>{
        // todo auto-commit
        self.letters.push(letter);
        self.suggestion = engine::suggest(&self.letters);
        self.set_text_as_suggestions_and_letters(context)
    }

    fn pop(&mut self, context: &ITfContext) -> Result<()>{
        // todo auto-commit
        self.letters.pop();
        if self.letters.is_empty() {
            self.abort(context)?;
            return Ok(());
        } 
        self.suggestion = engine::suggest(&self.letters);
        self.set_text_as_suggestions_and_letters(context)
    }

    // commit the suggestion
    fn commit(&mut self, context: &ITfContext) -> Result<()>{
        if self.suggestion.is_empty() {
            self.letters.push(b' '.into());
            self.set_text(context, &self.letters)?;
        } else {
            self.set_text(context, &self.suggestion)?;
        }
        self.end(context)
    }

    // commit the suggestion and release possble trailing ascii characters.
    fn commit_release(&mut self, context: &ITfContext) -> Result<()>{
        // todo
        self.commit(context)
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
        self.set_text(context, &"")?;
        self.end(context)
    }
}