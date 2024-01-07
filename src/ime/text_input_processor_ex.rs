use std::ptr;

use windows::Win32::UI::TextServices::{ITfTextInputProcessorEx_Impl, ITfThreadMgr, ITfTextInputProcessor_Impl};
use windows::core::Result;

use crate::ime::Ime;
use crate::debug;

// when activated, the thread manager and the client ID will be passed in.
// hold them until deactivated for later use.
// thread manager is an essential component for many tasks.
impl ITfTextInputProcessor_Impl for Ime {
    #[allow(non_snake_case)]
    fn Activate(&self, thread_mgr: Option<&ITfThreadMgr>, client_id: u32) -> Result<()> {
        // self.thread_mgr = thread_mgr.map(|it|it as *const ITfThreadMgr).unwrap_or(ptr::null());
        // self.client_id = Some(client_id);
        debug("<ITfTextInputProcessor_Impl> Activate");
        Ok(())
    }

    #[allow(non_snake_case)]
    fn Deactivate(&self) -> Result<()> {
        // self.thread_mgr = ptr::null();
        // self.client_id = None;
        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for Ime {
    #[allow(non_snake_case)]
    fn ActivateEx(&self, ptim: Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> Result<()> {
        self.Activate(ptim, tid)
    }
}