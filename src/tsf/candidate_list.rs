use std::{mem::{size_of, self}, ffi::OsString, os::windows::ffi::OsStrExt};

use log::{trace, debug, error, warn};
use windows::{Win32::{UI::{TextServices::ITfThreadMgr, WindowsAndMessaging::{CreateWindowExA, WS_POPUPWINDOW, WS_VISIBLE, ShowWindow, SW_HIDE, WNDCLASSEXA, RegisterClassExA, IDC_ARROW, LoadCursorW, HICON, DefWindowProcA, CS_IME, CS_DBLCLKS, CS_HREDRAW, CS_VREDRAW, WS_EX_TOOLWINDOW, WS_EX_NOACTIVATE, WS_EX_TOPMOST, SW_SHOWNOACTIVATE, SetWindowPos, SWP_NOACTIVATE, HWND_TOPMOST, WS_CHILD, SetWindowTextW, SendMessageA, WM_SETFONT, SWP_NOMOVE, SWP_NOSIZE}}, Foundation::{HWND, GetLastError, WPARAM, LPARAM, LRESULT, E_FAIL, SIZE}, Graphics::Gdi::{COLOR_MENU, HBRUSH, CreateFontA, OUT_TT_PRECIS, HFONT, HDC, GetDC, SelectObject, GetTextExtentPoint32W}}, core::{s, PCSTR, Error, HSTRING}};
use windows::core::Result;

use crate::dll_module;

const FONT_SIZE: i32 = 24;

const PADDING_TOP: i32 = 0;
const PADDING_RIGHT: i32 = 2;
const PADDING_LEFT: i32 = 0;
const PADDING_BOTTOM: i32 = 2;

const POS_OFFSETX: i32 = 2;
const POS_OFFSETY: i32 = 2;

const WINDOW_CLASS: PCSTR = s!("CANDIDATE_LIST");
static mut FONT: HFONT = unsafe { mem::zeroed() };
static mut DC: HDC = unsafe { mem::zeroed() };

/// To create a window you need to register the window class beforehand.
pub fn setup() -> Result<()> {
    let wcex = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        // TODO 这些 Style 是幹嘛的
        style: CS_IME | CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW,
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

    unsafe {
        if RegisterClassExA(&wcex) == 0 {
            error!("Failed to register window class for candidate list");
            return GetLastError();
        }
        debug!("Registered window class for candidate list.");
        FONT = CreateFontA (
            FONT_SIZE, 0, 0, 0, 
            0, 0, 0, 0, 
            0, 
            OUT_TT_PRECIS.0 as u32, 0, 0, 
            0, s!("linja waso lili"));
        if FONT == mem::zeroed() {
            error!("Failed to create font.");
            return GetLastError();
        }
        DC = GetDC(None);
        SelectObject(DC, FONT);
        debug!("Created font.");
    }
    Ok(())
}

#[allow(unused)]
unsafe extern "system" fn wind_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // trace!("wind_proc({:?}, {}, {:?}, {:?})", hwnd, wm(msg), wparam, lparam);
    DefWindowProcA(hwnd, msg, wparam, lparam)
}

//----------------------------------------------------------------------------
//
//  The implementation
//
//----------------------------------------------------------------------------

#[derive(Default)]
pub struct CandidateList {
    window: HWND,
    label: HWND,
}

impl CandidateList {
    pub fn create(thread_mgr: &ITfThreadMgr) -> Result<CandidateList> {
        // WS_EX_TOOLWINDOW: A floating toolbar that won't appear in taskbar and ALT+TAB.
        // WS_EX_NOACTIVATE: A window that doesn't take the foreground thus not making parent window losing focus.
        // WS_EX_TOPMOST: A window that is topmost.
        // WS_POPUPWINDOW: A window having not top bar.
        // see: https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles

        let parent_window = unsafe {thread_mgr.GetFocus()?.GetTop()?.GetActiveView()?.GetWnd()}?;
        let window = unsafe{ CreateWindowExA(
            WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_TOPMOST, 
            WINDOW_CLASS, 
            PCSTR::null(),
            WS_POPUPWINDOW,
            0, 0, 0, 0, 
            parent_window, None, dll_module(),
            None) };
        if window.0 == 0 {
            error!("CreateWindowExA returned null.");
            return match unsafe{ GetLastError() } {
                Ok(_) => Err(Error::from(E_FAIL)),
                Err(e) => Err(e)
            };
        }
        // The
        let label = unsafe { CreateWindowExA(
            WS_EX_TOPMOST,
            s!("STATIC"),
            PCSTR::null(), 
            WS_VISIBLE | WS_CHILD, 
            0, 0, 2048, 128, 
            window, None, dll_module(), 
            None) };
        unsafe{ SendMessageA(label, WM_SETFONT, WPARAM {0: FONT.0 as usize}, LPARAM {0: 1}) };
        debug!("Created candidate list.");
        Ok(CandidateList{window, label})
    }

