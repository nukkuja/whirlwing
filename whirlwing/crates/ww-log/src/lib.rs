pub fn log_hello() {
    println!("Hello");
}

#[inline]
pub fn write_time() -> String {
    format!("{}", chrono::Local::now().format("%Y-%m-%d | %H:%M:%S"))
}

#[macro_export]
macro_rules! log {
    ($($args:tt),*) => {
        println!("{} | {}", $crate::write_time(), format!($($args),*))
    };
}