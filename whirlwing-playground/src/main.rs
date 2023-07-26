fn main() {
    use whirlwing::wwg_log::{trace, info, warn, err};
    let f = 5;
    trace!("I'm calling trace macro! Var = {}.", f);
    info!("This is info macro. This information was provided to you by me :)");
    warn!("I WARN YOU!");
    err!("Your app is definitely broken.");
    
}
