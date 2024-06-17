//! # Cronitor
//! 
//! Cronitor is a simple and efficient cron framework in Rust. It allows you to schedule and run cron jobs with ease.
//! 
//! ## Examples
//! 
//! Here are some examples of how to use Cronitor:
//! 
//! ### Example 1: Run a job every minute
//! 
//! ```rust
//! use cronitor_macro::cronitor;
//! use cronitor_runtime::cron_runtime;
//! 
//! #[cronitor("*/1 * * * *")]
//! fn ping_every_minute() {
//!     println!("Ping! Pong!");
//! }
//! 
//! fn main() {
//!     cron_runtime();
//!     
//!     loop {
//!         std::thread::park();
//!     }
//! }
//! ```
//! 
//! ### Example 2: Run multiple jobs
//! 
//! ```rust
//! use cronitor_macro::cronitor;
//! use cronitor_runtime::cron_runtime;
//! 
//! #[cronitor("*/5 * * * *")]
//! fn ping_every_5_minutes() {
//!     println!("Ping! Pong!");
//! }
//! 
//! #[cronitor("*/15 * * * *")]
//! fn ping_every_15_minutes() {
//!     println!("Ping! Pong! part 2");
//! }
//! 
//! fn main() {
//!     cron_runtime();
//!     
//!     loop {
//!         std::thread::park();
//!     }
//! }
//! ```
//! 
//! ### Example 3: Run a job every day at 6 AM
//! 
//! ```rust
//! use cronitor_macro::cronitor;
//! use cronitor_runtime::cron_runtime;
//! 
//! #[cronitor("0 6 * * *")]
//! fn ping_every_day_at_6() {
//!     println!("Ping! Pong!");
//! }
//! 
//! fn main() {
//!     cron_runtime();
//!     
//!     loop {
//!         std::thread::park();
//!     }
//! }
//! ```
//! 
//! ## Example files
//! Example files can be found in /examples folder. You can run these examples using the following command:
//! 
//! [Examples folder](https://github.com/floris-xlx/cronitor/tree/main/examples)
//! 
//! ```sh
//! cargo run --example EXAMPLE_FILE.rs
//! ```
//! 
//! ## Timezone
//! 
//! Cronitor picks the timezone that the computer is in, ensuring that your cron jobs run according to the local time settings.
//! 
//! ## Synchronous Contexts
//! 
//! Cronitor is designed for synchronous contexts for now. This means that it is best suited for applications where tasks need to be executed in a sequential manner.
//! 
//! ## Verify Cron Expressions
//! 
//! If you want to verify a cron expression, you can use this website: [https://crontab.cronhub.io/](https://crontab.cronhub.io/)


pub use cronitor_macro::*;
pub use cronitor_runtime::*;
