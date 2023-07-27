//! This crate provides logging utilities and macros.
//! `log` feature provides macros for client log and `engine_log` feature provides macros for engine log.
//! 
//! Log is initialized by default and prints to stderr.
//! Use `log_utils::set_log_default` and `log_utils::set_engine_log_default` to set severity filters. 
//! Use `log_utils::set_log_custom` and `log_utils::set_engie_log_custom` to set up custom logger which will receive log messages.
//! 
//! Use `trace!, info!, warn!, err!` macros to log your text as you would use eprintln!.
//! 
//! # Examples
//! 
//! use whirlwing::wwg_log::{trace, warn, log_utils};
//! log_utils::set_log_default(log_utils::Severity::Warn);
//! trace!("Hello, World!"); // This will not be printed.
//! let x = 5;
//! warn!("x = {}", 5); // This message will be printed.
//! 
pub mod log_macros;
pub mod log_utils;