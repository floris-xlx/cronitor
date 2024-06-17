use cronitor_macro::cronitor;
use cronitor_runtime::cron_runtime;
use std::thread;
use std::time::Duration;

#[cronitor("* * * * 1-5")]
fn send_alert() {
    println!("Sending alert!");
}

fn main() {
    cron_runtime();
    loop {
        // Your application logic here
        thread::sleep(Duration::from_secs(1));
    }
}
