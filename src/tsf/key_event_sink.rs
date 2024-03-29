use core::fmt;
use std::ffi::OsString;
use log::{debug, trace, warn};
use windows::{core::GUID, Win32::{Foundation::{BOOL, FALSE, LPARAM, TRUE, WPARAM}, UI::TextServices::{ITfContext, ITfKeyEventSink_Impl}}};
use windows::core::Result;
use crate::{extend::{GUIDExt, OsStrExt2}, engine::engine};
use super::{edit_session, TextService, TextServiceInner};
use self::Input::*;
use self::Shortcut::*;
//----------------------------------------------------------------------------
//
//  A "sink" for key events. From here on the processing begins.
//  First thing first is to simplify the overly complicated key events to "inputs"
//
//----------------------------------------------------------------------------

#[allow(non_snake_case)]
impl ITfKeyEventSink_Impl for TextService {
    /// The return value suggests if the key event **will be** eaten or not **if** `OnKeyDown` is called.
    /// 
    /// If `true`, the client **may** ignore the actual return value of `OnTestKeyDown` afterwards.
    /// Thus you cannot always return `true` to "capture" every event and expect to "release" them later
    ///  in `OnKeyDown` by returning `false`.
    /// 
    /// If `false`, the clinet **may** not call `OnKeyDown` afterwards.
    /// Thus try to gather any needed infomations and states in `OnTestKeyDown` if possible since it
    /// may be your only chance.
    /// 
    /// `wparam` indicates the key that is pressed.
    /// The 0-15 bits of `_lparam` indicates the repeat count (ignored here because it's actually always 1). 
    /// (See https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-keydown for detail).
    fn OnTestKeyDown(&self, _context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyDown({:#04X})", wparam.0);
        let mut inner = self.write()?;
        // remember the "hold" of the modifiers but not eat the event since 
        // they can be parts of a shortcut
        if inner.modifiers.try_insert(wparam) {
            return Ok(FALSE);
        }
        if inner.modifiers.preparing_shortcut() {
            let shorcut = Shortcut::from(wparam.0, inner.modifiers.ctrl(), inner.modifiers.alt(), inner.modifiers.shift());
            inner.test_shortcut(shorcut)
        } else {
            let input = Input::from(wparam.0, inner.modifiers.shift());
            inner.test_input(input)
        }
    }

    /// The return value suggests if the key event **is** eaten or not.
    /// The client might call `OnKeyDown` directly without calling `OnTestKeyDown` beforehand.
    /// The client might call `OnKeyDown` even if `OnTestKeyDown` returned `false`.
    /// The client can be an asshole. Remember that.
    fn OnKeyDown(&self, context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnKeyDown({:#04X})", wparam.0);
        let mut inner = self.write()?;
        if inner.modifiers.try_insert(wparam) {
            return Ok(FALSE);
        }
        if inner.modifiers.preparing_shortcut() {
            let shorcut = Shortcut::from(wparam.0, inner.modifiers.ctrl(), inner.modifiers.alt(), inner.modifiers.shift());
            inner.handle_shortcut(shorcut)
        } else {
            let input = Input::from(wparam.0, inner.modifiers.shift());
            inner.handle_input(input, context)
        }
    }

    /// Flip the modifiers back
    fn OnTestKeyUp(&self, _context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyUp({:#04X})", wparam.0);
        self.write()?.modifiers.try_remove(wparam);
        Ok(FALSE)
    }

    fn OnKeyUp(&self, _context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnKeyUp({:#04X})", wparam.0);
        self.write()?.modifiers.try_remove(wparam);
        Ok(FALSE)
    }

    /// I 've never seen this thing called.
    fn OnPreservedKey(&self, _context: Option<&ITfContext>, rguid: *const GUID) -> Result<BOOL> {
        trace!("OnPreservedKey({:?})", unsafe{ rguid.as_ref() }.map(GUID::to_rfc4122));
        Ok(FALSE)
    }

    fn OnSetFocus(&self, foreground:BOOL) -> Result<()> {
        trace!("OnSetFocus({})", foreground.as_bool());
        if !foreground.as_bool() {
            self.write()?.abort()
        } else {
            Ok(())
        }
    }
}

/// Keep track of if the modifiers(CTRL, ALT and SHIFT).
/// WIN is ignored since Windows already captures all shortcuts containing WIN for us.
/// See https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes for keycodes.
#[derive(Default)]
pub struct Modifiers {
    pressed: [bool;9],
    shift_count: u8,
    ctrl_count: u8,
    alt_count: u8,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers::default()
    }

    fn try_insert(&mut self, wparam: WPARAM) -> bool {
        let Some(index) = Self::index(wparam) else {
            return false;
        };
        if self.pressed[index] == false {
            self.pressed[index] = true;
            if Self::is_shift(wparam) {
                self.shift_count += 1;
            } else if Self::is_ctrl(wparam) {
                self.ctrl_count += 1;
            } else {
                self.alt_count += 1;
            }
        } 
        debug!("{:?}", self);
        true
    }

    fn try_remove(&mut self, wparam: WPARAM) -> bool { 
        let Some(index) = Self::index(wparam) else {
            return false;
        };
        if self.pressed[index] == true {
            self.pressed[index] = false;
            if Self::is_shift(wparam) {
                self.shift_count -= 1;
            } else if Self::is_ctrl(wparam) {
                self.ctrl_count -= 1;
            } else {
                self.alt_count -= 1;
            }
        }
        debug!("{:?}", self);
        true
    }

    fn preparing_shortcut(&self) -> bool {
        self.ctrl() || self.alt()
    }

    fn ctrl(&self) -> bool {
        self.ctrl_count > 0
    }

    fn alt(&self) -> bool {
        self.alt_count > 0
    }

    fn shift(&self) -> bool {
        self.shift_count > 0
    }
}

#[allow(unused)]
impl Modifiers {
    const SHIFT:  usize = 0x10;
    const CTRL:   usize = 0x11;
    const ALT:    usize = 0x12;

    const LSHIFT: usize = 0xA0;
    const RSHIFT: usize = 0xA1;
    const LCTRL:  usize = 0xA2;
    const RCTRL:  usize = 0xA3;
    const LATL:   usize = 0xA4;
    const RALT:   usize = 0xA5;

    const fn index(wparam: WPARAM) -> Option<usize> {
        match wparam.0 {
            0x10..=0x12 => Some(wparam.0 - 0x10),
            0xA0..=0xA5 => Some(wparam.0 - 0xA0 + 3),
            _ => None
        }
    }
    const NAMES: [&'static str;9] = [
        "SHIFT", "CTRL", "ALT", "LSHIFT", "RSHIFT", "LCTRL", "RCTRL", "LALT", "RALT"];

    const fn is_shift(wparam: WPARAM) -> bool {
        wparam.0 == Self::SHIFT ||
        wparam.0 == Self::LSHIFT ||
        wparam.0 == Self::RSHIFT
    }

    const fn is_ctrl(wparam: WPARAM) -> bool {
        wparam.0 == Self::CTRL ||
        wparam.0 == Self::LCTRL ||
        wparam.0 == Self::RCTRL
    }
}

impl fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Modifiers: [")?;
        let mut comma = false;
        for (index, pressed) in self.pressed.iter().enumerate() {
            if *pressed {
                if comma == false {
                    comma = true
                } else {
                    f.write_str(", ")?;
                }
                f.write_str(Self::NAMES[index])?;
            }
        }
        f.write_str("]")
    }
}

#[derive(Debug)]
enum Shortcut {
    Switch(usize),
    Undefine,
}

impl Shortcut {
    fn from(key_code: usize, ctrl: bool, alt: bool, shift: bool) -> Shortcut {
        trace!("Shortcut::from({}, {}, {}, {})", key_code, ctrl, alt, shift);
        use Shortcut::*;
        let input = Input::from(key_code, false);
        match (ctrl, alt, shift, input)  {
            (true, false, true, Number(num)) => Switch(num),
            _ => Undefine
        }
    }
}


/// Inputs that are easier to understand and handle.
/// See https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes for keycodes.
#[derive(Debug)]
enum Input {
    Letter(char), Number(usize), Punct(char),
    Space, Backspace, Enter, Tab,
    Left, Up, Right, Down,
    Unknown(usize)
}

impl Input {
    fn from(key_code: usize, shift: bool) -> Input {
        use Input::*;
        fn offset(key_code: usize, from: usize ) -> u8 {
            (key_code - from).try_into().unwrap()
        }
        fn add(ch: char, offset: u8) -> char {
            let char: u8 = ch.try_into().unwrap();
            let sum: u8 = char + offset;
            sum.try_into().unwrap()
        }
        match (key_code, shift) {
            // Letter keys
            (0x41..=0x5A, false) => Letter(add('a', offset(key_code, 0x41))),
            (0x41..=0x5A, true ) => Letter(add('A', offset(key_code, 0x41))),
            // Numbers
            (0x30..=0x39, false) => Number(0 + (key_code - 0x30)),
            (0x60..=0x69, _    ) => Number(0 + (key_code - 0x60)),
            // Punctuators
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
            // Punctuators, the miscellaneous ones as Microsoft calls them
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
            // Punctuators, the numpad ones
            (0x6A, _    ) => Punct('*'),
            (0x6B, _    ) => Punct('+'),
            (0x6C, _    ) => Punct('/'),
            (0x6D, _    ) => Punct('-'),
            // The special keys. They are for editing and operations.
            (0x08, _    ) => Backspace,
            (0x09, _    ) => Tab,
            (0x0D, _    ) => Enter,
            (0x20, _    ) => Space,
            (0x25, _    ) => Left,
            (0x26, _    ) => Up,
            (0x27, _    ) => Right,
            (0x28, _    ) => Down,
            
            _ => Unknown(key_code)
        }
    }
}

//----------------------------------------------------------------------------
//
//  After simplifying the overly-complicated events, we can start handling them.
//  Everthing after this point happens in "inner". Locking is nomore needed.
//
//----------------------------------------------------------------------------

impl TextServiceInner {
    fn test_input(&self, input: Input) -> Result<BOOL> {
        trace!("test_input({:?})", input);
        if self.composition.is_none() {
            match input {
                Letter(_) | Punct(_) | Space => Ok(TRUE),
                _ => Ok(FALSE),
            }
        } else {
            Ok(TRUE)
        }
    }

    fn handle_input(&mut self, input: Input, context: Option<&ITfContext>) -> Result<BOOL> {
        trace!("handle_input({:?})", input);
        let Some(context) = context else {
            warn!("Context is None");
            return Ok(FALSE);
        };
        self.context = Some(context.clone());
        if self.composition.is_none() {
            match input {
                // letters start compositions. punctuators need to be re-mapped.
                Letter(letter) => {
                    self.start_composition()?;
                    self.push(letter)?
                },
                Punct(punct) => {
                    self.insert_char(engine().remap_punct(punct))?
                },
                Space => {
                    self.insert_char(engine().remap_punct(' '))?
                }
                _ => {return Ok(FALSE)}
            }
        } else {
            match input {
                Letter(letter) => self.push(letter)?,
                Number(number) => {
                    if number == 0 {
                        ()
                    } else {
                        self.select(number - 1)?
                    }
                }
                Punct(punct) => self.force_commit(engine().remap_punct(punct))?,
                Space => self.commit()?,
                Enter => self.release()?,
                Backspace => self.pop()?,
                Tab => {
                    self.push(' ')?;
                    self.release()?
                } 
                // disable cursor movement because I am lazy.
                Left | Up | Right | Down => (),
                Unknown(_) => {
                    return Ok(FALSE);
                }
            }
        }
        return Ok(TRUE);
    }

    fn insert_char(&mut self, ch: char) -> Result<()> {
        self.char_buf.clear();
        self.char_buf.push(ch);
        let text = OsString::from(&self.char_buf).wchars();
        edit_session::insert_text(self.tid, self.context()?, &text)
    }

    fn test_shortcut(&self, shortcut: Shortcut) -> Result<BOOL> {
        if self.composition.is_none() {
            match shortcut {
                Switch(1) => Ok(TRUE),
                _ => Ok(FALSE),
            }
        } else {
            Ok(FALSE)
        }
    }

    fn handle_shortcut(&self, shortcut: Shortcut) -> Result<BOOL> {
        if self.composition.is_none() {
            match shortcut {
                Switch(1) => {    
                    engine().next_schema();
                    Ok(TRUE)
                }
                _ => Ok(FALSE),
            }
        } else {
            Ok(FALSE)
        }
    }
}

