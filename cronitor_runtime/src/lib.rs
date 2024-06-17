use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::thread;
use std::time::Duration;
use cron::Schedule;
use chrono::prelude::*;
use std::str::FromStr;

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
        self.tasks.insert(name, (cron_expression, task));
    }

    pub fn get_tasks(&self) -> &HashMap<String, (String, fn())> {
        &self.tasks
    }
}

pub fn cron_runtime() {
    thread::spawn(|| {
        loop {
            let tasks = CRON_REGISTRY.lock().unwrap().get_tasks().clone();
            let now = Utc::now();

            for (_, (cron_expression, task)) in tasks {
                let schedule = Schedule::from_str(&cron_expression).unwrap();

                if schedule.upcoming(Utc).take(1).next().unwrap() <= now {
                    task();
                }
            }

            thread::sleep(Duration::from_secs(60));
        }
    });
}
