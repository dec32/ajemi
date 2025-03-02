use Layout::*;

// Keyboard Indentifiers
// QWERTY
pub const US: u16 = 0x0409;
pub const CANADIAN_FRENCH: u32 = 0x00001009;
// AZERTY
pub const FRENCH: u32 = 0x0000_040C;
pub const BELGIAN_FRENCH: u32 = 0x0000_080C;
pub const BELGIAN_FRENCH_COMMA: u32 = 0x0001_080C;
pub const BELGIAN_FRENCH_PERIOD: u32 = 0x0000_0813;
// QWERTZ
pub const GERMAN: u32 = 0x0000_0407;
pub const GERMAN_IBM: u32 = 0x0001_0407;
pub const SWISS_FRENCH: u32 = 0x0000_100C;

// keyboard layouts
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Qwerty,
    German, 
    French,
    CanadianFrench,
}

impl<'a> TryFrom<&'a str> for Layout {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Layout::*;
        match value {
            "QWERTY" => Ok(Qwerty),
            "FRENCH" => Ok(French), 
            "GERMAN" => Ok(German),
            "CANADIAN_FRENCH" => Ok(CanadianFrench),
            _ => Err(())
        }
    }
}

impl Layout {
    pub fn from_lang_id(lang_id: u32) -> Layout {
        use Layout::*;
        match lang_id {
            GERMAN | GERMAN_IBM | SWISS_FRENCH => German,
            FRENCH | BELGIAN_FRENCH | BELGIAN_FRENCH_COMMA | BELGIAN_FRENCH_PERIOD => French,
            CANADIAN_FRENCH  => CanadianFrench,
            _ => Qwerty,
        }
    }

