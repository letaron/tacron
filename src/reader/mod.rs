pub mod crontab_reader;
use crate::TaCron;
use regex::Regex;
use regex::Captures;

// static TIME_FIELD_PATTERNS: [&str; 2] = ["[a-z0-9]+\\-[a-z0-9]+/[0-9]+", "[a-z0-9]+\\-[a-z0-9]+"];


#[derive(Debug)]
enum TimeFieldValue {
    // All,
    Unique(i8),
    // NamedUnique(String),
    Range(i8, i8),
    // NamedRange(String, String),
    Step(i8),
    SteppedRange(i8, i8, i8),
}


struct FieldHandler {
    regex: Regex,
    f: fn(Captures) -> TimeFieldValue
}


#[derive(Debug)]
pub struct Task {
    minute: Vec<TimeFieldValue>,
    hour: Vec<TimeFieldValue>,
}


fn parse_field(field: &String, field_handlers: &Vec<FieldHandler>) -> Vec<TimeFieldValue> {

    let mut values: Vec<TimeFieldValue> = Vec::new();

    for splitted_field in field.split(",") {
        for field_handler in field_handlers {
            let captures = field_handler.regex.captures(splitted_field);
            match captures {
                Some(x) => {
                    values.push((field_handler.f)(x))
                },
                None => {}
            }
        }
    }

    values
}


pub fn parse(ta_cron: &TaCron) -> Task {

    // time_field_patterns.insert("named unique", "^[a-z]+$");
    // time_field_patterns.insert("named range", "^[a-z]+-[a-z]+$");

    let unique_handler = FieldHandler { regex: Regex::new("^([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldValue::Unique(capture.get(1).unwrap().as_str().parse::<i8>().unwrap())
    }};

    let range_handler = FieldHandler { regex: Regex::new("^([0-9]+)-([0-9]+)$").unwrap(),
        f: |capture: Captures| {TimeFieldValue::Range(capture.get(1).unwrap().as_str().parse::<i8>().unwrap(), capture.get(2).unwrap().as_str().parse::<i8>().unwrap())
    }};

    let step_handler = FieldHandler { regex: Regex::new(r"^\*/([0-9]+)$").unwrap(),
        f: |capture: Captures| {TimeFieldValue::Step(capture.get(1).unwrap().as_str().parse::<i8>().unwrap())
    }};

    let stepped_range_handler = FieldHandler { regex: Regex::new("^([0-9]+)-([0-9]+)/([0-9]+)$").unwrap(),
        f: |capture: Captures| {TimeFieldValue::SteppedRange(capture.get(1).unwrap().as_str().parse::<i8>().unwrap(), capture.get(2).unwrap().as_str().parse::<i8>().unwrap(), capture.get(3).unwrap().as_str().parse::<i8>().unwrap())
    }};

    let mut non_named_handlers: Vec<FieldHandler> = Vec::new();
    non_named_handlers.push(unique_handler);
    non_named_handlers.push(range_handler);
    non_named_handlers.push(step_handler);
    non_named_handlers.push(stepped_range_handler);

    Task {
        minute: parse_field(&ta_cron.minute, &non_named_handlers),
        hour: parse_field(&ta_cron.hour, &non_named_handlers)
    }
}