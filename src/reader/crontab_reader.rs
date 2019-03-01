extern crate regex;
use crate::{Reader, TaCron};
use regex::Regex;
use std::fs;

pub struct CrontabReader {
    file: String,
}

impl CrontabReader {
    pub fn new(file: String) -> CrontabReader {
        CrontabReader { file: file }
    }
}

impl Reader for CrontabReader {
    fn read(&self) -> Vec<TaCron> {
        let mut tasks: Vec<TaCron> = Vec::new();
        let content = fs::read_to_string(&self.file).unwrap();

        let comment_regex = Regex::new(r"^\s*#").unwrap();

        for line in content.split("\n") {
            if comment_regex.is_match(line) == true || line.len() == 0 {
                continue;
            }

            tasks.push(tacron_from_crontab_line(line.to_string(), &self.file));
        }

        tasks
    }
}

fn tacron_from_crontab_line(line: String, origin: &String) -> TaCron {
    let re = Regex::new(r"\s+").unwrap();
    let data = re.split(&line);
    let cron: Vec<&str> = data.collect();
    TaCron::new(
        cron[0].to_string(),
        cron[1].to_string(),
        cron[2].to_string(),
        cron[3].to_string(),
        cron[4].to_string(),
        cron[5].to_string(),
        origin.to_string(),
    )
}
