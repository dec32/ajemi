use std::{env, fs, path::PathBuf};
use toml::Table;
use crate::ui::Color;

static mut TABLE: Option<Table> = None;

unsafe fn _reload() -> Option<()>{
    let appdata = env::var("APPDATA").ok()?;
    let path = PathBuf::from(appdata).join("Ajemi").join("conf.toml");
    let text = fs::read_to_string(path).ok()?;
    TABLE = text.parse::<Table>().ok();
    None
}

#[allow(unused)]
pub fn reload() {
    unsafe { _reload() };
}

#[allow(unused)]
pub fn get_str(key: &str) -> Option<&str> {
    unsafe {
        TABLE.as_ref()?.get(key)?.as_str()
    }
}
#[allow(unused)]
pub fn get_i64(key: &str) -> Option<i64> {
    unsafe {
        TABLE.as_ref()?.get(key)?.as_integer()
    }
}
#[allow(unused)]
pub fn get_i32(key: &str) -> Option<i32> {
    Some(get_i64(key)? as i32)
} 
#[allow(unused)]
pub fn get_u32(key: &str) -> Option<u32> {
    Some(get_i64(key)? as u32)
} 
#[allow(unused)]
pub fn get_usize(key: &str) -> Option<usize> {
    Some(get_i64(key)? as usize)
} 
#[allow(unused)]
pub fn get_color(key: &str) -> Option<Color> {
    Some(Color::hex(get_u32(key)?))
}
