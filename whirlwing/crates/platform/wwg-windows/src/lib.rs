#![allow(dead_code)]

macro_rules! get_wgl_extension {
    ($name_str:literal, $name:ident($($args:ty),*)) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder);
    };

    ($name_str:literal, $name:ident($($args:ty),*) -> $ret:ty) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder);
    };

        ($name_str:literal, $name:ident($($args:ty),*,)) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder);
    };

    ($name_str:literal, $name:ident($($args:ty),*,) -> $ret:ty) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder);
    };
}

const WGL_NUMBER_PIXEL_FORMATS_ARB: i32 = 0x2000;
const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
const WGL_DRAW_TO_BITMAP_ARB: i32 = 0x2002;
const WGL_ACCELERATION_ARB: i32 = 0x2003;
const WGL_NEED_PALETTE_ARB: i32 = 0x2004;
const WGL_NEED_SYSTEM_PALETTE_ARB: i32 = 0x2005;
const WGL_SWAP_LAYER_BUFFERS_ARB: i32 = 0x2006;
const WGL_SWAP_METHOD_ARB: i32 = 0x2007;
const WGL_NUMBER_OVERLAYS_ARB: i32 = 0x2008;
const WGL_NUMBER_UNDERLAYS_ARB: i32 = 0x2009;
const WGL_TRANSPARENT_ARB: i32 = 0x200A;
const WGL_TRANSPARENT_RED_VALUE_ARB: i32 = 0x2037;
const WGL_TRANSPARENT_GREEN_VALUE_ARB: i32 = 0x2038;
const WGL_TRANSPARENT_BLUE_VALUE_ARB: i32 = 0x2039;
const WGL_TRANSPARENT_ALPHA_VALUE_ARB: i32 = 0x203A;
const WGL_TRANSPARENT_INDEX_VALUE_ARB: i32 = 0x203B;
const WGL_SHARE_DEPTH_ARB: i32 = 0x200C;
const WGL_SHARE_STENCIL_ARB: i32 = 0x200D;
const WGL_SHARE_ACCUM_ARB: i32 = 0x200E;
const WGL_SUPPORT_GDI_ARB: i32 = 0x200F;
const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
const WGL_STEREO_ARB: i32 = 0x2012;
const WGL_PIXEL_TYPE_ARB: i32 = 0x2013;
const WGL_COLOR_BITS_ARB: i32 = 0x2014;
const WGL_RED_BITS_ARB: i32 = 0x2015;
const WGL_RED_SHIFT_ARB: i32 = 0x2016;
const WGL_GREEN_BITS_ARB: i32 = 0x2017;
const WGL_GREEN_SHIFT_ARB: i32 = 0x2018;
const WGL_BLUE_BITS_ARB: i32 = 0x2019;
const WGL_BLUE_SHIFT_ARB: i32 = 0x201A;
const WGL_ALPHA_BITS_ARB: i32 = 0x201B;
const WGL_ALPHA_SHIFT_ARB: i32 = 0x201C;
const WGL_ACCUM_BITS_ARB: i32 = 0x201D;
const WGL_ACCUM_RED_BITS_ARB: i32 = 0x201E;
const WGL_ACCUM_GREEN_BITS_ARB: i32 = 0x201F;
const WGL_ACCUM_BLUE_BITS_ARB: i32 = 0x2020;
const WGL_ACCUM_ALPHA_BITS_ARB: i32 = 0x2021;
const WGL_DEPTH_BITS_ARB: i32 = 0x2022;
const WGL_STENCIL_BITS_ARB: i32 = 0x2023;
const WGL_AUX_BUFFERS_ARB: i32 = 0x2024;

const WGL_NO_ACCELERATION_ARB: i32 = 0x2025;
const WGL_GENERIC_ACCELERATION_ARB: i32 = 0x2026;
const WGL_FULL_ACCELERATION_ARB: i32 = 0x2027;

const WGL_SWAP_EXCHANGE_ARB: i32 = 0x2028;
const WGL_SWAP_COPY_ARB: i32 = 0x2029;
const WGL_SWAP_UNDEFINED_ARB: i32 = 0x202A;

const WGL_TYPE_RGBA_ARB: i32 = 0x202B;
const WGL_TYPE_COLORINDEX_ARB: i32 = 0x202C;

const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
const WGL_CONTEXT_LAYER_PLANE_ARB: i32 = 0x2093;
const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

const WGL_CONTEXT_DEBUG_BIT_ARB: i32 = 0x0001;
const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: i32 = 0x0002;

const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: i32 = 0x00000002;

const GL_TRUE: i32 = 1;
const GL_FALSE: i32 = 0;

use windows::{
    core::PCSTR,
    core::PCWSTR,
    s, w,
    Win32::Foundation::{HMODULE, HWND, LPARAM, LRESULT, RECT, WPARAM},
    Win32::{
        Graphics::{
            Gdi::{GetDC, HBRUSH, HDC, WGL_SWAP_MAIN_PLANE},
            OpenGL::*,
        },
        System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW},
        UI::WindowsAndMessaging::*,
    },
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

