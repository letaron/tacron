extern crate regex;
use crate::{RawCron, TaCron, Reader, TimeFieldValue};
//use TaCron;
use regex::Regex;
use std::fs;
use regex::Captures;

pub struct CrontabReader {
    file: String,
}


// If regex match, return the result of function "f"
struct FieldHandler {
    regex: Regex,
    f: fn(Captures) -> TimeFieldValue,
}

fn parse_field(field: &String, field_handlers: &[&FieldHandler]) -> Vec<TimeFieldValue> {
    let mut values: Vec<TimeFieldValue> = Vec::new();

    for specifier in field.split(",") {
        for field_handler in field_handlers {
            match field_handler.regex.captures(specifier) {
                Some(x) => values.push((field_handler.f)(x)),
                None => {}
            }
        }
    }

    values
}

pub fn parse(ta_cron: &RawCron) -> TaCron {
    let all_handler = FieldHandler {
        regex: Regex::new(r"\*").unwrap(),
        f: |_capture: Captures| TimeFieldValue::All,
    };

    let unique_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::Unique(capture.get(1).unwrap().as_str().parse::<i8>().unwrap())
        },
    };

    let range_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)-([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::Range(
                capture.get(1).unwrap().as_str().parse::<i8>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<i8>().unwrap(),
            )
        },
    };

    let step_handler = FieldHandler {
        regex: Regex::new(r"^\*/([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::Step(capture.get(1).unwrap().as_str().parse::<i8>().unwrap())
        },
    };

    let stepped_range_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)-([0-9]+)/([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::SteppedRange(
                capture.get(1).unwrap().as_str().parse::<i8>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<i8>().unwrap(),
                capture.get(3).unwrap().as_str().parse::<i8>().unwrap(),
            )
        },
    };

    let named_unique_handler = FieldHandler {
        regex: Regex::new("^([a-z]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::NamedUnique(capture.get(1).unwrap().as_str().to_string())
        },
    };

    let named_range_handler = FieldHandler {
        regex: Regex::new("^([a-z]+)-([a-z]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::NamedRange(
                capture.get(1).unwrap().as_str().to_string(),
                capture.get(2).unwrap().as_str().to_string(),
            )
        },
    };

    let non_named_handlers = [
        &all_handler,
        &unique_handler,
        &range_handler,
        &step_handler,
        &stepped_range_handler,
    ];
    let named_handlers = [
        &all_handler,
        &unique_handler,
        &range_handler,
        &step_handler,
        &stepped_range_handler,
        &named_unique_handler,
        &named_range_handler,
    ];

    TaCron {
        minute: parse_field(&ta_cron.minute, &non_named_handlers),
        hour: parse_field(&ta_cron.hour, &non_named_handlers),
        dom: parse_field(&ta_cron.dom, &named_handlers),
        month: parse_field(&ta_cron.month, &named_handlers),
        dow: parse_field(&ta_cron.dow, &named_handlers),
    }
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
                        "0".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "1".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@monthly" => RawCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@weekly" => RawCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        "0".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@daily" | "@midnight" => RawCron::new(
                        "0".to_string(),
                        "0".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        "*".to_string(),
                        cron[1..].join(" "),
                        self.file.to_string(),
                    ),
                    "@hourly" => RawCron::new(
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
                tasks.push(RawCron::new(
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
