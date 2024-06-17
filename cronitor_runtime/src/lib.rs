use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::thread;
use std::time::Duration;
use chrono::prelude::*;
use colored::*;
use cron_parser::parse;

lazy_static! {
    pub static ref CRON_REGISTRY: Mutex<CronRegistry> = Mutex::new(CronRegistry::new());
}

pub struct CronRegistry {
    tasks: HashMap<String, (String, fn())>,
}

impl CronRegistry {
    pub fn new() -> Self {
        CronRegistry {
            tasks: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, cron_expression: String, task: fn()) {
        if let Ok(_) = parse(&cron_expression, &Local::now()) {
            self.tasks.insert(name.clone(), (cron_expression.clone(), task));
            self.log_next_run(&name, &cron_expression);
        } else {
            println!("Invalid cron expression for {}: {}", name.bright_red(), cron_expression.bright_red());
        }
    }

    fn log_next_run(&self, name: &str, cron_expression: &str) {
        println!("Cron expression for {}: {}", name.bright_yellow(), cron_expression);
        if let Ok(next_run) = parse(cron_expression, &Local::now()) {
            let now = Local::now();
            let duration = next_run - now;
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            println!(
                "{}\n{}\n{}\n{}\n{}\n{}",
                "+---------------------------------+".bright_blue(),
                format!("| Current time: {}", now.format("%Y-%m-%d %H:%M:%S")).bright_red(),
                format!("| Function: {}", name).bright_green(),
                format!("| Frequency: {}", cron_expression).bright_yellow(),
                format!("| Next run in: {} hours, {} minutes", hours, minutes).bright_red(),
                "+---------------------------------+".bright_blue()
            );
        } else {
            println!("Failed to calculate the next run time for {} with expression {}", name.bright_red(), cron_expression.bright_red());
        }
    }

    pub fn get_tasks(&self) -> &HashMap<String, (String, fn())> {
        &self.tasks
    }
}

pub fn cron_runtime() {
    thread::spawn(|| {
        loop {
            let tasks: HashMap<String, (String, fn())> = CRON_REGISTRY.lock().unwrap().get_tasks().clone();
            let now = Local::now();

            for (name, (cron_expression, task)) in tasks {
                if let Ok(next_run) = parse(&cron_expression, &now.with_timezone(&Utc)) {
                    let next_run = next_run.with_timezone(&Local);

                    println!(
                        "Next run for {}: {}",
                        name.bright_cyan().bold(),
                        next_run.format("%Y-%m-%d %H:%M:%S").to_string().bold()
                    );

                    if next_run <= now + chrono::Duration::seconds(60) {
                        println!("Running task: {}", name.bright_green());
                        task();
                    } else {
                        println!("Task {} not due yet. Next run: {}", name.bright_yellow(), next_run.to_rfc3339());
                    }
                } else {
                    println!("No upcoming run for {}", name.bright_red());
                }
            }

            thread::sleep(Duration::from_secs(60));
        }
    });
}
