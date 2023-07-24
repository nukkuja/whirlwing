// This module is intended for internal use.
#[doc(hidden)]
pub mod log_utils;

#[macro_export]
macro_rules! trace {
    ($($args:tt),*) => {
        println!("{}", $crate::log_utils::_paint_trace(format!("{} | {} | {}", "TRACE", $crate::log_utils::_time(), format!($($args),*))));
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt),*) => {
        println!("{}", $crate::log_utils::_paint_info(format!("{} | {} | {}", "INFO", $crate::log_utils::_time(), format!($($args),*))));
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt),*) => {
        println!("{}", $crate::log_utils::_paint_warn(format!("{} | {} | {}", "WARNING", $crate::log_utils::_time(), format!($($args),*))));
    };
}

#[macro_export]
macro_rules! err {
    ($($args:tt),*) => {
        println!("{}", $crate::log_utils::_paint_err(format!("{} | {} | {}", "ERROR", $crate::log_utils::_time(), format!($($args),*))));
    };
}
