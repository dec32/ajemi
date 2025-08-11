pub mod candidate_list;
use windows::Win32::{
    Foundation::COLORREF,
    Graphics::Gdi::{CreateSolidBrush, HBRUSH},
};

trait Color {
    fn to_color_ref(self) -> COLORREF;
    fn to_hbrush(self) -> HBRUSH;
}

impl Color for u32 {
    fn to_color_ref(mut self) -> COLORREF {
        let b = (self % 0x100) as u8;
        self /= 0x100;
        let g = (self % 0x100) as u8;
        self /= 0x100;
        let r = (self % 0x100) as u8;
        COLORREF(b as u32 * 0x10000 + g as u32 * 0x100 + r as u32)
    }

    fn to_hbrush(self) -> HBRUSH {
        unsafe { CreateSolidBrush(self.to_color_ref()) }
    }
}
