// These are static variables that will collect log messages.
pub static mut LOG: &'static dyn Log = &Logger::new(Severity::Trace);

#[cfg(feature = "engine_log")]
pub static mut LOG_ENGINE: &'static dyn Log = &Logger::new(Severity::Trace);

/// This function sets standard logger, which prints log messages to stderr.
/// You can use this function to set severity filter.
pub fn set_log_default(severity_filter: Severity) {
    set_log_inner(|| {
        let logger = Box::new(Logger { severity_filter });
        Box::leak(logger)
    });
}

/// This function sets your custom logger, which on log message will trigger print function of Log trait.
pub fn set_log_custom(logger: Box<dyn Log>) {
    set_log_inner(|| Box::leak(logger))
}

/// Works exactly as `set_log_default` but affects engine_log.
#[cfg(feature = "engine_log")]
pub fn set_engine_log_default(severity_filter: Severity) {
    set_engine_log_inner(|| {
        let logger = Box::new(Logger { severity_filter });
        Box::leak(logger)
    });
}

/// Works exactly as `set_log_custom` but affects engine_log.
#[cfg(feature = "engine_log")]
pub fn set_engine_log_custom(logger: Box<dyn Log>) {
    set_engine_log_inner(|| Box::leak(logger))
}

/// Trait must be implemented by object to set it as logger.
/// Print function will be called every time when one of log macros is used.
pub trait Log {
    fn print(&self, message: LogMessage);
}

/// Standard logger. Prints log messages to stderr.
#[derive(Debug)]
struct Logger {
    severity_filter: Severity,
}

impl Logger {
    const fn new(severity_filter: Severity) -> Self {
        Logger { severity_filter }
    }
}

impl Log for Logger {
    fn print(&self, message: LogMessage) {
        if message.severity >= self.severity_filter {
            let formatted;
            if message.engine_log {
                formatted = format!(
                    "WHIRLWING | {}\t| {} | {}",
                    message.severity,
                    message.format_time(),
                    message.content
                );
            } else {
                formatted = format!(
                    "LOG | {}\t| {} | {}",
                    message.severity,
                    message.format_time(),
                    message.content
                );
            }
            eprintln!("{}", message.colour.paint(formatted));
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Severity {
    Trace = 1,
    Info = 2,
    Warn = 3,
    Err = 4,
    None = 5,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Severity::Trace => write!(f, "TRACE"),
            Severity::Info => write!(f, "INFO"),
            Severity::Warn => write!(f, "WARNING"),
            Severity::Err => write!(f, "ERROR"),
            Severity::None => write!(f, "NONE"),
        }
    }
}

#[derive(Debug)]
pub struct LogMessage {
    content: String,
    severity: Severity,
    time: chrono::DateTime<chrono::Local>,
    colour: ansi_term::Colour,
    engine_log: bool,
}

impl LogMessage {
    pub fn new(content: String, severity: Severity, colour: ansi_term::Colour, engine_log: bool) -> Self {
        LogMessage { content, severity, time: chrono::Local::now(), colour, engine_log }
    }

    pub fn format_time(&self) -> String {
        format!("{}", self.time.format("%d.%m.%Y | %H:%M:%S"))
    }
}

// Next functions are mostly used by macros and are not intended for external usage.

#[doc(hidden)]
#[inline]
pub fn _format_log_message_trace(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Trace,
        ansi_term::Colour::RGB(0, 255, 255),
        false,
    )
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_info(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Info,
        ansi_term::Colour::White,
        false,
    )
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_warn(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Warn,
        ansi_term::Colour::Yellow,
        false,
    )
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_err(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Err,
        ansi_term::Colour::Red,
        false,
    )
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_trace(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Trace,
        ansi_term::Colour::RGB(0, 255, 255),
        true,
    )
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_info(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Info,
        ansi_term::Colour::White,
        true,
    )
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_warn(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Warn,
        ansi_term::Colour::Yellow,
        true,
    )
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_err(args: std::fmt::Arguments) -> LogMessage {
    LogMessage::new(
        format!("{}", args),
        Severity::Err,
        ansi_term::Colour::Red,
        true,
    )
}

fn set_log_inner<F: FnOnce() -> &'static dyn Log>(make_logger: F) {
    unsafe {
        LOG = make_logger();
    }
}

#[cfg(feature = "engine_log")]
fn set_engine_log_inner<F: FnOnce() -> &'static dyn Log>(make_logger: F) {
    unsafe {
        LOG_ENGINE = make_logger();
    }
}
