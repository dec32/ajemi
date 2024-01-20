use std::mem::size_of;

use log::{trace, debug, error};
use windows::{Win32::{UI::WindowsAndMessaging::{CreateWindowExA, WS_POPUPWINDOW, WS_VISIBLE, WS_EX_LEFT, ShowWindow, SW_SHOW, SW_HIDE, WNDCLASSEXA, CS_NOCLOSE, RegisterClassExA, IDC_ARROW, LoadCursorW, HICON}, Foundation::{HWND, GetLastError, WPARAM, LPARAM, LRESULT}, Graphics::Gdi::{COLOR_MENU, HBRUSH}}, core::{s, PCSTR}};
use windows::core::Result;

use crate::{global, dll_module, extend::ResultExt};

//----------------------------------------------------------------------------
//
//  To create a window you need to register the window class beforehand.
//
//----------------------------------------------------------------------------

const WINDOW_CLASS: PCSTR = s!("CandidateList");

pub fn setup() -> Result<()> {
    let wcex = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        style: CS_NOCLOSE,
        lpfnWndProc: Some(wind_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: dll_module(),
        hIcon: HICON::default(),
        hCursor: unsafe{ LoadCursorW(None, IDC_ARROW)? },
        hbrBackground: HBRUSH{0: COLOR_MENU.0 as isize},
        lpszMenuName: PCSTR::null(),
        lpszClassName: WINDOW_CLASS,
        hIconSm: HICON::default()
    };
    if unsafe{ RegisterClassExA(&wcex) } == 0 {
        return unsafe{ GetLastError() };
    }
    debug!("Registered window class for candidate list.");
    Ok(())
}

#[allow(unused)]
unsafe extern "system" fn wind_proc(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT {
    trace!("wind_proc");
    LRESULT::default()
}

//----------------------------------------------------------------------------
//
//  The implementation
//
//----------------------------------------------------------------------------

#[derive(Default)]
pub struct CandidateList {
    hwnd: Option<HWND>
}

impl CandidateList {
    pub fn new() -> CandidateList {
        CandidateList::default()
    }

    pub fn locate(&mut self, parent_window: HWND, pos: (i32, i32)) -> Result<()>{
        trace!("locate({:?}, ({}, {}))", parent_window, pos.0, pos.1);
        let hwnd = unsafe{ CreateWindowExA(
            WS_EX_LEFT, 
            WINDOW_CLASS, 
            s!("Invisible Title"),
            WS_POPUPWINDOW | WS_VISIBLE,
            pos.0, pos.1, 32, 32, 
            parent_window, None, global::dll_module(),
            None) };
        if hwnd.0 == 0 {
            error!("CreateWindowExA returned null.");
            return unsafe{ GetLastError().log_error() };
        }
        debug!("Created window.");
        unsafe{ ShowWindow(hwnd, SW_SHOW) };
        self.hwnd = Some(hwnd);
        Ok(())
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





