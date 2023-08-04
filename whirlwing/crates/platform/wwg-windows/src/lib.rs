#![allow(dead_code)]

pub mod windows_error;
mod windows_utils;

macro_rules! load_wgl_extension {
    ($name_str:literal into $name:ident($($args:ty),*)) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder);
    };

    ($name_str:literal into $name:ident($($args:ty),*) -> $ret:ty) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder);
    };

        ($name_str:literal into $name:ident($($args:ty),*,)) => {
        let function_holder = $crate::wglGetProcAddress($crate::s!($name_str)).unwrap();
        #[allow(non_snake_case)]
        let $name = std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder);
    };

    ($name_str:literal into $name:ident($($args:ty),*,) -> $ret:ty) => {
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

#[allow(unused_imports)]
use windows::{
    s, w,
    core::PCWSTR,
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
        System::LibraryLoader::*,
        Graphics::{
            Gdi::*,
            OpenGL::*,
        },
    },
};

use windows_utils::*;
use windows_error::*;

struct WindowsWindow {
    hwnd: HWND,
}

pub fn create_window() -> Result<(), WindowsError> {
    let h_instance = windows_utils::get_instance_handle();
    let fake_wnd_class_name = w!("Fake window class");
    let fake_wnd_name = w!("Fake window");

    let fake_wnd_class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_OWNDC,
        lpfnWndProc: Some(fake_wnd_proc),
        hInstance: h_instance,
        lpszClassName: fake_wnd_class_name,
        ..Default::default()
    };

    windows_utils::register_window_class(fake_wnd_class)?;
    wwg_log::wwg_trace!("Fake window class is registered!");

    let h_fake_wnd = new_window(
        WINDOW_EX_STYLE::default(),
        fake_wnd_class_name,
        fake_wnd_name,
        WINDOW_STYLE::default(),
        640,
        480,
        None,
        None,
        h_instance,
        None,
    )?;
    wwg_log::wwg_trace!("Fake window is created!");

    let fake_wnd_dc = get_device_context(h_fake_wnd)?;
    wwg_log::wwg_trace!("Device context of fake window retrieved!");

    let pfd = PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 32,
        cDepthBits: 24,
        cStencilBits: 8,
        iLayerType: PFD_MAIN_PLANE,
        ..Default::default()
    };

    let fake_wnd_pixel_format = choose_pixel_format(fake_wnd_dc, &pfd)?;
    wwg_log::wwg_trace!("Pixel format for fake window is chosen.");

    set_pixel_format(fake_wnd_dc, fake_wnd_pixel_format, &pfd)?;
    wwg_log::wwg_trace!("Pixel format for fake window is set.");
    
    Ok(())
}

unsafe extern "system" fn fake_wnd_proc(h_wnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT  {
    DefWindowProcW(h_wnd, msg, w_param, l_param)
}
