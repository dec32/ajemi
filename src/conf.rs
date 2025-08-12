use std::{env, fs, path::PathBuf, sync::OnceLock};

use serde::Deserialize;

use crate::{DEFAULT_CONF, Error, IME_NAME, Result, extend::ResultExt};

// use parking_lot::{RwLock, RwLockReadGuard};
//
// static CONF: OnceLock<RwLock<Conf>> = OnceLock::new();
//
// pub fn get() -> RwLockReadGuard<'static, Conf> {
//     CONF2.get_or_init(||RwLock::new(Conf::open_or_default())).read_recursive()
// }
//
// pub fn reload() {
//     // todo check for last modified
//     let mut conf = CONF2.get().unwrap().write();
//     *conf = Conf::open_or_default();
// }

static CONF: OnceLock<Conf> = OnceLock::new();

pub fn get() -> &'static Conf {
    CONF.get_or_init(Conf::open_or_default)
}

pub fn reload() {}

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub font: Font,
    pub layout: Layout,
    pub color: Color,
    pub behavior: Behavior,
}

impl Default for Conf {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONF).unwrap()
    }
}

impl Conf {
    pub fn open() -> Result<Conf> {
        let path = PathBuf::from(env::var("APPDATA")?)
            .join(IME_NAME)
            .join("conf.toml");
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(path, DEFAULT_CONF)?;
            return Ok(Conf::default());
        }
        let conf = fs::read_to_string(path)?;
        let conf = toml::from_str(&conf).map_err(|e| Error::ParseError("conf.toml", e))?;
        Ok(conf)
    }

    pub fn open_or_default() -> Conf {
        Conf::open().inspect_err_with_log().unwrap_or_default()
    }
}

#[derive(Deserialize, Debug)]
pub struct Font {
    pub name: String,
    pub size: i32,
}

#[derive(Deserialize, Debug)]
pub struct Color {
    pub candidate: u32,
    pub index: u32,
    pub background: u32,
    pub clip: u32,
    pub highlight: u32,
    pub highlighted: u32,
}

#[derive(Deserialize, Debug)]
pub struct Layout {
    pub vertical: bool,
}

#[derive(Deserialize, Debug)]
pub struct Behavior {
    pub long_pi: bool,
    pub long_glyph: bool,
    pub cjk_space: bool,
    #[serde(default)]
    pub toggle: Toggle,
}

#[derive(Debug, Clone, Copy)]
pub enum Toggle {
    Eisu,
    Ctrl,
    CapsLock,
}

impl Default for Toggle {
    fn default() -> Self {
        Self::Eisu
    }
}

#[test]
fn test_open() {
    let conf = get();
    println!("{conf:#?}")
}