    pub fn locate(&self, x: i32, y: i32) -> Result<()>{
        trace!("locate({x}, {y})");
        unsafe {SetWindowPos(
            self.window, HWND_TOPMOST, 
            x + POS_OFFSETX, y + POS_OFFSETY, 0, 0,
            SWP_NOACTIVATE | SWP_NOSIZE)? };
        Ok(())
    }
    
    pub fn show(&self, text: &OsString) -> Result<()>{
        trace!("show");
        unsafe {
            let height;
            let width;
            let mut size = SIZE::default();
            let encoded: Vec<u16> = text.encode_wide().collect();
            if GetTextExtentPoint32W(DC,&encoded , &mut size).as_bool() {
                height = PADDING_TOP + PADDING_BOTTOM + size.cy;
                width = PADDING_LEFT + PADDING_RIGHT + size.cx;
            } else {
                warn!("GetTextExtentPoint32W failed.");
                let len: i32 = text.to_string_lossy().chars().count().try_into().unwrap();
                height = PADDING_TOP + PADDING_BOTTOM + FONT_SIZE;
                width = PADDING_LEFT + PADDING_RIGHT + FONT_SIZE * len;
            }
            
            SetWindowTextW(self.label,&HSTRING::from(text))?;
            SetWindowPos(
                self.window, HWND_TOPMOST, 
                0, 0, width, height,
                SWP_NOACTIVATE | SWP_NOMOVE)?;
            ShowWindow(self.window, SW_SHOWNOACTIVATE);
        }
        Ok(())
    }

    pub fn hide(&self) {
        trace!("hide");
        unsafe { ShowWindow(self.window, SW_HIDE); }
    }
}

//----------------------------------------------------------------------------
//
//  Getter
//
//----------------------------------------------------------------------------

