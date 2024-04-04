use std::{env, fs, os::windows::fs::MetadataExt, path::PathBuf};
use anyhow::Result;
use toml::{Table, Value};
use crate::{extend::TableExt, ui::Color, DEFAULT_CONF, IME_NAME};
// font
pub static mut FONT: String = String::new(); 
pub static mut FONT_SIZE: i32 = 0;
// layout
pub static mut VERTICAL: bool = false;
// color scheme
pub static mut CANDI_COLOR: Color = Color::white();
pub static mut CANDI_HIGHLIGHTED_COLOR: Color = Color::white();
pub static mut INDEX_COLOR: Color = Color::white();
pub static mut CLIP_COLOR: Color = Color::white();
pub static mut BKG_COLOR: Color = Color::white();
pub static mut HIGHTLIGHT_COLOR: Color = Color::white();
// behavior
pub static mut LONG_PI: bool = false;
pub static mut LONG_GLYPH: bool = false;
pub static mut CJK_SPACE: bool = false;

static mut LAST_MODIFIED: u64 = 0;

pub fn setup() {
    unsafe { let _ = use_default(); }
}

pub fn reload() {
    unsafe { let _ = use_customized(); }
}

unsafe fn use_default() -> Result<()>{
    use_conf(DEFAULT_CONF)
}

unsafe fn use_customized() -> Result<()> {
    let path = PathBuf::from(env::var("APPDATA")?).join(IME_NAME).join("conf.toml");
    let last_modified = fs::metadata(&path)?.last_write_time();
    if last_modified == LAST_MODIFIED {
        return Ok(());
    }
    let customized = fs::read_to_string(path)?;
    use_conf(DEFAULT_CONF)?;
    use_conf(&customized)?;
    LAST_MODIFIED = last_modified;
    Ok(())
}

unsafe fn use_conf(text: &str) -> Result<()>{
    let mut table = text.parse::<Table>()?;
    if let Some(Value::Table(color)) = table.get_mut("color") {
        color.give("candidate", &mut CANDI_COLOR);
        color.give("highlighted", &mut CANDI_HIGHLIGHTED_COLOR);
        color.give("index", &mut INDEX_COLOR);
        color.give("clip", &mut CLIP_COLOR);
        color.give("background", &mut BKG_COLOR);
        color.give("highlight", &mut HIGHTLIGHT_COLOR);
    }

    if let Some(Value::Table(layout)) = table.get_mut("layout") {
        layout.give("vertical", &mut VERTICAL);
    }

    if let Some(Value::Table(font)) = table.get_mut("font") {
        font.give("name", &mut FONT);
        font.give("size", &mut FONT_SIZE);
    }

    if let Some(Value::Table(behavior)) = table.get_mut("behavior") {
        behavior.give("long_pi", &mut LONG_PI);
        behavior.give("long_glyph", &mut LONG_GLYPH);
        behavior.give("cjk_space", &mut CJK_SPACE);
    }
    Ok(())
}