pub fn window() -> WindowInternal {
    let hinstance = unsafe { GetModuleHandleW(None).unwrap() };
    let wnd_class_name = w!("Window class");

    register_window_class(hinstance, wnd_class_name);
    let hwnd = create_window(hinstance, wnd_class_name, w!("My Window"), 640, 480);
    let fake_wnd = create_fake_context(hinstance, wnd_class_name);

    unsafe {
        let hdc = GetDC(hwnd);
        get_wgl_extension!(
            "wglChoosePixelFormatARB",
            wglChoosePixelFormatARB(
                HDC,
                *const i32,
                *const f32,
                u32,
                *const i32,
                *const u32,
            ) -> i32
        );
        get_wgl_extension!(
            "wglCreateContextAttribsARB",
            wglCreateContextAttribsARB(HDC, HGLRC, *const i32) -> HGLRC
        );

        DestroyWindow(fake_wnd);

        let attrib_list = [
            WGL_DRAW_TO_WINDOW_ARB,
            GL_TRUE,
            WGL_SUPPORT_OPENGL_ARB,
            GL_TRUE,
            WGL_DOUBLE_BUFFER_ARB,
            GL_TRUE,
            WGL_PIXEL_TYPE_ARB,
            WGL_TYPE_RGBA_ARB,
            WGL_COLOR_BITS_ARB,
            32,
            WGL_DEPTH_BITS_ARB,
            24,
            WGL_STENCIL_BITS_ARB,
            8,
            0,
        ];

        let pixel_format = i32::default();
        let num_formats = u32::default();
        let format_found = wglChoosePixelFormatARB(
            hdc,
            attrib_list.as_ptr(),
            std::ptr::null(),
            0u32,
            &pixel_format,
            &num_formats,
        );
        if format_found != 0 {
            wwg_log::wwg_info!("Format found!");
        }
        let ppfd = PIXELFORMATDESCRIPTOR::default();
        SetPixelFormat(hdc, format_found, &ppfd);

        let attrib_list = [
            WGL_CONTEXT_MAJOR_VERSION_ARB,
            4,
            WGL_CONTEXT_MINOR_VERSION_ARB,
            6,
            WGL_CONTEXT_PROFILE_MASK_ARB,
            WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
            0,
        ];
        let hglrc = wglCreateContextAttribsARB(hdc, HGLRC(0), attrib_list.as_ptr());
        if hglrc.0 != 0 {
            wwg_log::wwg_info!("hglrc is created");
        }
        wglMakeCurrent(hdc, hglrc);

        ShowWindow(hwnd, SW_SHOW);
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(1f32, 0f32, 1f32, 1f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // SwapBuffers(hdc);
        wglSwapLayerBuffers(hdc, WGL_SWAP_MAIN_PLANE);
    }

    WindowInternal { hwnd }
}

fn create_fake_context(hinstance: HMODULE, wnd_class_name: PCWSTR) -> HWND {
    let hwnd = create_window(hinstance, wnd_class_name, w!("Fake window"), 640, 480);

    let pfd = PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cDepthBits: 24,
        cStencilBits: 8,
        ..Default::default()
    };

    unsafe {
        let hdc = GetDC(hwnd);
        let format = ChoosePixelFormat(hdc, &pfd);
        SetPixelFormat(hdc, format, &pfd);
        let hglrc = wglCreateContext(hdc).unwrap();
        wglMakeCurrent(hdc, hglrc);

        gl::load_with(|string| {
            let fn_name = PCSTR(format!("{string}\0").as_ptr());

            let hmodule = LoadLibraryW(w!("opengl32.dll")).unwrap();

            match GetProcAddress(hmodule, fn_name) {
                Some(val) => return val as *const std::os::raw::c_void,
                None => match wglGetProcAddress(fn_name) {
                    Some(val) => return val as *const std::os::raw::c_void,
                    None => {
                        wwg_log::wwg_info!("Couldn't load {string}.");
                        return std::ptr::null();
                    }
                },
            }
        });
        print_gl_version();
        print_wgl_extensions(hdc);
    };
    hwnd
}

fn register_window_class(hinstance: HMODULE, wnd_class_name: PCWSTR) {
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

    unsafe {
        RegisterClassExW(&wnd_class);
    }
}

fn create_window(
    hinstance: HMODULE,
    wnd_class_name: PCWSTR,
    wnd_name: PCWSTR,
    width: i32,
    height: i32,
) -> HWND {
    let wnd_style = WS_OVERLAPPED | WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU;

    let mut rect = RECT {
        left: 0,
        right: width,
        top: 0,
        bottom: height,
    };
    unsafe {
        AdjustWindowRect(&mut rect, wnd_style, false);
    }

    unsafe {
        CreateWindowExW(
            WS_EX_LEFT,
            wnd_class_name,
            wnd_name,
            wnd_style,
            200,
            200,
            rect.right - rect.left,
            rect.bottom - rect.top,
            None,
            None,
            hinstance,
            None,
        )
    }
}

unsafe fn print_gl_version() {
    let string = gl::GetString(GL_VERSION);
    let c_str = std::ffi::CStr::from_ptr(string as *const i8);
    let string = c_str.to_str().unwrap().to_owned();
    wwg_log::wwg_info!("OpenGL version: {}", string);
}

unsafe fn print_wgl_extensions(hdc: HDC) {
    get_wgl_extension!(
        "wglGetExtensionsStringARB",
        wglGetExtensionsStringARB(HDC) -> *const std::ffi::c_char
    );

    let string = wglGetExtensionsStringARB(hdc);
    let c_string = std::ffi::CString::from_raw(string as *mut i8);
    let string = c_string
        .to_str()
        .unwrap()
        .to_owned()
        .trim()
        .replace(" ", "\n\t");
    wwg_log::wwg_info!("WGL extensions:\n\t{}", string);
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match umsg {
        WM_DESTROY => {
            wwg_log::wwg_info!("Posting quit message");
            PostQuitMessage(0);
            LRESULT::default()
        }
        _ => DefWindowProcW(hwnd, umsg, wparam, lparam),
    }
}
