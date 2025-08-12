use std::{
    cmp::max,
    ffi::{CString, OsString},
    mem::{ManuallyDrop, size_of},
};

use log::{debug, error, trace};
use windows::{
    Win32::{
        Foundation::{BOOL, GetLastError, HWND, LPARAM, LRESULT, RECT, SIZE, WPARAM},
        Graphics::Gdi::{
            self, BeginPaint, CreateFontA, EndPaint, GetDC, GetDeviceCaps, GetTextExtentPoint32W,
            HDC, HFONT, InvalidateRect, LOGPIXELSY, OUT_TT_PRECIS, PAINTSTRUCT, ReleaseDC,
            SelectObject, SetBkMode, SetTextColor, TRANSPARENT, TextOutW,
        },
        UI::WindowsAndMessaging::{
            CS_DROPSHADOW, CS_HREDRAW, CS_IME, CS_VREDRAW, CreateWindowExA, DefWindowProcA,
            DestroyWindow, GetWindowLongPtrA, HICON, HWND_TOPMOST, IDC_ARROW, LoadCursorW,
            RegisterClassExA, SW_HIDE, SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
            SetWindowLongPtrA, SetWindowPos, ShowWindow, WINDOW_LONG_PTR_INDEX, WM_PAINT,
            WNDCLASSEXA, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
        },
    },
    core::{PCSTR, Result, s},
};

use super::Color;
use crate::{
    CANDI_INDEX_SUFFIX, CANDI_INDEX_SUFFIX_MONO, CANDI_INDEXES,
    conf::{self},
    engine::Suggestion,
    extend::OsStrExt2,
    global,
};

const WINDOW_CLASS: PCSTR = s!("CANDIDATE_LIST");
// Layout
const CLIP_WIDTH: i32 = 3;
const LABEL_PADDING_TOP: i32 = 2;
const LABEL_PADDING_BOTTOM: i32 = 2;
const LABEL_PADDING_LEFT: i32 = 3;
const LABEL_PADDING_RIGHT: i32 = 4;
const BORDER_WIDTH: i32 = 0;

const POS_OFFSETX: i32 = 2;
const POS_OFFSETY: i32 = 2;

#[cfg(target_pointer_width = "64")]
type LongPointer = isize;
#[cfg(target_pointer_width = "32")]
type LongPointer = i32;

/// To create a window you need to register the window class beforehand.
pub fn setup() -> Result<()> {
    let wcex = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        style: CS_IME | CS_HREDRAW | CS_VREDRAW | CS_DROPSHADOW,
        lpfnWndProc: Some(wind_proc),
        cbClsExtra: 0,
        cbWndExtra: size_of::<Box<PaintArg>>().try_into().unwrap(),
        hInstance: global::dll_module(),
        hIcon: HICON::default(),
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
        hbrBackground: 0xFFFFFFu32.to_hbrush(),
        lpszMenuName: PCSTR::null(),
        lpszClassName: WINDOW_CLASS,
        hIconSm: HICON::default(),
    };
    unsafe {
        if RegisterClassExA(&wcex) == 0 {
            error!("Failed to register window class for candidate list");
            return Err(GetLastError().into());
        }
        debug!("Registered window class for candidate list.");
    }
    Ok(())
}

/// use default handlers for everything but repaint
unsafe extern "system" fn wind_proc(
    window: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => paint(window),
        _ => unsafe { DefWindowProcA(window, msg, wparam, lparam) },
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
    candi_font: HFONT,
    index_font: HFONT,
    index_suffix: &'static str,
}

impl CandidateList {
    pub fn create(_parent_window: HWND) -> Result<CandidateList> {
        // WS_EX_TOOLWINDOW:  A floating toolbar that won't appear in taskbar and ALT+TAB.
        // WS_EX_NOACTIVATE:  A window that doesn't take the foreground thus not making parent window lose focus.
        // WS_EX_TOPMOST:     A window that is topmost.
        // WS_POPUP:          A window having no top bar or border.
        // see: https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles
        unsafe {
            let conf = conf::get();
            let window = CreateWindowExA(
                WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TOPMOST,
                WINDOW_CLASS,
                PCSTR::null(),
                WS_POPUP,
                0,
                0,
                0,
                0,
                None,
                None,
                global::dll_module(),
                None,
            );
            if window.0 == 0 {
                error!("CreateWindowExA returned null.");
                return Err(GetLastError().into());
            }
            let dc: HDC = GetDC(window);
            let pixel_per_inch = GetDeviceCaps(dc, LOGPIXELSY);
            let font_size = conf.font.size * pixel_per_inch / 72;
            let font_name = CString::new(conf.font.name.as_str()).unwrap();
            let font_name = PCSTR::from_raw(font_name.as_bytes_with_nul().as_ptr());
            let candi_font = CreateFontA(
                font_size,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                OUT_TT_PRECIS.0 as u32,
                0,
                0,
                0,
                font_name,
            );
            if candi_font.is_invalid() {
                error!("CreateFontA failed.");
                return Err(GetLastError().into());
            }

            let font_size = font_size * 70 / 100;
            let mut index_font = CreateFontA(
                font_size,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                OUT_TT_PRECIS.0 as u32,
                0,
                0,
                0,
                font_name,
            );
            if index_font.is_invalid() {
                index_font = candi_font;
            }

            // TODO this is no reliable at all
            let font_name = conf.font.name.to_ascii_lowercase();
            let index_suffix = if font_name.contains("mono") || font_name.contains("fairfax") {
                CANDI_INDEX_SUFFIX_MONO
            } else {
                CANDI_INDEX_SUFFIX
            };
            ReleaseDC(window, dc);
            Ok(CandidateList {
                window,
                candi_font,
                index_font,
                index_suffix,
            })
        }
    }

