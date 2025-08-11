use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{Error, IME_NAME, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Install {
    pub layout: Option<Layout>,
    pub langid: Option<u16>,
}

// keyboard layouts
#[derive(Serialize, Deserialize, EnumString, Display, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Qwerty,
    QwertyCFR,
    Azerty,
    Qwertz,
}

impl Install {
    fn path() -> Result<PathBuf> {
        Ok(PathBuf::from(env::var("APPDATA")?)
            .join(IME_NAME)
            .join("install.toml"))
    }

    pub fn open() -> Result<Self> {
        let info = fs::read_to_string(Self::path()?)?;
        let info =
            toml::from_str(info.as_str()).map_err(|err| Error::ParseError("install.toml", err))?;
        Ok(info)
    }

    pub fn save(&self) -> Result<()> {
        let info = toml::to_string(self).unwrap();
        fs::write(Self::path()?, info)?;
        Ok(())
    }
}

impl Layout {
    pub fn from_lang_id(lang_id: u32) -> Layout {
        use Layout::*;

        use crate::global::*;
        match lang_id {
            GERMAN | GERMAN_IBM | SWISS_FRENCH => Qwertz,
            FRENCH | BELGIAN_FRENCH | BELGIAN_FRENCH_COMMA | BELGIAN_FRENCH_PERIOD => Azerty,
            CANADIAN_FRENCH => QwertyCFR,
            _ => Qwerty,
        }
    }
}
