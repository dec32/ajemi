use windows::{core::Interface, Win32::{Foundation::COLORREF, Graphics::Gdi::{CreateSolidBrush, HBRUSH}}};
use windows::core::IntoParam;
use windows::core::Param;

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


// FIXME how is this not working?
// impl<T: From<Color> + Interface + Clone> IntoParam<T> for Color {
//     fn into_param(self) -> Param<T> {
//         Param::Owned(T::from(self.clone()))
//     }
// }


impl IntoParam<COLORREF> for Color {
    fn into_param(self) -> Param<COLORREF> {
        Param::Owned(self.into())
    }
}

impl IntoParam<HBRUSH> for Color {
    fn into_param(self) -> Param<HBRUSH> {
        Param::Owned(self.into())
    }
}