    pub fn locate(&self, x: i32, y: i32) -> Result<()> {
        trace!("locate({x}, {y})");
        unsafe {
            SetWindowPos(
                self.window,
                HWND_TOPMOST,
                x + POS_OFFSETX,
                y + POS_OFFSETY,
                0,
                0,
                SWP_NOACTIVATE | SWP_NOSIZE,
            )?
        };

        Ok(())
    }

    pub fn show(&self, suggs: &[Suggestion]) -> Result<()> {
        unsafe {
            let conf = conf::get();
            let mut indice = Vec::with_capacity(suggs.len());
            let mut candis = Vec::with_capacity(suggs.len());

            let mut candi_height: i32 = 0;
            let mut index_height: i32 = 0;
            let mut index_width: i32 = 0;
            let mut candi_width: i32 = 0;
            let mut candi_widths = Vec::with_capacity(suggs.len());

            let dc: HDC = GetDC(self.window);
            for (index, sugg) in suggs.iter().enumerate() {
                let mut size = SIZE::default();
                let index = format!("{}{}", CANDI_INDEXES[index], self.index_suffix);
                let index = OsString::from(index).wchars();
                SelectObject(dc, self.index_font);
                GetTextExtentPoint32W(dc, &index, &mut size);
                index_height = max(index_height, size.cy);
                index_width = max(index_width, size.cx);
                indice.push(index);

                let candi = OsString::from(&sugg.output).wchars();
                SelectObject(dc, self.candi_font);
                GetTextExtentPoint32W(dc, &candi, &mut size);
                candi_height = max(candi_height, size.cy);
                candi_width = max(candi_width, size.cx);
                candi_widths.push(size.cx);
                candis.push(candi);
            }
            ReleaseDC(self.window, dc);
            let row_height = max(candi_height, index_height);
            let label_height = LABEL_PADDING_TOP + row_height + LABEL_PADDING_BOTTOM;
            let mut wnd_height = 0;
            let mut wnd_width = 0;
            if conf.layout.vertical {
                let candi_num: i32 = suggs.len().try_into().unwrap();
                wnd_height += candi_num * label_height;
                wnd_width += CLIP_WIDTH
                    + LABEL_PADDING_LEFT
                    + index_width
                    + candi_width
                    + LABEL_PADDING_RIGHT;
                wnd_width = max(wnd_width, wnd_height * 4 / 5)
            } else {
                wnd_height += label_height;
                wnd_width += CLIP_WIDTH;
                for candi_width in candi_widths.iter() {
                    wnd_width += LABEL_PADDING_LEFT + LABEL_PADDING_RIGHT;
                    wnd_width += index_width;
                    wnd_width += candi_width;
                }
            }
            wnd_height += BORDER_WIDTH * 2;
            wnd_width += BORDER_WIDTH * 2;

            let highlight_width = if conf.layout.vertical {
                wnd_width - CLIP_WIDTH - BORDER_WIDTH * 2
            } else {
                LABEL_PADDING_LEFT + index_width + candi_widths[0] + LABEL_PADDING_RIGHT
            };

            // passing extra args to WndProc
            let arg = PaintArg {
                wnd_width,
                wnd_height,
                highlight_width,
                label_height,
                row_height,
                index_width,
                index_height,
                candi_widths: candi_widths.clone(),
                candi_height,
                candis,
                indice,
                index_font: self.index_font,
                candi_font: self.candi_font,
            };
            let long_ptr = arg.into_long_ptr();
            SetWindowLongPtrA(self.window, WINDOW_LONG_PTR_INDEX::default(), long_ptr);
            // resize and show
            SetWindowPos(
                self.window,
                HWND_TOPMOST,
                0,
                0,
                wnd_width,
                wnd_height,
                SWP_NOACTIVATE | SWP_NOMOVE,
            )?;
            ShowWindow(self.window, SW_SHOWNOACTIVATE);
            // force repaint
            InvalidateRect(self.window, None, BOOL::from(true));
        };
        Ok(())
    }

    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.window, SW_HIDE);
        }
    }

    pub fn destroy(&self) -> Result<()> {
        unsafe { DestroyWindow(self.window) }
    }
}