    #[allow(unused)]
    pub fn parse_keycode(&self, key_code: u16, shift: bool, altgr: bool) -> Option<char> {
        fn offset(key_code: u16, from: u16 ) -> u8 {
            (key_code - from).try_into().unwrap()
        }
        fn add(ch: char, offset: u8) -> char {
            let char: u8 = ch.try_into().unwrap();
            let sum: u8 = char + offset;
            sum.try_into().unwrap()
        }

        let char = match (self, shift, altgr, key_code) {
            // letters
            (_, false, false, VK_A..=VK_Z) => add('a', offset(key_code, 0x41)),
            (_, true,  false, VK_A..=VK_Z) => add('A', offset(key_code, 0x41)),
            // numbers on number row
            (French, true,  false, VK_0..=VK_9) => ('0' as u8 + key_code as u8 - 0x30) as char,
            (_,      false, false, VK_0..=VK_9) => ('0' as u8 + key_code as u8 - 0x30) as char,
            // symbols on number row
            (French, false, false, VK_1) => '&',
            (French, false, false, VK_2) => 'é',
            (French, false, false, VK_3) => '"',
            (French, false, false, VK_4) => '\'',
            (French, false, false, VK_5) => '(',
            (French, false, false, VK_6) => '-',
            (French, false, false, VK_7) => 'è',
            (French, false, false, VK_8) => '_',
            (French, false, false, VK_9) => 'ç',
            (French, false, false, VK_0) => 'à',
            (French, false, true, VK_2) => '~',
            (French, false, true, VK_3) => '#',
            (French, false, true, VK_4) => '{',
            (French, false, true, VK_5) => '[',
            (French, false, true, VK_6) => '|',
            (French, false, true, VK_7) => '`',
            (French, false, true, VK_8) => '\\',
            (French, false, true, VK_9) => '^',
            (CanadianFrench, true, false, VK_2) => '"',
            (German, true, false, VK_2) => '"',
            (German, true, false, VK_3) => '§',
            (German, true, false, VK_6) => '&',
            (German, true, false, VK_7) => '/',
            (German, true, false, VK_8) => '&',
            (German, true, false, VK_9) => '/',
            (German, true, false, VK_0) => '=',
            (German, false, true, VK_2) => '²',
            (German, false, true, VK_3) => '³',
            (German, false, true, VK_7) => '{',
            (German, false, true, VK_8) => '[',
            (German, false, true, VK_9) => ']',
            (German, false, true, VK_0) => '}',
            (_, true, false, VK_1) => '!',
            (_, true, false, VK_2) => '@',
            (_, true, false, VK_3) => '#',
            (_, true, false, VK_4) => '$',
            (_, true, false, VK_5) => '%',
            (_, true, false, VK_6) => '^',
            (_, true, false, VK_7) => '&',
            (_, true, false, VK_8) => '*',
            (_, true, false, VK_9) => '(',
            (_, true, false, VK_0) => ')',
            // miscellaneous keys (canadian french)
            (CanadianFrench, false, false, VK_OEM_2) => 'é',
            (CanadianFrench, false, false, VK_OEM_3) => '`',
            (CanadianFrench, false, false, VK_OEM_4) => '^',
            (CanadianFrench, false, false, VK_OEM_5) => '<',
            (CanadianFrench, false, false, VK_OEM_6) => '¸',
            (CanadianFrench, false, false, VK_OEM_7) => '#',
            (CanadianFrench, true, false, VK_OEM_2) => 'É',
            (CanadianFrench, true, false, VK_OEM_3) => '`',
            (CanadianFrench, true, false, VK_OEM_4) => '^',
            (CanadianFrench, true, false, VK_OEM_5) => '>',
            (CanadianFrench, true, false, VK_OEM_6) => '¨',
            (CanadianFrench, true, false, VK_OEM_7) => '|',
            (CanadianFrench, true, false, VK_OEM_COMMA) => '\'',
            (CanadianFrench, false, true, VK_OEM_1) => '~',
            (CanadianFrench, false, true, VK_OEM_2) => '´',
            (CanadianFrench, false, true, VK_OEM_3) => '{',
            (CanadianFrench, false, true, VK_OEM_4) => '[',
            (CanadianFrench, false, true, VK_OEM_5) => '}',
            (CanadianFrench, false, true, VK_OEM_6) => ']',
            (CanadianFrench, false, true, VK_OEM_7) => '\\',
            // miscellaneous keys (french)
            (French, false, false, VK_OEM_1) => '$',
            (French, false, false, VK_OEM_2) => ':',
            (French, false, false, VK_OEM_3) => 'ù',
            (French, false, false, VK_OEM_4) => ')',
            (French, false, false, VK_OEM_5) => '*',
            (French, false, false, VK_OEM_6) => '^',
            (French, false, false, VK_OEM_7) => '²',
            (French, false, false, VK_OEM_8) => '!',
            (French, false, false, VK_OEM_PERIOD) => ';',
            (French, true, false, VK_OEM_1) => '£',
            (French, true, false, VK_OEM_2) => '/',
            (French, true, false, VK_OEM_3) => '%',
            (French, true, false, VK_OEM_4) => '°',
            (French, true, false, VK_OEM_5) => 'μ',
            (French, true, false, VK_OEM_6) => '¨',
            (French, false, false, VK_OEM_8) => '§',
            (French, true, false, VK_OEM_COMMA) => '?',
            (French, false, true, VK_OEM_1) => '¤',
            (French, false, true, VK_OEM_4) => ']',
            (French, false, true, VK_OEM_PLUS) => '}',
            // miscellaneous keys (german)
            (German, false, false, VK_OEM_1) => 'ü',
            (German, false, false, VK_OEM_2) => '#',
            (German, false, false, VK_OEM_3) => 'ö',
            (German, false, false, VK_OEM_4) => 'ß',
            (German, false, false, VK_OEM_5) => '^',
            (German, false, false, VK_OEM_6) => '´',
            (German, false, false, VK_OEM_7) => 'ä',
            (German, false, false, VK_OEM_8) => '!',
            (German, false, false, VK_OEM_PLUS) => '+',
            (German, true, false, VK_OEM_1) => 'Ü',
            (German, true, false, VK_OEM_2) => '\'',
            (German, true, false, VK_OEM_3) => 'Ö',
            (German, true, false, VK_OEM_4) => '?',
            (German, true, false, VK_OEM_5) => '°',
            (German, true, false, VK_OEM_6) => '`',
            (German, true, false, VK_OEM_7) => 'Ä',
            (German, true, false, VK_OEM_8) => '!',
            (German, true, false, VK_OEM_COMMA) => '?',
            (German, true, false, VK_OEM_PERIOD) => ':',
            (German, true, false, VK_OEM_PLUS) => '*',
            (German, false, true, VK_OEM_4) => '\\',
            // miscellaneous keys (qwerty)
            (_, false, false, VK_OEM_1) => ';',
            (_, false, false, VK_OEM_2) => '\'',
            (_, false, false, VK_OEM_3) => '`',
            (_, false, false, VK_OEM_4) => '[',
            (_, false, false, VK_OEM_5) => '\\',
            (_, false, false, VK_OEM_6) => ']',
            (_, false, false, VK_OEM_7) => '\'',
            (_, false, false, VK_OEM_MINUS) => '-',
            (_, false, false, VK_OEM_PLUS) => '=',
            (_, false, false, VK_OEM_COMMA) => ',',
            (_, false, false, VK_OEM_PERIOD) => '.',
            (_, true,  false, VK_OEM_1) => ':',
            (_, true,  false, VK_OEM_2) => '?',
            (_, true,  false, VK_OEM_3) => '~',
            (_, true,  false, VK_OEM_4) => '{',
            (_, true,  false, VK_OEM_5) => '|',
            (_, true,  false, VK_OEM_6) => ']',
            (_, true,  false, VK_OEM_7) => '"',
            (_, true,  false, VK_OEM_MINUS) => '_',
            (_, true,  false, VK_OEM_PLUS) => '+',
            (_, true,  false, VK_OEM_COMMA) => '<',
            (_, true,  false, VK_OEM_PERIOD) => '>',
            _ => return None
        };
        Some(char)
    }
}



