#[cfg(target_os = "windows")]
pub use key_codes_win32::*;

#[cfg(target_os = "windows")]
mod key_codes_win32 {
    pub const KC_A: char = 'A';
    pub const ESCAPE: char = '\u{1B}';
}