#[allow(unused)]
const fn wm(code: u32) -> &'static str {
    match code {
        0x0000 =>"WM_NULL",
        0x0001 =>"WM_CREATE",
        0x0002 =>"WM_DESTROY",
        0x0003 =>"WM_MOVE",
        0x0005 =>"WM_SIZE",
        0x0006 =>"WM_ACTIVATE",
        0x0007 =>"WM_SETFOCUS",
        0x0008 =>"WM_KILLFOCUS",
        0x000A =>"WM_ENABLE",
        0x000B =>"WM_SETREDRAW",
        0x000C =>"WM_SETTEXT",
        0x000D =>"WM_GETTEXT",
        0x000E =>"WM_GETTEXTLENGTH",
        0x000F =>"WM_PAINT",
        0x0010 =>"WM_CLOSE",
        0x0011 =>"WM_QUERYENDSESSION",
        0x0013 =>"WM_QUERYOPEN",
        0x0016 =>"WM_ENDSESSION",
        0x0012 =>"WM_QUIT",
        0x0014 =>"WM_ERASEBKGND",
        0x0015 =>"WM_SYSCOLORCHANGE",
        0x0018 =>"WM_SHOWWINDOW",
        0x001A =>"WM_WININICHANGE",
        0x001B =>"WM_DEVMODECHANGE",
        0x001C =>"WM_ACTIVATEAPP",
        0x001D =>"WM_FONTCHANGE",
        0x001E =>"WM_TIMECHANGE",
        0x001F =>"WM_CANCELMODE",
        0x0020 =>"WM_SETCURSOR",
        0x0021 =>"WM_MOUSEACTIVATE",
        0x0022 =>"WM_CHILDACTIVATE",
        0x0023 =>"WM_QUEUESYNC",
        0x0024 =>"WM_GETMINMAXINFO",
        0x0026 =>"WM_PAINTICON",
        0x0027 =>"WM_ICONERASEBKGND",
        0x0028 =>"WM_NEXTDLGCTL",
        0x002A =>"WM_SPOOLERSTATUS",
        0x002B =>"WM_DRAWITEM",
        0x002C =>"WM_MEASUREITEM",
        0x002D =>"WM_DELETEITEM",
        0x002E =>"WM_VKEYTOITEM",
        0x002F =>"WM_CHARTOITEM",
        0x0030 =>"WM_SETFONT",
        0x0031 =>"WM_GETFONT",
        0x0032 =>"WM_SETHOTKEY",
        0x0033 =>"WM_GETHOTKEY",
        0x0037 =>"WM_QUERYDRAGICON",
        0x0039 =>"WM_COMPAREITEM",
        0x003D =>"WM_GETOBJECT",
        0x0041 =>"WM_COMPACTING",
        0x0044 =>"WM_COMMNOTIFY",
        0x0046 =>"WM_WINDOWPOSCHANGING",
        0x0047 =>"WM_WINDOWPOSCHANGED",
        0x0048 =>"WM_POWER",
        0x004A =>"WM_COPYDATA",
        0x004B =>"WM_CANCELJOURNAL",
        0x004E =>"WM_NOTIFY",
        0x0050 =>"WM_INPUTLANGCHANGEREQUEST",
        0x0051 =>"WM_INPUTLANGCHANGE",
        0x0052 =>"WM_TCARD",
        0x0053 =>"WM_HELP",
        0x0054 =>"WM_USERCHANGED",
        0x0055 =>"WM_NOTIFYFORMAT",
        0x007B =>"WM_CONTEXTMENU",
        0x007C =>"WM_STYLECHANGING",
        0x007D =>"WM_STYLECHANGED",
        0x007E =>"WM_DISPLAYCHANGE",
        0x007F =>"WM_GETICON",
        0x0080 =>"WM_SETICON",
        0x0081 =>"WM_NCCREATE",
        0x0082 =>"WM_NCDESTROY",
        0x0083 =>"WM_NCCALCSIZE",
        0x0084 =>"WM_NCHITTEST",
        0x0085 =>"WM_NCPAINT",
        0x0086 =>"WM_NCACTIVATE",
        0x0087 =>"WM_GETDLGCODE",
        0x0088 =>"WM_SYNCPAINT",
        0x00A0 =>"WM_NCMOUSEMOVE",
        0x00A1 =>"WM_NCLBUTTONDOWN",
        0x00A2 =>"WM_NCLBUTTONUP",
        0x00A3 =>"WM_NCLBUTTONDBLCLK",
        0x00A4 =>"WM_NCRBUTTONDOWN",
        0x00A5 =>"WM_NCRBUTTONUP",
        0x00A6 =>"WM_NCRBUTTONDBLCLK",
        0x00A7 =>"WM_NCMBUTTONDOWN",
        0x00A8 =>"WM_NCMBUTTONUP",
        0x00A9 =>"WM_NCMBUTTONDBLCLK",
        0x00AB =>"WM_NCXBUTTONDOWN",
        0x00AC =>"WM_NCXBUTTONUP",
        0x00AD =>"WM_NCXBUTTONDBLCLK",
        0x00FE =>"WM_INPUT_DEVICE_CHANGE",
        0x00FF =>"WM_INPUT",
        0x0100 =>"WM_KEYDOWN",
        0x0101 =>"WM_KEYUP",
        0x0102 =>"WM_CHAR",
        0x0103 =>"WM_DEADCHAR",
        0x0104 =>"WM_SYSKEYDOWN",
        0x0105 =>"WM_SYSKEYUP",
        0x0106 =>"WM_SYSCHAR",
        0x0107 =>"WM_SYSDEADCHAR",
        0x0109 =>"WM_KEYLAST",
        0x0108 =>"WM_KEYLAST",
        0x010D =>"WM_IME_STARTCOMPOSITION",
        0x010E =>"WM_IME_ENDCOMPOSITION",
        0x010F =>"WM_IME_COMPOSITION",
        0x0110 =>"WM_INITDIALOG",
        0x0111 =>"WM_COMMAND",
        0x0112 =>"WM_SYSCOMMAND",
        0x0113 =>"WM_TIMER",
        0x0114 =>"WM_HSCROLL",
        0x0115 =>"WM_VSCROLL",
        0x0116 =>"WM_INITMENU",
        0x0117 =>"WM_INITMENUPOPUP",
        0x0119 =>"WM_GESTURE",
        0x011A =>"WM_GESTURENOTIFY",
        0x011F =>"WM_MENUSELECT",
        0x0120 =>"WM_MENUCHAR",
        0x0121 =>"WM_ENTERIDLE",
        0x0122 =>"WM_MENURBUTTONUP",
        0x0123 =>"WM_MENUDRAG",
        0x0124 =>"WM_MENUGETOBJECT",
        0x0125 =>"WM_UNINITMENUPOPUP",
        0x0126 =>"WM_MENUCOMMAND",
        0x0127 =>"WM_CHANGEUISTATE",
        0x0128 =>"WM_UPDATEUISTATE",
        0x0129 =>"WM_QUERYUISTATE",
        0x0132 =>"WM_CTLCOLORMSGBOX",
        0x0133 =>"WM_CTLCOLOREDIT",
        0x0134 =>"WM_CTLCOLORLISTBOX",
        0x0135 =>"WM_CTLCOLORBTN",
        0x0136 =>"WM_CTLCOLORDLG",
        0x0137 =>"WM_CTLCOLORSCROLLBAR",
        0x0138 =>"WM_CTLCOLORSTATIC",
        0x0200 =>"WM_MOUSEMOVE",
        0x0201 =>"WM_LBUTTONDOWN",
        0x0202 =>"WM_LBUTTONUP",
        0x0203 =>"WM_LBUTTONDBLCLK",
        0x0204 =>"WM_RBUTTONDOWN",
        0x0205 =>"WM_RBUTTONUP",
        0x0206 =>"WM_RBUTTONDBLCLK",
        0x0207 =>"WM_MBUTTONDOWN",
        0x0208 =>"WM_MBUTTONUP",
        0x0209 =>"WM_MBUTTONDBLCLK",
        0x020A =>"WM_MOUSEWHEEL",
        0x020B =>"WM_XBUTTONDOWN",
        0x020C =>"WM_XBUTTONUP",
        0x020D =>"WM_XBUTTONDBLCLK",
        0x020E =>"WM_MOUSEHWHEEL",
        0x0210 =>"WM_PARENTNOTIFY",
        0x0211 =>"WM_ENTERMENULOOP",
        0x0212 =>"WM_EXITMENULOOP",
        0x0213 =>"WM_NEXTMENU",
        0x0214 =>"WM_SIZING",
        0x0215 =>"WM_CAPTURECHANGED",
        0x0216 =>"WM_MOVING",
        0x0218 =>"WM_POWERBROADCAST",
        0x0219 =>"WM_DEVICECHANGE",
        0x0220 =>"WM_MDICREATE",
        0x0221 =>"WM_MDIDESTROY",
        0x0222 =>"WM_MDIACTIVATE",
        0x0223 =>"WM_MDIRESTORE",
        0x0224 =>"WM_MDINEXT",
        0x0225 =>"WM_MDIMAXIMIZE",
        0x0226 =>"WM_MDITILE",
        0x0227 =>"WM_MDICASCADE",
        0x0228 =>"WM_MDIICONARRANGE",
        0x0229 =>"WM_MDIGETACTIVE",
        0x0230 =>"WM_MDISETMENU",
        0x0231 =>"WM_ENTERSIZEMOVE",
        0x0232 =>"WM_EXITSIZEMOVE",
        0x0233 =>"WM_DROPFILES",
        0x0234 =>"WM_MDIREFRESHMENU",
        0x238 =>"WM_POINTERDEVICECHANGE",
        0x239 =>"WM_POINTERDEVICEINRANGE",
        0x23A =>"WM_POINTERDEVICEOUTOFRANGE",
        0x0240 =>"WM_TOUCH",
        0x0241 =>"WM_NCPOINTERUPDATE",
        0x0242 =>"WM_NCPOINTERDOWN",
        0x0243 =>"WM_NCPOINTERUP",
        0x0245 =>"WM_POINTERUPDATE",
        0x0246 =>"WM_POINTERDOWN",
        0x0247 =>"WM_POINTERUP",
        0x0249 =>"WM_POINTERENTER",
        0x024A =>"WM_POINTERLEAVE",
        0x024B =>"WM_POINTERACTIVATE",
        0x024C =>"WM_POINTERCAPTURECHANGED",
        0x024D =>"WM_TOUCHHITTESTING",
        0x024E =>"WM_POINTERWHEEL",
        0x024F =>"WM_POINTERHWHEEL",
        0x0251 =>"WM_POINTERROUTEDTO",
        0x0252 =>"WM_POINTERROUTEDAWAY",
        0x0253 =>"WM_POINTERROUTEDRELEASED",
        0x0281 =>"WM_IME_SETCONTEXT",
        0x0282 =>"WM_IME_NOTIFY",
        0x0283 =>"WM_IME_CONTROL",
        0x0284 =>"WM_IME_COMPOSITIONFULL",
        0x0285 =>"WM_IME_SELECT",
        0x0286 =>"WM_IME_CHAR",
        0x0288 =>"WM_IME_REQUEST",
        0x0290 =>"WM_IME_KEYDOWN",
        0x0291 =>"WM_IME_KEYUP",
        0x02A1 =>"WM_MOUSEHOVER",
        0x02A3 =>"WM_MOUSELEAVE",
        0x02A0 =>"WM_NCMOUSEHOVER",
        0x02A2 =>"WM_NCMOUSELEAVE",
        0x02B1 =>"WM_WTSSESSION_CHANGE",
        0x02c0 =>"WM_TABLET_FIRST",
        0x02df =>"WM_TABLET_LAST",
        0x02E0 =>"WM_DPICHANGED",
        0x02E2 =>"WM_DPICHANGED_BEFOREPARENT",
        0x02E3 =>"WM_DPICHANGED_AFTERPARENT",
        0x02E4 =>"WM_GETDPISCALEDSIZE",
        0x0300 =>"WM_CUT",
        0x0301 =>"WM_COPY",
        0x0302 =>"WM_PASTE",
        0x0303 =>"WM_CLEAR",
        0x0304 =>"WM_UNDO",
        0x0305 =>"WM_RENDERFORMAT",
        0x0306 =>"WM_RENDERALLFORMATS",
        0x0307 =>"WM_DESTROYCLIPBOARD",
        0x0308 =>"WM_DRAWCLIPBOARD",
        0x0309 =>"WM_PAINTCLIPBOARD",
        0x030A =>"WM_VSCROLLCLIPBOARD",
        0x030B =>"WM_SIZECLIPBOARD",
        0x030C =>"WM_ASKCBFORMATNAME",
        0x030D =>"WM_CHANGECBCHAIN",
        0x030E =>"WM_HSCROLLCLIPBOARD",
        0x030F =>"WM_QUERYNEWPALETTE",
        0x0310 =>"WM_PALETTEISCHANGING",
        0x0311 =>"WM_PALETTECHANGED",
        0x0312 =>"WM_HOTKEY",
        0x0317 =>"WM_PRINT",
        0x0318 =>"WM_PRINTCLIENT",
        0x0319 =>"WM_APPCOMMAND",
        0x031A =>"WM_THEMECHANGED",
        0x031D =>"WM_CLIPBOARDUPDATE",
        0x031E =>"WM_DWMCOMPOSITIONCHANGED",
        0x031F =>"WM_DWMNCRENDERINGCHANGED",
        0x0320 =>"WM_DWMCOLORIZATIONCOLORCHANGED",
        0x0321 =>"WM_DWMWINDOWMAXIMIZEDCHANGE",
        0x0323 =>"WM_DWMSENDICONICTHUMBNAIL",
        0x0326 =>"WM_DWMSENDICONICLIVEPREVIEWBITMAP",
        0x033F =>"WM_GETTITLEBARINFOEX",
        0x0358 =>"WM_HANDHELDFIRST",
        0x035F =>"WM_HANDHELDLAST",
        0x0360 =>"WM_AFXFIRST",
        0x037F =>"WM_AFXLAST",
        0x0380 =>"WM_PENWINFIRST",
        0x038F =>"WM_PENWINLAST",
        0x8000 =>"WM_APP",
        _ => "UNKNOWN",
    }
}


