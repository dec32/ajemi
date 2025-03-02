// Keyboard Indentifiers
// QWERTY
pub const US: u16 = 0x0409;
pub const CANADIAN_FRENCH: u32 = 0x00001009;
// DVORAK
pub const US_DVORAK: u32 = 0x0001_0409;
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
    Dvorak,
    Qwertz, 
    Azerty,
    Custom,
    CanadianFrench,
}

impl<'a> TryFrom<&'a str> for Layout {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Layout::*;
        match value {
            "QWERTY" => Ok(Qwerty),
            "AZERTY" => Ok(Azerty), 
            "DVORAK" => Ok(Dvorak),
            "QWERTZ" => Ok(Qwertz),
            "CUSTOM" => Ok(Custom),
            "QWERTY_CFR" => Ok(CanadianFrench),
            _ => Err(())
        }
    }
}

impl Layout {
    pub fn from_lang_id(lang_id: u32) -> Layout {
        use Layout::*;
        match lang_id {
            US_DVORAK => Dvorak,
            GERMAN | GERMAN_IBM | SWISS_FRENCH => Qwertz,
            FRENCH | BELGIAN_FRENCH | BELGIAN_FRENCH_COMMA | BELGIAN_FRENCH_PERIOD => Azerty,
            CANADIAN_FRENCH  => CanadianFrench,
            _ => if lang_id >> 28 == 0xA {
                Custom
            } else {
                Qwerty
            }
        }
    }

    #[allow(unused)]
    pub fn parse_keycode(key_code: u32, shift: bool, altgr: bool) -> Option<char> {
        todo!()
    }
}
