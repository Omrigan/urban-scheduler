use crate::error::{Error, Result};
use chrono::{DateTime, FixedOffset, Local, Utc};

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Report {
    stages: Vec<Stage>,
    start_time: DateTime<Local>,
}

#[derive(Debug, Serialize)]
pub struct PublicReport {
    stages: Vec<Stage>,
    start_time: String,
    finish_time: String,
}


impl Report {
    pub fn new() -> Self {
        let mut report = Report {
            stages: Vec::new(),
            start_time: Local::now()
        };
        report.checkpoint("start");
        report
    }
    pub fn checkpoint(&mut self, name: &'static str) {
        let delta = (Local::now() - self.start_time);
        self.stages.push(Stage {
            name,
            timestamp: delta.num_milliseconds() as u64,
        })
    }
    pub fn finish(mut self) -> PublicReport {
        self.checkpoint("done");

        PublicReport {
            stages: self.stages,
            start_time: self.start_time.to_string(),
            finish_time: Local::now().to_string()
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
struct Stage {
    name: &'static str,
    timestamp: u64,
}