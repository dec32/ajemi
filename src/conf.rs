use std::{env, fs, os::windows::fs::MetadataExt, path::PathBuf};
use toml::{Table, Value};
use crate::{extend::TableExt, ui::Color};
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
pub static mut PANEL_COLOR: Color = Color::white();
pub static mut HIGHTLIGHT_COLOR: Color = Color::white();

static mut LAST_MODIFIED: u64 = 0;

pub fn setup() {
    unsafe { use_default(); }
}

pub fn reload() {
    unsafe { use_customized(); }
}

unsafe fn use_default() {
    FONT = "sitelen seli kiwen juniko".to_string();
    FONT_SIZE = 20;
    VERTICAL = false;
    CANDI_COLOR = Color::gray(0);
    CANDI_HIGHLIGHTED_COLOR = Color::gray(0);
    INDEX_COLOR = Color::gray(160);
    CLIP_COLOR = Color::hex(0x0078D7);
    PANEL_COLOR = Color::gray(250);
    HIGHTLIGHT_COLOR = Color::rgb(232, 232, 255);
}

unsafe fn use_customized() -> Option<()> {
    let path = PathBuf::from(env::var("APPDATA").ok()?).join("Ajemi").join("conf.toml");
    let last_modified = fs::metadata(&path).ok()?.last_write_time();
    if last_modified == LAST_MODIFIED {
        return Some(());
    }
    use_default();
    let text = fs::read_to_string(path).ok()?;
    let mut table = text.parse::<Table>().ok()?;
    if let Some(Value::Table(color)) = table.get_mut("color") {
        color.give("candidate", &mut CANDI_COLOR);
        color.give("highlighted", &mut CANDI_HIGHLIGHTED_COLOR);
        color.give("index", &mut INDEX_COLOR);
        color.give("clip", &mut CLIP_COLOR);
        color.give("background", &mut PANEL_COLOR);
        color.give("highlight", &mut HIGHTLIGHT_COLOR);
    }

    if let Some(Value::Table(layout)) = table.get_mut("layout") {
        layout.give("vertical", &mut VERTICAL);
    }

    if let Some(Value::Table(font)) = table.get_mut("font") {
        font.give("family", &mut FONT);
        font.give("size", &mut FONT_SIZE);
    }
    Some(())
}