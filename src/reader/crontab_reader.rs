extern crate regex;
use crate::{RawCron, Reader};
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

            let cron: Vec<&str> = line_regex.split(line).collect();

            let times_specs;
            let command_index;

            if line.chars().next() == Some('@') {
                command_index = 1;
                times_specs = match cron[0] {
                    "@yearly" | "@annually" => ["0", "0", "1", "1", "*"],
                    "@monthly" => ["0", "0", "1", "*", "*"],
                    "@weekly" => ["0", "0", "*", "*", "0"],
                    "@daily" | "@midnight" => ["0", "0", "*", "*", "*"],
                    "@hourly" => ["0", "*", "*", "*", "*"],
                    x => panic!("Invalid crontab value: {}", x),
                };
            } else {
                command_index = 5;
                times_specs = [cron[0], cron[1], cron[2], cron[3], cron[4]]
            }

            tasks.push(RawCron::new(
                times_specs[0],
                times_specs[1],
                times_specs[2],
                times_specs[3],
                times_specs[4],
                &cron[command_index..].join(" "),
                &self.file,
            ));
        }

        tasks
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
