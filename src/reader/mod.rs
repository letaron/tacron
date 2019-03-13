pub mod crontab_reader;
use crate::{RawCron, TaCron, TimeFieldSpec};
use regex::{Captures, Regex};

// If regex match, return the result of function "f"
struct FieldHandler {
    regex: Regex,
    f: fn(Captures) -> TimeFieldSpec,
}

fn parse_field(field: &String, field_handlers: &[&FieldHandler]) -> Vec<TimeFieldSpec> {
    let mut values: Vec<TimeFieldSpec> = Vec::new();

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
        regex: Regex::new(r"^(\*)$").unwrap(),
        f: |_capture: Captures| TimeFieldSpec::All,
    };

    let unique_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::Unique(capture.get(1).unwrap().as_str().parse::<i8>().unwrap())
        },
    };

    let range_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)-([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::Range(
                capture.get(1).unwrap().as_str().parse::<i8>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<i8>().unwrap(),
            )
        },
    };

    let step_handler = FieldHandler {
        regex: Regex::new(r"^\*/([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::Step(capture.get(1).unwrap().as_str().parse::<i8>().unwrap())
        },
    };

    let stepped_range_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)-([0-9]+)/([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::SteppedRange(
                capture.get(1).unwrap().as_str().parse::<i8>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<i8>().unwrap(),
                capture.get(3).unwrap().as_str().parse::<i8>().unwrap(),
            )
        },
    };

    let named_unique_handler = FieldHandler {
        regex: Regex::new("^([a-z]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::NamedUnique(capture.get(1).unwrap().as_str().to_string())
        },
    };

    let named_range_handler = FieldHandler {
        regex: Regex::new("^([a-z]+)-([a-z]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::NamedRange(
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
