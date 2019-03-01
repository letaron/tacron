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
        let line_regex = Regex::new(r"\s+").unwrap();

        for line in content.split("\n") {
            if comment_regex.is_match(line) == true || line.len() == 0 {
                continue;
            }

            let data = line_regex.split(line);
            let cron: Vec<&str> = data.collect();

            if line.chars().next() == Some('@') {
                let ta_cron = match cron[0] {
                    "@yearly" | "@annually" => TaCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "1".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@monthly" => TaCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@weekly" => TaCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        "0".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@daily" | "@midnight" => TaCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@hourly" => TaCron::new(
                        "0".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    x => panic!("Invalid crontab value: {}", x),
                };

                tasks.push(ta_cron);
            } else {
                tasks.push(TaCron::new(
                    cron[0].to_string(),
                    cron[1].to_string(),
                    cron[2].to_string(),
                    cron[3].to_string(),
                    cron[4].to_string(),
                    cron[5..].join(" "),
                    self.file.to_string(),
                ));
            }
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
        assert_eq!(tasks.len(), 9);

        let mut task = &tasks[0];
        assert_eq!(task.minute, "0");
        assert_eq!(task.hour, "1");
        assert_eq!(task.dom, "2");
        assert_eq!(task.month, "*");
        assert_eq!(task.dow, "*");
        assert_eq!(task.action, "/foo/bar");
        assert_eq!(task.origin, origin);

        task = &tasks[1];
        assert_eq!(task.minute, "1");
        assert_eq!(task.hour, "2-3");
        assert_eq!(task.dom, "3,4");
        assert_eq!(task.month, "4");
        assert_eq!(task.dow, "5");
        assert_eq!(task.action, "baz \"foo\" 2>&1");
        assert_eq!(task.origin, origin);
    }
}
