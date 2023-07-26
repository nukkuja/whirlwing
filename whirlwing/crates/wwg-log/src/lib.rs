// This module is intended for internal use.
#[doc(hidden)]
pub mod log_utils;

pub mod log_macros;

#[macro_export]
macro_rules! trace {
    ($($args:tt),+) => {
        let temp = format!("{}\t| {} | {}", "TRACE", $crate::log_utils::_time(), format!($($args),+));
        let temp = format!("{}", $crate::log_utils::_paint_trace(temp));
        let log_message = $crate::log_utils::LogMessage::new(temp, $crate::log_utils::Severity::Trace);
        unsafe { $crate::log_utils::LOG.print(log_message); }
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
