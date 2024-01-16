use std::{sync::RwLock, collections::HashSet};
use log::{trace, warn};
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink, ITfKeyEventSink_Impl}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::{GUID, implement}};
use windows::core::Result;
use crate::{ime::edit_session, extend::GUIDExt, engine::engine};
use super::composition::Composition;

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
            tid,
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
    fn on_test_key_down(&mut self, _context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
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
    fn on_test_key_up(&mut self, _context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
        if is_caw(wparam) {
            self.caws.remove(&wparam.0);
        }
        if is_shift(wparam) {
            return Ok(TRUE);
        }
        Ok(FALSE)
    }

    fn on_key_up(&mut self, _context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
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

    fn on_set_focus(&self, _foreground:BOOL) ->  Result<()> {
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
            // More punctuators
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
                Letter(letter) => {
                    self.composition.start(context)?;
                    self.composition.push(context, letter)?
                },
                Punct(punct) => {
                    self.insert_char(context, engine().remap_punct(punct))?
                },
                _ => {return Ok(FALSE)}
            }
        } else {
            match event {
                Letter(letter) => self.composition.push(context, letter)?,
                Number(number) => {
                    // todo numbers can be used to select from candidate list
                    self.composition.push(context, number)?;
                    self.composition.force_commit(context)?;
                },
                Punct(punct) => {
                    self.composition.push(context, engine().remap_punct(punct))?;
                    self.composition.force_commit(context)?;

                    // the more proper way is:
                    //  
                    // self.composition.force_commit(context)?;
                    // self.insert_char(context, &engine::remap_punct(punct))?;
                    //
                    // however by doing so the suggestion will be eaten and print 2 puncts. no sure why.
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

    fn insert_char(&self, context: &ITfContext, char: char) -> Result<()> {
        trace!("insert_char('{char}')");
        edit_session::insert_text(self.tid, context, &[char.try_into().unwrap()])
    }
}

