extern crate regex;
use crate::reader::parse;
use crate::{RawCron, Reader, TaCron};
use regex::Regex;
use std::fs;

pub struct CrontabReader {
    file: String,
}

impl CrontabReader {
    pub fn new(file: String) -> Self {
        CrontabReader { file: file }
    }
}

impl Reader for CrontabReader {
    fn read(&self) -> Vec<RawCron> {
        let mut tasks: Vec<RawCron> = Vec::new();
        let content = fs::read_to_string(&self.file).expect("Unable to read file");

        let comment_regex = Regex::new(r"^\s*#").unwrap();
        let line_regex = Regex::new(r"\s+").unwrap();

        for line in content.split("\n") {
            if comment_regex.is_match(line) == true || line.len() == 0 {
                continue;
            }

            let data = line_regex.split(line);
            let cron: Vec<&str> = data.collect();

            if line.chars().next() == Some('@') {
                let ta_cron = match cron[0] {
                    "@yearly" | "@annually" => RawCron::new(
                        "0",
                        "0",
                        "1",
                        "1",
                        "*",
                        &cron[1..].join(" "),
                        &self.file,
                    ),
                    "@monthly" => RawCron::new(
                        "0",
                        "0",
                        "1",
                        "*",
                        "*",
                        &cron[1..].join(" "),
                        &self.file,
                    ),
                    "@weekly" => RawCron::new(
                        "0",
                        "0",
                        "*",
                        "*",
                        "0",
                        &cron[1..].join(" "),
                        &self.file,
                    ),
                    "@daily" | "@midnight" => RawCron::new(
                        "0",
                        "0",
                        "*",
                        "*",
                        "*",
                        &cron[1..].join(" "),
                        &self.file,
                    ),
                    "@hourly" => RawCron::new(
                        "0",
                        "*",
                        "*",
                        "*",
                        "*",
                        &cron[1..].join(" "),
                        &self.file,
                    ),
                    x => panic!("Invalid crontab value: {}", x),
                };

                tasks.push(ta_cron);
            } else {
                tasks.push(RawCron::new(
                    cron[0],
                    cron[1],
                    cron[2],
                    cron[3],
                    cron[4],
                    &cron[5..].join(" "),
                    &self.file,
                ));
            }
        }

        tasks
    }

    fn tacrons(&self) -> Vec<TaCron> {
        let raw_crons = self.read();
        let mut ta_crons = Vec::new();
        for raw_cron in raw_crons {
            ta_crons.push(parse(&raw_cron));
        }
        ta_crons
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::crontab_reader::CrontabReader;
    use crate::Reader;

    #[test]
    fn it_reads() {
        let origin = "fixtures/crontab";
        let reader = CrontabReader {
            file: origin.to_string(),
        };

        let tasks = reader.read();
        assert_eq!(tasks.len(), 4);

        let task = &tasks[0];
        assert_eq!(task.minute, "0");
        assert_eq!(task.hour, "1");
        assert_eq!(task.dom, "2");
        assert_eq!(task.month, "*");
        assert_eq!(task.dow, "*");
        assert_eq!(task.action, "/foo/bar");
        assert_eq!(task.origin, origin);

        let task = &tasks[1];
        assert_eq!(task.minute, "1");
        assert_eq!(task.hour, "2-3");
        assert_eq!(task.dom, "3,4");
        assert_eq!(task.month, "4");
        assert_eq!(task.dow, "5");
        assert_eq!(task.action, "baz \"foo\" 2>&1");
        assert_eq!(task.origin, origin);
    }
}
