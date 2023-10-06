#[allow(unused_imports)]
use whirlwing::log;

fn main() {
    log::log_utils::set_engine_log_default(log::log_utils::Severity::Debug);
    log::info!("Hello");
    whirlwing::app::run();
}