struct PaintArg {
    wnd_width: i32,
    wnd_height: i32,
    highlight_width: i32,
    label_height: i32,
    row_height: i32,
    index_width: i32,
    index_height: i32,
    candi_widths: Vec<i32>,
    candi_height: i32,
    index_font: HFONT,
    candi_font: HFONT,
    indice: Vec<Vec<u16>>,
    candis: Vec<Vec<u16>>,
}
impl PaintArg {
    fn into_long_ptr(self) -> LongPointer {
        ManuallyDrop::new(Box::new(self)).as_ref() as *const PaintArg as LongPointer
    }
    unsafe fn from_long_ptr(long_ptr: LongPointer) -> Option<Box<PaintArg>> {
        if long_ptr == 0 {
            None
        } else {
            Some(unsafe { Box::from_raw(long_ptr as *mut PaintArg) })
        }
    }
}
fn paint(window: HWND) -> LRESULT {
    let conf = conf::get();
    // load the extra arg
    let arg = unsafe {
        PaintArg::from_long_ptr(GetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::default()))
    };
    let Some(arg) = arg else {
        error!("Args for repaint is not found.");
        return LRESULT::default();
    };
    unsafe { SetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::default(), 0) };
    let mut ps = PAINTSTRUCT::default();
    let dc: HDC = unsafe { BeginPaint(window, &mut ps) };
    if dc.is_invalid() {
        error!("BeginPaint failed.");
        return LRESULT::default();
    }
    unsafe {
        // window
        FillRect(
            dc,
            0,
            0,
            arg.wnd_width,
            arg.wnd_height,
            conf.color.background,
        );
        // clip
        FillRect(
            dc,
            BORDER_WIDTH,
            BORDER_WIDTH,
            CLIP_WIDTH,
            arg.label_height,
            conf.color.clip,
        );
        // highlight
        FillRect(
            dc,
            BORDER_WIDTH + CLIP_WIDTH,
            BORDER_WIDTH,
            arg.highlight_width,
            arg.label_height,
            conf.color.highlight,
        );
    }

    // highlighted text
    let mut index_x = BORDER_WIDTH + CLIP_WIDTH + LABEL_PADDING_LEFT;
    let mut candi_x = BORDER_WIDTH + index_x + arg.index_width;
    let mut index_y = BORDER_WIDTH + LABEL_PADDING_TOP + (arg.row_height - arg.index_height) / 2;
    let mut candi_y = BORDER_WIDTH + LABEL_PADDING_TOP + (arg.row_height - arg.candi_height) / 2;
    unsafe {
        SetBkMode(dc, TRANSPARENT);
        TextOut(
            dc,
            index_x,
            index_y,
            &arg.indice[0],
            conf.color.index,
            arg.index_font,
        );
        TextOut(
            dc,
            candi_x,
            candi_y,
            &arg.candis[0],
            conf.color.highlighted,
            arg.candi_font,
        );
    }
    // normal text
    for i in 1..arg.candis.len() {
        if conf.layout.vertical {
            index_y += arg.label_height;
            candi_y += arg.label_height;
        } else {
            index_x += arg.index_width
                + arg.candi_widths[i - 1]
                + LABEL_PADDING_LEFT
                + LABEL_PADDING_RIGHT;
            candi_x += arg.index_width
                + arg.candi_widths[i - 1]
                + LABEL_PADDING_LEFT
                + LABEL_PADDING_RIGHT;
        }
        unsafe {
            TextOut(
                dc,
                index_x,
                index_y,
                &arg.indice[i],
                conf.color.index,
                arg.index_font,
            );
            TextOut(
                dc,
                candi_x,
                candi_y,
                &arg.candis[i],
                conf.color.candidate,
                arg.candi_font,
            );
        }
    }
    unsafe {
        ReleaseDC(window, dc);
        EndPaint(window, &ps);
    }
    LRESULT::default()
}

#[allow(non_snake_case)]
unsafe fn TextOut(hdc: HDC, x: i32, y: i32, wchars: &[u16], color: u32, font: HFONT) {
    unsafe {
        SelectObject(hdc, font);
        SetTextColor(hdc, color.to_color_ref());
        TextOutW(hdc, x, y, wchars);
    }
}

#[allow(non_snake_case)]
unsafe fn FillRect(hdc: HDC, x: i32, y: i32, width: i32, height: i32, color: u32) {
    let rect = RECT {
        left: x,
        top: y,
        right: x + width,
        bottom: height,
    };
    unsafe { Gdi::FillRect(hdc, &rect, color.to_hbrush()) };
}
