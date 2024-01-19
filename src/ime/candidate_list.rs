use std::mem::{self, size_of};

use log::{trace, debug};
use windows::{Win32::{UI::WindowsAndMessaging::{CreateWindowExA, WS_POPUPWINDOW, WS_VISIBLE, WS_EX_LEFT, ShowWindow, SW_SHOW, SW_HIDE, GetClassInfoExA, WNDCLASSEXA, CS_NOCLOSE, RegisterClassExA, LoadCursorA, IDC_ARROW, LoadCursorW}, Foundation::{HWND, GetLastError, WPARAM, LPARAM, LRESULT}, Graphics::Gdi::{COLOR_MENU, HBRUSH}}, core::{s, PCSTR}};
use windows::core::Result;
use crate::DLL_MOUDLE;

#[derive(Default)]
pub struct CandidateList {
    hwnd: Option<HWND>
}

impl CandidateList {
    pub fn new() -> CandidateList {
        CandidateList::default()
    }

    pub fn locate(&mut self, parent: HWND, pos: (i32, i32)) -> Result<()>{
        trace!("locate({:?}, ({}, {}))", parent, pos.0, pos.1);
        unsafe {
            let mut wcex :WNDCLASSEXA = mem::zeroed();
            // GetClassInfoExA(DLL_MOUDLE.unwrap(), s!("AjemiCandidateList"), &mut wcex);
            wcex.cbSize = size_of::<WNDCLASSEXA>() as u32;
            wcex.style = CS_NOCLOSE;
            wcex.hCursor = LoadCursorW(None, IDC_ARROW)?;
            wcex.hInstance = DLL_MOUDLE.unwrap();
            wcex.hbrBackground = HBRUSH{0: COLOR_MENU.0 as isize};
            wcex.lpfnWndProc = None;
            wcex.lpszClassName = s!("AjemiCandidateList");

            if RegisterClassExA(&wcex) == 0 {
                return GetLastError();
            }
            debug!("Registered window class.");

            let hwnd = CreateWindowExA(
                WS_EX_LEFT, 
                s!("AjemiCandidateList"), 
                s!("AjemiCandidateList"),
                WS_POPUPWINDOW | WS_VISIBLE,
                pos.0, pos.1, 32, 32, 
                parent, None, DLL_MOUDLE.unwrap(),
                None);
            if hwnd.0 == 0 {
                return GetLastError();
            }
            debug!("Created window.");
            ShowWindow(hwnd, SW_SHOW);
            self.hwnd = Some(hwnd);
            Ok(())
        }
    }
    
    
    pub fn show(&self, text: &str) {
        // now all you need todo is find a UI framework that supports putting a floating panel 
        // at any given position
    }

    pub fn hide(&mut self) {
        let Some(hwnd) = self.hwnd else {
            return;
        };
        unsafe { ShowWindow(hwnd, SW_HIDE); }
        self.hwnd = None;
    }
}