pub trait Log {
    fn print(&self, message: LogMessage);
}

pub fn set_log(logger: Box<dyn Log>) {
    set_log_inner(|| Box::leak(logger))
}

pub fn set_log_default() {
    set_log_inner(|| &Logger { severity_filter: Severity::Trace });
}

fn set_log_inner<F: FnOnce() -> &'static dyn Log>(make_logger: F) {
    unsafe { LOG = make_logger(); }
} 

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

pub static mut LOG: &'static dyn Log = &Logger::new(Severity::Trace);


pub static mut LOG_ENGINE: &'static dyn Log = &Logger::new(Severity::Trace);

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Severity {
    Trace = 1,
    Info = 2,
    Warn = 3,
    Err = 4,
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

#[doc(hidden)]
#[inline]
pub fn _time() -> String {
    format!("{}", chrono::Local::now().format("%Y-%m-%d | %H:%M:%S"))
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
