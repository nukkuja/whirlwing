fn main() {
    use whirlwing::wwg_log::{err, info, trace, warn, wwg_err, wwg_info, wwg_trace, wwg_warn};
    whirlwing::wwg_log::log_utils::set_log_default(whirlwing::wwg_log::log_utils::Severity::None);
    let f = 5;
    trace!("I'm calling trace macro! Var = {}.", f);
    info!("This is info macro. This information was provided to you by me :)");
    warn!("I WARN YOU!");
    err!("Your app is definitely broken.");

    wwg_trace!("I'm calling trace macro! Var = {}.", f);
    wwg_info!("This is info macro. This information was provided to you by me :)");
    wwg_warn!("I WARN YOU!");
    wwg_err!("Your app is definitely broken.");
}
