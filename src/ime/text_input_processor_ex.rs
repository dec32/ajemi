use windows::Win32::UI::TextServices::{ITfTextInputProcessorEx_Impl, ITfThreadMgr, ITfTextInputProcessor_Impl};
use windows::core::Result;
use crate::ime::Ime;

impl ITfTextInputProcessorEx_Impl for Ime {
    #[allow(non_snake_case)]
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> Result<()> {
        self.Activate(ptim, tid)
    }
}