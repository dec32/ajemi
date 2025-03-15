use std::{env, fs, path::PathBuf};
use serde::Deserialize;
use crate::{extend::ResultExt, Error, Result, DEFAULT_CONF, IME_NAME};

static mut CONF: Conf = Conf::new_const();

pub fn get() -> &'static Conf {
    unsafe { &CONF }
}

pub fn setup() {
    unsafe { CONF = Conf::open_or_default() }
}

pub fn reload() {

}

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub font: Font,
    pub layout: Layout,
    pub color: Color,
    pub behavior: Behavior
}

impl Default for Conf {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONF).unwrap()
    }
}

impl Conf {
    const fn new_const() -> Conf {
        Conf {
            font: Font { name: String::new(), size: 0 },
            layout: Layout { vertical: false },
            color: Color { candidate: 0, index: 0, background: 0, clip: 0, highlight: 0, highlighted: 0 },
            behavior: Behavior { long_pi: false, long_glyph: false, cjk_space: false },
        }
    }

    pub fn open() -> Result<Conf> {
        let path = PathBuf::from(env::var("APPDATA")?).join(IME_NAME).join("conf.toml");
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::write(path, DEFAULT_CONF)?;
            return Ok(Conf::default());
        }
        let conf = fs::read_to_string(path)?;
        let conf = toml::from_str(&conf).map_err(|e|Error::ParseError("conf.toml", e))?;
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
    pub highlighted: u32
}

#[derive(Deserialize, Debug)]
pub struct Layout {
    pub vertical: bool
}

#[derive(Deserialize, Debug)]
pub struct Behavior {
    pub long_pi: bool,
    pub long_glyph: bool,
    pub cjk_space: bool
}

#[test]
fn test_open() {
    let conf = get();
    println!("{conf:#?}")
}