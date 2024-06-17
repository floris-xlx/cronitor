use cronitor_core::{cron_runtime, cronitor};

#[cronitor("*/5 * * * *")]
fn ping_every_5_minutes() {
    println!("Ping! Pong!");
}

fn main() {
    cron_runtime();
    
    loop {
        std::thread::park();
    }
}