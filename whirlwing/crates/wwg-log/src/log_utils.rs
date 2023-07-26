pub static mut LOG: &'static dyn Log = &Logger::new(Severity::Trace);

#[cfg(feature = "engine_log")]
pub static mut LOG_ENGINE: &'static dyn Log = &Logger::new(Severity::Trace);

pub fn set_log_default(severity_filter: Severity) {
    set_log_inner(|| {
        let logger = Box::new(Logger { severity_filter });
        Box::leak(logger)
    });
}

pub fn set_log_custom(logger: Box<dyn Log>) {
    set_log_inner(|| Box::leak(logger))
}

#[cfg(feature = "engine_log")]
pub fn set_engine_log_default(severity_filter: Severity) {
    set_engine_log_inner(|| {
        let logger = Box::new(Logger { severity_filter });
        Box::leak(logger)
    });
}

#[cfg(feature = "engine_log")]
pub fn set_engine_log_custom(logger: Box<dyn Log>) {
    set_engine_log_inner(|| Box::leak(logger))
}

pub trait Log {
    fn print(&self, message: LogMessage);
}

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
            eprintln!("{}", message.content);
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LogMessage {
    content: String,
    severity: Severity,
}

impl LogMessage {
    pub fn new(content: String, severity: Severity) -> Self {
        LogMessage { content, severity }
    }
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_args!("{}", self.content))
    }
}

fn set_log_inner<F: FnOnce() -> &'static dyn Log>(make_logger: F) {
    unsafe {
        LOG = make_logger();
    }
}

#[cfg(feature = "engine_log")]
fn set_engine_log_inner<F: FnOnce() -> &'static dyn Log>(make_logger: F) {
    unsafe {
        LOG = make_logger();
    }
}

#[doc(hidden)]
#[inline]
pub fn _time() -> String {
    format!("{}", chrono::Local::now().format("%d.%m.%Y | %H:%M:%S"))
}

#[doc(hidden)]
#[inline]
pub fn _paint_trace<'a>(string: String) -> ansi_term::ANSIGenericString<'a, str> {
    ansi_term::Colour::RGB(0, 255, 255).paint(string)
}

#[doc(hidden)]
#[inline]
pub fn _paint_info<'a>(string: String) -> ansi_term::ANSIGenericString<'a, str> {
    ansi_term::Colour::White.paint(string)
}

#[doc(hidden)]
#[inline]
pub fn _paint_warn<'a>(string: String) -> ansi_term::ANSIGenericString<'a, str> {
    ansi_term::Colour::Yellow.paint(string)
}

#[doc(hidden)]
#[inline]
pub fn _paint_err<'a>(string: String) -> ansi_term::ANSIGenericString<'a, str> {
    ansi_term::Colour::Red.paint(string)
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_trace(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "TRACE", _time(), args);
    let temp = format!("{}", _paint_trace(temp));
    LogMessage::new(temp, Severity::Trace)
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_info(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "INFO", _time(), args);
    let temp = format!("{}", _paint_info(temp));
    LogMessage::new(temp, Severity::Info)
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_warn(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "WARN", _time(), args);
    let temp = format!("{}", _paint_warn(temp));
    LogMessage::new(temp, Severity::Warn)
}

#[doc(hidden)]
#[inline]
pub fn _format_log_message_err(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "ERROR", _time(), args);
    let temp = format!("{}", _paint_err(temp));
    LogMessage::new(temp, Severity::Err)
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_trace(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "WHIRLWING | TRACE", _time(), args);
    let temp = format!("{}", _paint_trace(temp));
    LogMessage::new(temp, Severity::Trace)
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_info(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "WHIRLWING | INFO", _time(), args);
    let temp = format!("{}", _paint_info(temp));
    LogMessage::new(temp, Severity::Info)
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_warn(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "WHIRLWING | WARN", _time(), args);
    let temp = format!("{}", _paint_warn(temp));
    LogMessage::new(temp, Severity::Warn)
}

#[cfg(feature = "engine_log")]
#[doc(hidden)]
#[inline]
pub fn _format_log_engine_message_err(args: std::fmt::Arguments) -> LogMessage {
    let temp = format!("{}\t| {} | {}", "WHIRLWING | ERROR", _time(), args);
    let temp = format!("{}", _paint_err(temp));
    LogMessage::new(temp, Severity::Err)
}
