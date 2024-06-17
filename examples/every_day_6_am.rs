use cronitor_macro::cronitor;
use cronitor_runtime::cron_runtime;

#[cronitor("0 4 * * *")]
fn ping_every_day_at_6() {
    println!("Ping! Pong!");
}


fn main() {
    cron_runtime();
    
    loop {
        std::thread::park();
    }
}
