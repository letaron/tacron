pub mod crontab_reader;
use crate::TaCron;
use regex::Regex;
use std::collections::HashMap;

// static TIME_FIELD_PATTERNS: [&str; 2] = ["[a-z0-9]+\\-[a-z0-9]+/[0-9]+", "[a-z0-9]+\\-[a-z0-9]+"];

static UNIQUE_REGEX: &str = "^[0-9]+$";
static RANGE_REGEX: &str = "^[0-9]+-[0-9]+$";


#[derive(Debug)]
enum TimeFieldValue {
    All,
    Unique(i8),
    NamedUnique(String),
    Range(i8, i8),
    NamedRange(String, String),
    Step(i8),
    SteppedRange(),
}


#[derive(Debug)]
pub struct Task {
    minute: Vec<TimeFieldValue>
}

fn parse_unique(value: &str) -> Option<TimeFieldValue> {
    if Regex::new(UNIQUE_REGEX).unwrap().is_match(value) {
        Some(TimeFieldValue::Unique(value.parse::<i8>().unwrap()))
    } else {
        None
    }
}

fn parse_range(value: &str) -> Option<TimeFieldValue> {
    if Regex::new(RANGE_REGEX).unwrap().is_match(value) {
        let values: Vec<&str> = value.split("-").collect();
        Some(TimeFieldValue::Range(values[0].parse::<i8>().unwrap(), values[1].parse::<i8>().unwrap()))
    } else {
        None
    }
}


fn parse_field(field: &String, func_list: &Vec<fn(&str) -> Option<TimeFieldValue>>) -> Vec<TimeFieldValue> {

    let mut values: Vec<TimeFieldValue> = Vec::new();

    for splitted_field in field.split(",") {
        for func in func_list {
            match func(splitted_field) {
                Some(x) => {
                    values.push(x)
                },
                None => {}
            }
        }
    }

    values

}

pub fn parse(ta_cron: &TaCron) -> Task {

    // let mut time_field_patterns = HashMap::new();
    // time_field_patterns.insert("unique", "^[0-9]+$");
    // time_field_patterns.insert("named unique", "^[a-z]+$");
    // time_field_patterns.insert("range", "^[0-9]+-[0-9]+$");
    // time_field_patterns.insert("named range", "^[a-z]+-[a-z]+$");
    // time_field_patterns.insert("step", r"^\*/[0-9]+$");
    // time_field_patterns.insert("stepped range", "^[0-9]+-[0-9]+$");


    // let mut minutes: Vec<TimeFieldValue> = Vec::new();
    // for minute_specifier in ta_cron.minute.split(",") {

    //     if minute_specifier == "*" {
    //         minutes.push(TimeFieldValue::All);
    //         continue;
    //     }

    //     match time_field_patterns.get(&"unique") {
    //         Some(&expression) => {
    //             if Regex::new(expression).unwrap().is_match(minute_specifier) {
    //                 minutes.push(TimeFieldValue::Unique(minute_specifier.parse::<i8>().unwrap()));
    //             }
    //         },
    //         _ => panic!("\"unique\" key missing")
    //     }

    //     match time_field_patterns.get(&"range") {
    //         Some(&expression) => {
    //             if Regex::new(expression).unwrap().is_match(minute_specifier) {
    //                 let values: Vec<&str> = minute_specifier.split("-").collect();
    //                 minutes.push(TimeFieldValue::Range(values[0].parse::<i8>().unwrap(), values[1].parse::<i8>().unwrap()));
    //             }
    //         },
    //         _ => panic!("\"range\" key missing")
    //     }
        
    // }

    let mut funcs: Vec<fn(&str) -> Option<TimeFieldValue>> = Vec::new();
    funcs.push(parse_unique);
    funcs.push(parse_range);

    Task {
        minute: parse_field(&ta_cron.minute, &funcs)
    }
}
