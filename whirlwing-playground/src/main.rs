fn main() {
    whirlwing::wwg_log::log_utils::set_engine_log_default(whirlwing::wwg_log::log_utils::Severity::Info);
    whirlwing::app::Application::new().run();
}