const VK_0: u16 = 48;
const VK_1: u16 = 49;
const VK_2: u16 = 50;
const VK_3: u16 = 51;
const VK_4: u16 = 52;
const VK_5: u16 = 53;
const VK_6: u16 = 54;
const VK_7: u16 = 55;
const VK_8: u16 = 56;
const VK_9: u16 = 57;
const VK_A: u16 = 65;
const VK_ABNT_C1: u16 = 193;
const VK_ABNT_C2: u16 = 194;
const VK_ACCEPT: u16 = 30;
const VK_ADD: u16 = 107;
const VK_APPS: u16 = 93;
const VK_ATTN: u16 = 246;
const VK_B: u16 = 66;
const VK_BACK: u16 = 8;
const VK_BROWSER_BACK: u16 = 166;
const VK_BROWSER_FAVORITES: u16 = 171;
const VK_BROWSER_FORWARD: u16 = 167;
const VK_BROWSER_HOME: u16 = 172;
const VK_BROWSER_REFRESH: u16 = 168;
const VK_BROWSER_SEARCH: u16 = 170;
const VK_BROWSER_STOP: u16 = 169;
const VK_C: u16 = 67;
const VK_CANCEL: u16 = 3;
const VK_CAPITAL: u16 = 20;
const VK_CLEAR: u16 = 12;
const VK_CONTROL: u16 = 17;
const VK_CONVERT: u16 = 28;
const VK_CRSEL: u16 = 247;
const VK_D: u16 = 68;
const VK_DBE_ALPHANUMERIC: u16 = 240;
const VK_DBE_CODEINPUT: u16 = 250;
const VK_DBE_DBCSCHAR: u16 = 244;
const VK_DBE_DETERMINESTRING: u16 = 252;
const VK_DBE_ENTERDLGCONVERSIONMODE: u16 = 253;
const VK_DBE_ENTERIMECONFIGMODE: u16 = 248;
const VK_DBE_ENTERWORDREGISTERMODE: u16 = 247;
const VK_DBE_FLUSHSTRING: u16 = 249;
const VK_DBE_HIRAGANA: u16 = 242;
const VK_DBE_KATAKANA: u16 = 241;
const VK_DBE_NOCODEINPUT: u16 = 251;
const VK_DBE_NOROMAN: u16 = 246;
const VK_DBE_ROMAN: u16 = 245;
const VK_DBE_SBCSCHAR: u16 = 243;
const VK_DECIMAL: u16 = 110;
const VK_DELETE: u16 = 46;
const VK_DIVIDE: u16 = 111;
const VK_DOWN: u16 = 40;
const VK_E: u16 = 69;
const VK_END: u16 = 35;
const VK_EREOF: u16 = 249;
const VK_ESCAPE: u16 = 27;
const VK_EXECUTE: u16 = 43;
const VK_EXSEL: u16 = 248;
const VK_F: u16 = 70;
const VK_F1: u16 = 112;
const VK_F10: u16 = 121;
const VK_F11: u16 = 122;
const VK_F12: u16 = 123;
const VK_F13: u16 = 124;
const VK_F14: u16 = 125;
const VK_F15: u16 = 126;
const VK_F16: u16 = 127;
const VK_F17: u16 = 128;
const VK_F18: u16 = 129;
const VK_F19: u16 = 130;
const VK_F2: u16 = 113;
const VK_F20: u16 = 131;
const VK_F21: u16 = 132;
const VK_F22: u16 = 133;
const VK_F23: u16 = 134;
const VK_F24: u16 = 135;
const VK_F3: u16 = 114;
const VK_F4: u16 = 115;
const VK_F5: u16 = 116;
const VK_F6: u16 = 117;
const VK_F7: u16 = 118;
const VK_F8: u16 = 119;
const VK_F9: u16 = 120;
const VK_FINAL: u16 = 24;
const VK_G: u16 = 71;
const VK_GAMEPAD_A: u16 = 195;
const VK_GAMEPAD_B: u16 = 196;
const VK_GAMEPAD_DPAD_DOWN: u16 = 204;
const VK_GAMEPAD_DPAD_LEFT: u16 = 205;
const VK_GAMEPAD_DPAD_RIGHT: u16 = 206;
const VK_GAMEPAD_DPAD_UP: u16 = 203;
const VK_GAMEPAD_LEFT_SHOULDER: u16 = 200;
const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: u16 = 209;
const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: u16 = 212;
const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: u16 = 214;
const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: u16 = 213;
const VK_GAMEPAD_LEFT_THUMBSTICK_UP: u16 = 211;
const VK_GAMEPAD_LEFT_TRIGGER: u16 = 201;
const VK_GAMEPAD_MENU: u16 = 207;
const VK_GAMEPAD_RIGHT_SHOULDER: u16 = 199;
const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: u16 = 210;
const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: u16 = 216;
const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: u16 = 218;
const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: u16 = 217;
const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: u16 = 215;
const VK_GAMEPAD_RIGHT_TRIGGER: u16 = 202;
const VK_GAMEPAD_VIEW: u16 = 208;
const VK_GAMEPAD_X: u16 = 197;
const VK_GAMEPAD_Y: u16 = 198;
const VK_H: u16 = 72;
const VK_HANGEUL: u16 = 21;
const VK_HANGUL: u16 = 21;
const VK_HANJA: u16 = 25;
const VK_HELP: u16 = 47;
const VK_HOME: u16 = 36;
const VK_I: u16 = 73;
const VK_ICO_00: u16 = 228;
const VK_ICO_CLEAR: u16 = 230;
const VK_ICO_HELP: u16 = 227;
const VK_IME_OFF: u16 = 26;
const VK_IME_ON: u16 = 22;
const VK_INSERT: u16 = 45;
const VK_J: u16 = 74;
const VK_JUNJA: u16 = 23;
const VK_K: u16 = 75;
const VK_KANA: u16 = 21;
const VK_KANJI: u16 = 25;
const VK_L: u16 = 76;
const VK_LAUNCH_APP1: u16 = 182;
const VK_LAUNCH_APP2: u16 = 183;
const VK_LAUNCH_MAIL: u16 = 180;
const VK_LAUNCH_MEDIA_SELECT: u16 = 181;
const VK_LBUTTON: u16 = 1;
const VK_LCONTROL: u16 = 162;
const VK_LEFT: u16 = 37;
const VK_LMENU: u16 = 164;
const VK_LSHIFT: u16 = 160;
const VK_LWIN: u16 = 91;
const VK_M: u16 = 77;
const VK_MBUTTON: u16 = 4;
const VK_MEDIA_NEXT_TRACK: u16 = 176;
const VK_MEDIA_PLAY_PAUSE: u16 = 179;
const VK_MEDIA_PREV_TRACK: u16 = 177;
const VK_MEDIA_STOP: u16 = 178;
const VK_MENU: u16 = 18;
const VK_MODECHANGE: u16 = 31;
const VK_MULTIPLY: u16 = 106;
const VK_N: u16 = 78;
const VK_NAVIGATION_ACCEPT: u16 = 142;
const VK_NAVIGATION_CANCEL: u16 = 143;
const VK_NAVIGATION_DOWN: u16 = 139;
const VK_NAVIGATION_LEFT: u16 = 140;
const VK_NAVIGATION_MENU: u16 = 137;
const VK_NAVIGATION_RIGHT: u16 = 141;
const VK_NAVIGATION_UP: u16 = 138;
const VK_NAVIGATION_VIEW: u16 = 136;
const VK_NEXT: u16 = 34;
const VK_NONAME: u16 = 252;
const VK_NONCONVERT: u16 = 29;
const VK_NUMLOCK: u16 = 144;
const VK_NUMPAD0: u16 = 96;
const VK_NUMPAD1: u16 = 97;
const VK_NUMPAD2: u16 = 98;
const VK_NUMPAD3: u16 = 99;
const VK_NUMPAD4: u16 = 100;
const VK_NUMPAD5: u16 = 101;
const VK_NUMPAD6: u16 = 102;
const VK_NUMPAD7: u16 = 103;
const VK_NUMPAD8: u16 = 104;
const VK_NUMPAD9: u16 = 105;
const VK_O: u16 = 79;
const VK_OEM_1: u16 = 186;
const VK_OEM_102: u16 = 226;
const VK_OEM_2: u16 = 191;
const VK_OEM_3: u16 = 192;
const VK_OEM_4: u16 = 219;
const VK_OEM_5: u16 = 220;
const VK_OEM_6: u16 = 221;
const VK_OEM_7: u16 = 222;
const VK_OEM_8: u16 = 223;
const VK_OEM_ATTN: u16 = 240;
const VK_OEM_AUTO: u16 = 243;
const VK_OEM_AX: u16 = 225;
const VK_OEM_BACKTAB: u16 = 245;
const VK_OEM_CLEAR: u16 = 254;
const VK_OEM_COMMA: u16 = 188;
const VK_OEM_COPY: u16 = 242;
const VK_OEM_CUSEL: u16 = 239;
const VK_OEM_ENLW: u16 = 244;
const VK_OEM_FINISH: u16 = 241;
const VK_OEM_FJ_JISHO: u16 = 146;
const VK_OEM_FJ_LOYA: u16 = 149;
const VK_OEM_FJ_MASSHOU: u16 = 147;
const VK_OEM_FJ_ROYA: u16 = 150;
const VK_OEM_FJ_TOUROKU: u16 = 148;
const VK_OEM_JUMP: u16 = 234;
const VK_OEM_MINUS: u16 = 189;
const VK_OEM_NEC_EQUAL: u16 = 146;
const VK_OEM_PA1: u16 = 235;
const VK_OEM_PA2: u16 = 236;
const VK_OEM_PA3: u16 = 237;
const VK_OEM_PERIOD: u16 = 190;
const VK_OEM_PLUS: u16 = 187;
const VK_OEM_RESET: u16 = 233;
const VK_OEM_WSCTRL: u16 = 238;
const VK_P: u16 = 80;
const VK_PA1: u16 = 253;
const VK_PACKET: u16 = 231;
const VK_PAUSE: u16 = 19;
const VK_PLAY: u16 = 250;
const VK_PRINT: u16 = 42;
const VK_PRIOR: u16 = 33;
const VK_PROCESSKEY: u16 = 229;
const VK_Q: u16 = 81;
const VK_R: u16 = 82;
const VK_RBUTTON: u16 = 2;
const VK_RCONTROL: u16 = 163;
const VK_RETURN: u16 = 13;
const VK_RIGHT: u16 = 39;
const VK_RMENU: u16 = 165;
const VK_RSHIFT: u16 = 161;
const VK_RWIN: u16 = 92;
const VK_S: u16 = 83;
const VK_SCROLL: u16 = 145;
const VK_SELECT: u16 = 41;
const VK_SEPARATOR: u16 = 108;
const VK_SHIFT: u16 = 16;
const VK_SLEEP: u16 = 95;
const VK_SNAPSHOT: u16 = 44;
const VK_SPACE: u16 = 32;
const VK_SUBTRACT: u16 = 109;
const VK_T: u16 = 84;
const VK_TAB: u16 = 9;
const VK_U: u16 = 85;
const VK_UP: u16 = 38;
const VK_V: u16 = 86;
const VK_VOLUME_DOWN: u16 = 174;
const VK_VOLUME_MUTE: u16 = 173;
const VK_VOLUME_UP: u16 = 175;
const VK_W: u16 = 87;
const VK_X: u16 = 88;
const VK_XBUTTON1: u16 = 5;
const VK_XBUTTON2: u16 = 6;
const VK_Y: u16 = 89;
const VK_Z: u16 = 90;
const VK_ZOOM: u16 = 251;