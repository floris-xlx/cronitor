use cronitor_macro::cronitor;
use cronitor_runtime::cron_runtime;

#[cronitor("*/1 * * * *")]
fn ping_every_minute() {
    println!("Ping! Pong!");
}

fn main() {
    cron_runtime();
    
    loop {
        std::thread::park();
    }
}
