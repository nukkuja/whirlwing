#![allow(dead_code)]

pub mod win32_error;
mod win32_utils;

mod win32_window;
mod win32_windowing_context;

macro_rules! load_wgl_extension {
    ($name_str:literal into $name:ident($($args:ty),*)) => {
        let function_holder = $crate::wgl_get_proc_address($crate::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder) };
    };

    ($name_str:literal into $name:ident($($args:ty),*) -> $ret:ty) => {
        let function_holder = $crate::wgl_get_proc_address($crate::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder) };
    };

        ($name_str:literal into $name:ident($($args:ty),*,)) => {
        let function_holder = $crate::wgl_get_proc_address($crate::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*)>
        (function_holder) };
    };

    ($name_str:literal into $name:ident($($args:ty),*,) -> $ret:ty) => {
        let function_holder = $crate::wgl_get_proc_address($crate::s!($name_str))?;
        #[allow(non_snake_case)]
        let $name = unsafe { std::mem::transmute::
        <unsafe extern "system" fn () -> isize,
        unsafe extern "system" fn($($args),*) -> $ret>
        (function_holder) };
    };
}

#[allow(unused_imports)]
use windows::{
    core::{PCSTR, PCWSTR},
    s, w,
    Win32::{
        Foundation::*,
        Graphics::{Gdi::*, OpenGL::*},
        System::LibraryLoader::*,
        UI::WindowsAndMessaging::*,
    },
};

use win32_error::*;
use win32_utils::GL_TRUE;
use win32_utils::*;
use wwg_window_internal::Window;

struct WindowsWindow {
    hwnd: HWND,
}

pub fn create_wc_and_window() -> Result<(), WindowsError> {
    use wwg_window_internal::WindowingContext;
    let wc = win32_windowing_context::WindowingContextWin32::init()?;
    let window = wc.create_window("Hello, Whirlwing", 1280, 720)?;
    let window2 = wc.create_window("Helper Window", 100, 100)?;
    window.make_current()?;
    unsafe {
        ShowWindow(window.hwnd, SW_SHOW);
        gl::Viewport(0, 0, 1280, 720);
        gl::ClearColor(1f32, 0f32, 1f32, 1f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        SwapBuffers(window.device_context);
    }

    window2.make_current()?;
    unsafe {
        ShowWindow(window2.hwnd, SW_SHOW);
        gl::Viewport(0, 0, 100, 100);
        gl::ClearColor(0f32, 1f32, 1f32, 1f32);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        SwapBuffers(window2.device_context);
    }
    loop {}
    Ok(())
}
