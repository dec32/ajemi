pub mod candidate_list;

use toml::Value;
use windows::Win32::{Foundation::COLORREF, Graphics::Gdi::{CreateSolidBrush, HBRUSH}};
use windows::core::IntoParam;
use windows::core::Param;

use crate::extend::LoadValue;

#[derive(Default, Clone, Copy)]
pub struct Color{
    r:u8, g:u8, b:u8
}

impl Color {
    pub const fn rgb(r:u8, g:u8, b:u8) -> Color {
        Color{r, g, b}
    }

    pub const fn hex(mut hex: u32) -> Color {
        let b = (hex % 0x100) as u8;
        hex /= 0x100;
        let g = (hex % 0x100) as u8;
        hex /= 0x100;
        let r = (hex % 0x100) as u8;
        Color::rgb(r, g, b)
    }

    pub const fn gray(gray: u8) -> Color {
        Color::rgb(gray, gray, gray)
    }

    pub const fn white() -> Color {
        Color::hex(0xFFFFFF)
    }
}

impl LoadValue for Color {
    fn load(&mut self, value: Value) {
        if let Value::Integer(value) = value {
            *self = Color::hex(value as u32);
        }
    }
}
impl TryFrom<Value> for Color {
    type Error = ();
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Integer(value) = value {
            Ok(Color::hex(value as u32))
        } else {
            Err(())
        }
    }
}

impl From<Color> for COLORREF {
    fn from(color: Color) -> Self {
        COLORREF{0: color.b as u32 * 0x10000 + color.g as u32 * 0x100 + color.r as u32}
    }
}

impl From<Color> for HBRUSH {
    fn from(color: Color) -> Self {
        unsafe{ CreateSolidBrush(COLORREF::from(color)) }
    }
}

impl IntoParam<COLORREF> for Color {
    unsafe fn into_param(self) -> Param<COLORREF> {
        Param::Owned(self.into())
    }
}

impl IntoParam<HBRUSH> for Color {
    unsafe fn into_param(self) -> Param<HBRUSH> {
        Param::Owned(self.into())
    }
}

