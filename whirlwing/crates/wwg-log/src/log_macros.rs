#[macro_export]
macro_rules! trace {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_message_trace(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG.print(msg);
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_message_debug(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG.print(msg);
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_message_info(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG.print(msg);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_message_warn(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG.print(msg);
        }
    };
}

#[macro_export]
macro_rules! err {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_message_err(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG.print(msg);
        }
    }
}

#[cfg(feature = "engine_log")]
#[macro_export]
macro_rules! wwg_trace {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_engine_message_trace(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG_ENGINE.print(msg);
        }
    };
}

#[cfg(feature = "engine_log")]
#[macro_export]
macro_rules! wwg_debug {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_engine_message_debug(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG_ENGINE.print(msg);
        }
    };
}

#[cfg(feature = "engine_log")]
#[macro_export]
macro_rules! wwg_info {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_engine_message_info(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG_ENGINE.print(msg);
        }
    };
}

#[cfg(feature = "engine_log")]
#[macro_export]
macro_rules! wwg_warn {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_engine_message_warn(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG_ENGINE.print(msg);
        }
    };
}

#[cfg(feature = "engine_log")]
#[macro_export]
macro_rules! wwg_err {
    ($($args:tt),+) => {
        let msg = $crate::log_utils::_format_log_engine_message_err(format_args!($($args),+));
        unsafe {
            $crate::log_utils::LOG_ENGINE.print(msg);
        }
    };
}

#[cfg(not(feature = "engine_log"))]
#[macro_export]
macro_rules! wwg_trace {
    ($($_:tt),+) => {};
}

#[cfg(not(feature = "engine_log"))]
#[macro_export]
macro_rules! wwg_debug {
    ($($_:tt),+) => {};
}

#[cfg(not(feature = "engine_log"))]
#[macro_export]
macro_rules! wwg_info {
    ($($_:tt),+) => {};
}

#[cfg(not(feature = "engine_log"))]
#[macro_export]
macro_rules! wwg_warn {
    ($($_:tt),+) => {};
}

#[cfg(not(feature = "engine_log"))]
#[macro_export]
macro_rules! wwg_err {
    ($($_:tt),+) => {};
}
