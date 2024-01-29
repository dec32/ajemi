use std::{cmp::max, ffi::{CString, OsString}, mem::{self, size_of, ManuallyDrop}};
use log::{trace, debug, error};
use windows::{Win32::{UI::WindowsAndMessaging::{CreateWindowExA, DefWindowProcA, DestroyWindow, GetWindowLongPtrA, LoadCursorW, RegisterClassExA, SetWindowLongPtrA, SetWindowPos, ShowWindow, CS_HREDRAW, CS_IME, CS_VREDRAW, HICON, HWND_TOPMOST, IDC_ARROW, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_SHOWNOACTIVATE, WINDOW_LONG_PTR_INDEX, WM_PAINT, WNDCLASSEXA, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP}, Foundation::{GetLastError, BOOL, E_FAIL, HWND, LPARAM, LRESULT, RECT, SIZE, WPARAM}, Graphics::Gdi::{BeginPaint, CreateFontA, EndPaint, FillRect, GetDC, GetTextExtentPoint32W, InvalidateRect, SelectObject, SetBkMode, SetTextColor, TextOutW, HDC, HFONT, OUT_TT_PRECIS, PAINTSTRUCT, TRANSPARENT}}, core::{s, PCSTR}};
use windows::core::Result;
use crate::{engine::Suggestion, extend::OsStrExt2, global, ui::Color};

const TEXT_HEIGHT: i32 = 24;
const TEXT_COLOR: Color = Color::gray(0);
const TEXT_HIGHLIGHT_COLOR: Color = Color::gray(0);

const CLIP_WIDTH: i32 = 3;
const CLIP_COLOR: Color =  Color::rgb(0, 120, 215);

const LABEL_COLOR: Color = Color::gray(250);
const LABEL_HIGHTLIGHT_COLOR: Color = Color::gray(230);

const LABEL_PADDING_TOP: i32 = 4;
const LABEL_PADDING_BOTTOM: i32 = 4;
const LABEL_PADDING_LEFT: i32 = 2;
const LABEL_PADDING_RIGHT: i32 = 2;

const POS_OFFSETX: i32 = 2;
const POS_OFFSETY: i32 = 2;


const WINDOW_CLASS: PCSTR = s!("CANDIDATE_LIST");
static mut FONT: HFONT = unsafe { mem::zeroed() };

/// To create a window you need to register the window class beforehand.
pub fn setup() -> Result<()> {
    let wcex = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        style: CS_IME | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wind_proc),
        cbClsExtra: 0,
        cbWndExtra: size_of::<Box<PaintArg>>().try_into().unwrap(),
        hInstance: global::dll_module(),
        hIcon: HICON::default(),
        hCursor: unsafe{ LoadCursorW(None, IDC_ARROW)? },
        hbrBackground: Color::hex(0xFFFFFF).into(),
        lpszMenuName: PCSTR::null(),
        lpszClassName: WINDOW_CLASS,
        hIconSm: HICON::default()
    };

    unsafe {
        if RegisterClassExA(&wcex) == 0 {
            error!("Failed to register window class for candidate list");
            return GetLastError();
        }
        debug!("Registered window class for candidate list.");
        let font_name = CString::new(global::FONT).unwrap();
        let font_name = PCSTR::from_raw(font_name.as_bytes_with_nul().as_ptr());
        FONT = CreateFontA (
            TEXT_HEIGHT, 0, 0, 0, 
            0, 0, 0, 0, 
            0, 
            OUT_TT_PRECIS.0 as u32, 0, 0, 
            0, font_name);
        if FONT == mem::zeroed() {
            error!("Failed to create font.");
            return GetLastError();
        }
        debug!("Created font.");
    }
    Ok(())
}

// use default handlers for everything but repaint
unsafe extern "system" fn wind_proc(window: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => paint(window),
        _  => DefWindowProcA(window, msg, wparam, lparam)
    }
}


//----------------------------------------------------------------------------
//
//  The implementation
//
//----------------------------------------------------------------------------

#[derive(Default)]
pub struct CandidateList {
    window: HWND,
}

impl CandidateList {
    pub fn create(parent_window: HWND) -> Result<CandidateList> {
        // WS_EX_TOOLWINDOW: A floating toolbar that won't appear in taskbar and ALT+TAB.
        // WS_EX_NOACTIVATE: A window that doesn't take the foreground thus not making parent window lose focus.
        // WS_EX_TOPMOST:    A window that is topmost.
        // WS_POPUP:         A window having no top bar or border.
        // see: https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles
        let window = unsafe{ CreateWindowExA(
            WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TOPMOST, 
            WINDOW_CLASS, 
            PCSTR::null(),
            WS_POPUP,
            0, 0, 0, 0, 
            parent_window, None, global::dll_module(),
            None) };
        if window.0 == 0 {
            error!("CreateWindowExA returned null.");
            return match unsafe{ GetLastError() } {
                Ok(_) => Err(E_FAIL.into()),
                Err(e) => Err(e)
            };
        }
        Ok(CandidateList{ window })
    }

