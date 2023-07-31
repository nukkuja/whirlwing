use windows::{
    core::PCWSTR,
    Win32::Foundation::{
        HWND, WPARAM, LPARAM, LRESULT, RECT,
    },
    Win32::{
        UI::WindowsAndMessaging::*,
        Graphics::Gdi::{HBRUSH, GetDC},
        System::LibraryLoader::GetModuleHandleW,
    },
    w,
};

pub struct WindowInternal {
    hwnd: HWND,
}

impl WindowInternal {
    pub fn process_messages(&self) -> bool {
        let mut msg = MSG::default();
        loop {
            unsafe { 
                if PeekMessageW(&mut msg, self.hwnd, 0, 0, PM_REMOVE) != true {
                    return false;
                }
                if msg.message == WM_QUIT {
                    return true;
                }
    
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

pub fn create_window() -> WindowInternal {
    let wnd_class_name = w!("Whirlwing window class");
    let hinstance = unsafe { GetModuleHandleW(None).unwrap() };

    let wnd_class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_OWNDC,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: HICON::default(),
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW).unwrap() },
        hbrBackground: HBRUSH::default(),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: wnd_class_name,
        hIconSm: HICON::default(),
    };

    unsafe { RegisterClassExW(&wnd_class); }

    let wnd_style = WS_OVERLAPPED | WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU;

    let mut rect = RECT { left: 0, right: 800, top: 0, bottom: 600 };
    unsafe { AdjustWindowRect(&mut rect, wnd_style, false); }


    let hwnd = unsafe { CreateWindowExW(
        WS_EX_LEFT,
        wnd_class_name,
        w!("Whirlwing window"), 
        wnd_style,
        200,
        200,
        rect.right - rect.left,
        rect.bottom - rect.top,
        None,
        None,
        hinstance,
        None,
    ) };

    unsafe { ShowWindow(hwnd, SW_SHOW); }

    WindowInternal { hwnd }
}

unsafe extern "system"
fn window_proc(hwnd: HWND, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match umsg {
        WM_DESTROY => {
            wwg_log::warn!("Posting quit message");
            PostQuitMessage(0);
            LRESULT::default()
        }
        _ => DefWindowProcW(hwnd, umsg, wparam, lparam),
    }
}
