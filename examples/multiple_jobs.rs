use cronitor_macro::cronitor;
use cronitor_runtime::cron_runtime;

#[cronitor("*/5 * * * *")]
fn ping_every_5_minutes() {
    println!("Ping! Pong!");
}

#[cronitor("*/15 * * * *")]
fn ping_every_15_minutes() {
    println!("Ping! Pong! part 2");
}


fn main() {
    cron_runtime();
    
    loop {
        std::thread::park();
    }
}