    pub fn locate(&self, x: i32, y: i32) -> Result<()>{
        trace!("locate({x}, {y})");
        unsafe {SetWindowPos(
            self.window, HWND_TOPMOST, 
            x + POS_OFFSETX, y + POS_OFFSETY, 0, 0,
            SWP_NOACTIVATE | SWP_NOSIZE)? };

        Ok(())
    }

    pub fn show(&self, suggs: &Vec<Suggestion>) -> Result<()> {
        unsafe{ 
            let mut text_size = SIZE::default();
            let mut text_list = Vec::with_capacity(suggs.len());
            let dc: HDC = GetDC(self.window);
            SelectObject(dc, FONT);
            for (index, sugg) in suggs.iter().enumerate() {
                let index = index + 1;
                let text = &sugg.output;
                // indexes are not guaranteed to be the same width but whatever
                let text = OsString::from(format!("{index}. {text}")).wchars();
                let mut size = SIZE::default();
                GetTextExtentPoint32W(dc,&text, &mut size);
                text_size.cy = size.cy;
                text_size.cx = max(text_size.cx, size.cx);
                text_list.push(text);
            }

            let candidates: i32 = suggs.len().try_into().unwrap();
            let wnd_size = SIZE {
                cx: CLIP_WIDTH + LABEL_PADDING_LEFT + text_size.cx + LABEL_PADDING_RIGHT,
                cy: candidates * (LABEL_PADDING_TOP + text_size.cy + LABEL_PADDING_BOTTOM)
            };
            // passing extra args to WndProc
            let arg = PaintArg {wnd_size, text_size, text_list}.to_long_ptr();
            SetWindowLongPtrA(self.window, WINDOW_LONG_PTR_INDEX::default(), arg);
            // resize and show
            SetWindowPos(
                self.window, HWND_TOPMOST, 
                0, 0, wnd_size.cx, wnd_size.cy,
                SWP_NOACTIVATE | SWP_NOMOVE)?;
            ShowWindow(self.window, SW_SHOWNOACTIVATE);
            // force repaint
            InvalidateRect(self.window, None, BOOL::from(true));
        };
        Ok(())
    }

    pub fn hide(&self) {
        unsafe { ShowWindow(self.window, SW_HIDE); }
    }

    pub fn destroy(&self) -> Result<()> {
        unsafe { DestroyWindow(self.window) }
    }
}

struct PaintArg {
    wnd_size: SIZE,
    text_size: SIZE,
    text_list: Vec<Vec<u16>>,
}
impl PaintArg {
    unsafe fn to_long_ptr(self) -> isize{
        // TODO make sure there's no memory leak
        mem::transmute(ManuallyDrop::new(Box::new(self)))
    }
    unsafe fn from_long_ptr(long_ptr: isize) -> Option<Box<PaintArg>>{
        if long_ptr == 0 {
            None
        } else {
            Some(mem::transmute(long_ptr))
        }
    }
}

unsafe fn paint(window: HWND) -> LRESULT{
    // load the extra arg
    let Some(arg) = PaintArg::from_long_ptr(GetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::default())) else {
        error!("Args for repaint is not found.");
        return LRESULT::default();
    };
    SetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::default(), 0);
    // specify the area to repainted (in this case, the whole window)
    let mut ps = PAINTSTRUCT::default();
    let dc: HDC = BeginPaint(window, &mut ps);
    if dc.is_invalid() {
        error!("BeginPaint failed.");
        return LRESULT::default();
    }
    // paint
    let label_size = SIZE{
        cx: LABEL_PADDING_LEFT + arg.text_size.cx + LABEL_PADDING_RIGHT,
        cy: LABEL_PADDING_TOP + arg.text_size.cy + LABEL_PADDING_BOTTOM,
    };
    let wnd = RECT{ left: 0, top: 0, right: arg.wnd_size.cx, bottom: arg.wnd_size.cy};
    let clip = RECT{ left: 0, top: 0, right: CLIP_WIDTH, bottom: label_size.cy };
    let label_highlight = RECT{ left: 0, top: 0, right: arg.wnd_size.cx, bottom: arg.wnd_size.cy};
    FillRect(dc, &wnd, LABEL_COLOR);
    FillRect(dc, &label_highlight, LABEL_HIGHTLIGHT_COLOR);
    FillRect(dc, &clip, CLIP_COLOR);
    // text
    let text_x = CLIP_WIDTH + LABEL_PADDING_LEFT;
    let mut text_y = LABEL_PADDING_TOP;

    SelectObject(dc, FONT);
    SetBkMode(dc, TRANSPARENT);
    SetTextColor(dc, TEXT_HIGHLIGHT_COLOR);
    TextOutW(dc, text_x, text_y, &arg.text_list[0]);

    SetTextColor(dc, TEXT_COLOR);
    for text in arg.text_list[1..].iter() {
        text_y += label_size.cy;
        TextOutW(dc, text_x, text_y, text);
    }

    EndPaint(window, &mut ps);
    LRESULT::default()
}



