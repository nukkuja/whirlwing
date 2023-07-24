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
