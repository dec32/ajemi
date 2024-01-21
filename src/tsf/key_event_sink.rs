use std::ffi::OsString;
use log::{trace, warn};
use windows::{Win32::{UI::TextServices::{ITfContext, ITfKeyEventSink_Impl}, Foundation::{WPARAM, LPARAM, BOOL, TRUE, FALSE}}, core::GUID};
use windows::core::Result;
use crate::{extend::{GUIDExt, OsStrExt2}, engine::engine};
use super::{edit_session, TextService, TextServiceInner};
use self::Input::*;

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
    fn OnTestKeyDown(&self, _context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyDown({:#04X})", wparam.0);
        let mut inner = self.write()?;
        if is_caw(wparam) {
            // keep track of the shortcuts
            inner.caws.insert(wparam.0);
            return Ok(FALSE);
        }
        if !inner.caws.is_empty(){
            // avoid clashing with shortcuts
            return Ok(FALSE);
        }
        if is_shift(wparam) {
            return Ok(TRUE)
        }
        let input = Input::from(wparam.0, inner.shift);
        inner.test_input(input)
    }

    /// The return value suggests if the key event **is** eaten or not.
    /// The client might call `OnKeyDown` directly without calling `OnTestKeyDown` beforehand.
    fn OnKeyDown(&self, context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
        trace!("OnKeyDown({:#04X})", wparam.0);
        let mut inner = self.write()?;
        if is_caw(wparam) {
            inner.caws.insert(wparam.0);
            return Ok(FALSE);
        }
        if !inner.caws.is_empty() {
            return Ok(FALSE);
        }
        // remember the "holding" state but not eat the event since
        // shift is also often a part of a shortcut
        if is_shift(wparam) {
            inner.shift = true;
            return Ok(FALSE)
        }
        // let repeat = 0xFFFF & lparam.0;
        let input = Input::from(wparam.0, inner.shift);
        inner.handle_input(input, context)
    }

    /// Ignore all key up events. I know this may cause trouble in the future because there're always
    /// some asshole programs not calling these fuctions properly.
    fn OnTestKeyUp(&self, _context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
        trace!("OnTestKeyUp({:#04X})", wparam.0);
        let mut inner = self.write()?;
        if is_caw(wparam) {
            inner.caws.remove(&wparam.0);
        }
        if is_shift(wparam) {
            return Ok(TRUE);
        }
        Ok(FALSE)
    }

    fn OnKeyUp(&self, _context: Option<&ITfContext>, wparam:WPARAM, _lparam:LPARAM) -> Result<BOOL> {
        trace!("OnKeyUp({:#04X})", wparam.0);
        let mut inner = self.write()?;
        if is_caw(wparam) {
            inner.caws.remove(&wparam.0);
        }
        if is_shift(wparam) {
            inner.shift = false;
            if inner.caws.is_empty() {
                // todo ASCII mode toggle
            }
        }
        Ok(FALSE)
    }

    fn OnPreservedKey(&self, _context: Option<&ITfContext>, rguid: *const GUID) -> Result<BOOL> {
        trace!("OnPreservedKey({:?})", unsafe{ rguid.as_ref() }.map(GUID::to_rfc4122));
        Ok(FALSE)
    }

    fn OnSetFocus(&self, foreground:BOOL) ->  Result<()> {
        trace!("OnSetFocus({})", foreground.as_bool());
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
    Tab,
    Unknown
}

impl Input {
    fn from (key_code: usize, shift: bool) -> Input {
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
            (0x30..=0x39, false) => Number(add('0', offset(key_code, 0x30))),
            (0x60..=0x69, _    ) => Number(add('0', offset(key_code, 0x60))),
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
            
            _ => Unknown
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
    fn test_input(&self, event: Input) -> Result<BOOL> {
        trace!("test_input");
        if self.composition.is_none() {
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
        let Some(context) = context else {
            warn!("Context is None");
            return Ok(FALSE);
        };
        self.context = Some(context.clone());
        if self.composition.is_none() {
            match event {
                // letters start compositions. punctuators need to be re-mapped.
                Letter(letter) => {
                    self.start_composition()?;
                    self.push(letter)?
                },
                Punct(punct) => {
                    self.insert_char(engine().remap_punct(punct))?
                },
                _ => {return Ok(FALSE)}
            }
        } else {
            match event {
                Letter(letter) => self.push(letter)?,
                Number(number) => {
                    // todo numbers can be used to select from candidate list
                    self.force_commit(number)?;
                },
                Punct(punct) => {
                    self.force_commit(engine().remap_punct(punct))?;

                    // the more proper way is:
                    //  
                    // self.composition.force_commit(context)?;
                    // self.insert_char(context, &engine::remap_punct(punct))?;
                    //
                    // however by doing so the suggestion will be eaten and print 2 puncts. no sure why.
                },
                Space => self.commit()?,
                Enter => self.release()?,
                Backspace => self.pop()?,
                Tab => {
                    self.push(' ')?;
                    self.release()?
                } 
                // disable cursor movement because I am lazy.
                Left|Up|Right|Down => (),
                Unknown => {
                    self.abort()?;
                    return Ok(FALSE);
                }
            }
        }
        return Ok(TRUE);
    }

    fn insert_char(&self, ch: char) -> Result<()> {
        // todo avoid heap alloc
        let mut text = String::with_capacity(1);
        text.push(ch);
        let text = OsString::from(text).wchars();
        edit_session::insert_text(self.tid, self.context()?, &text)
    }
}

