use windows::Win32::UI::TextServices::{ITfTextInputProcessorEx_Impl, ITfThreadMgr, ITfTextInputProcessor_Impl};
use windows::core::Result;

use crate::ime::IME;


impl ITfTextInputProcessorEx_Impl for IME {
    #[allow(non_snake_case)]
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> Result<()> {
        todo!()
    }
}


impl ITfTextInputProcessor_Impl for IME {
    #[allow(non_snake_case)]
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        todo!()
    }

    #[allow(non_snake_case)]
    fn Deactivate(&self) -> Result<()> {
        todo!()
    }
}