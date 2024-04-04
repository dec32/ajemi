use std::ffi::OsString;
use log::{trace, warn};
use windows::{core::GUID, Win32::{Foundation::{BOOL, FALSE, LPARAM, TRUE, WPARAM}, UI::{Input::KeyboardAndMouse::{VK_CAPITAL, VK_CONTROL, VK_LCONTROL, VK_LSHIFT, VK_MENU, VK_RCONTROL, VK_RSHIFT, VK_SHIFT}, TextServices::{ITfContext, ITfKeyEventSink_Impl}}}};
use windows::core::Result;
use crate::{engine::engine, extend::{GUIDExt, OsStrExt2, VKExt}};
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
        // disable the IME completly when CapsLock is on
        if VK_CAPITAL.is_toggled() {
            inner.abort()?;
            return Ok(FALSE);
        }
        // detect shortcut
        if let Some(shortcut) = Shortcut::try_from(wparam.0) {
            return inner.test_shortcut(shortcut);
        }
        let input = Input::from(wparam.0);
        inner.test_input(input)
    }

    /// The return value suggests if the key event **is** eaten or not.
    /// The client might call `OnKeyDown` directly without calling `OnTestKeyDown` beforehand.
    /// The client might call `OnKeyDown` even if `OnTestKeyDown` returned `false`.
    /// The client can be an asshole. Remember that.
    fn OnKeyDown(&self, context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnKeyDown({:#04X})", wparam.0);
        let mut inner = self.write()?;
        if VK_CAPITAL.is_toggled() {
            inner.abort()?;
            return Ok(FALSE);
        }
        if let Some(shortcut) = Shortcut::try_from(wparam.0) {
            return inner.handle_shortcut(shortcut);
        }
        let input = Input::from(wparam.0);
        inner.handle_input(input, context)
    }

    /// Flip the modifiers back
    fn OnTestKeyUp(&self, _context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyUp({:#04X})", wparam.0);
        Ok(FALSE)
    }

    fn OnKeyUp(&self, _context: Option<&ITfContext>, wparam: WPARAM, _lparam: LPARAM) -> Result<BOOL> {
        trace!("OnKeyUp({:#04X})", wparam.0);
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

#[derive(Debug)]
enum Shortcut {
    NextSchema,
    Undefine,
}

impl Shortcut {
    fn try_from(key_code: usize) -> Option<Shortcut> {
        let ctrl = VK_CONTROL.is_down() || VK_LCONTROL.is_down() || VK_RCONTROL.is_down();
        let alt = VK_MENU.is_down();
        let shift = VK_SHIFT.is_down() || VK_LSHIFT.is_down() || VK_RSHIFT.is_down();
        match (ctrl, alt, shift, key_code) {
            (true, false, true, 0x4E) => Some(NextSchema), // Ctrl + Shift + N
            (true, ..) | (_, true, ..) => Some(Undefine),
            _ => None,
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
    fn from(key_code: usize) -> Input {
        fn offset(key_code: usize, from: usize ) -> u8 {
            (key_code - from).try_into().unwrap()
        }
        fn add(ch: char, offset: u8) -> char {
            let char: u8 = ch.try_into().unwrap();
            let sum: u8 = char + offset;
            sum.try_into().unwrap()
        }
        let shift = VK_SHIFT.is_down() || VK_LSHIFT.is_down() || VK_RSHIFT.is_down();
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
                NextSchema => Ok(TRUE),
                _ => Ok(FALSE),
            }
        } else {
            Ok(FALSE)
        }
    }

    fn handle_shortcut(&self, shortcut: Shortcut) -> Result<BOOL> {
        if self.composition.is_none() {
            match shortcut {
                NextSchema => {    
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

