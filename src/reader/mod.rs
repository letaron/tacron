pub mod crontab_reader;
pub mod file_reader;
use crate::time_units::days_of_month::DaysOfMonth;
use crate::time_units::days_of_week::DaysOfWeek;
use crate::time_units::hours::Hours;
use crate::time_units::minutes::Minutes;
use crate::time_units::months::Months;
use crate::time_units::{TimeFieldSpec, TimeUnitItem};
use crate::TaCron;
use crontab_reader::instantiate_crontab_readers;
use file_reader::instantiate_file_readers;
use regex::{Captures, Regex};
use std::collections::HashMap;

pub trait Reader {
    fn raw_crons(&self) -> Vec<RawCron>;

    fn tacrons(&self) -> Vec<TaCron> {
        let raw_crons = self.raw_crons();
        let mut tacrons = Vec::new();
        for raw_cron in raw_crons {
            tacrons.push(parse(&raw_cron));
        }
        tacrons
    }
}

pub fn instantiate_readers(
    config: &HashMap<String, Vec<String>>,
) -> Vec<Box<Reader + Sync + Send>> {
    let mut readers: Vec<Box<Reader + Sync + Send>> = Vec::new();

    let mut type_instantiator_mapping: Vec<(
        &str,
        fn(&mut Vec<Box<(Reader + Send + Sync)>>, &Vec<String>),
    )> = Vec::new();
    type_instantiator_mapping.push(("files", instantiate_file_readers));
    type_instantiator_mapping.push(("crontabs", instantiate_crontab_readers));

    for (r#type, instantiator) in type_instantiator_mapping {
        if let Some(values) = config.get(r#type) {
            instantiator(&mut readers, values);
        }
    }
    readers
}

/// Compute all tacrons for all readers
pub fn retrieve_tacrons(readers: &Vec<Box<Reader + Sync + Send>>) -> Vec<TaCron> {
    let mut tacrons: Vec<TaCron> = Vec::new();
    for reader in readers {
        let mut reader_tacrons = reader.tacrons();
        tacrons.append(&mut reader_tacrons)
    }

    tacrons
}

// Represent a not-yet parsed line of a crontab
#[derive(Debug)]
pub struct RawCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    command: String,
    source: String,
}

impl RawCron {
    fn new(
        minute: String, hour: String, dom: String, month: String, dow: String, command: String,
        source: String,
    ) -> RawCron {
        RawCron {
            minute,
            hour,
            dom,
            month,
            dow,
            command,
            source,
        }
    }
}

// If regex match, return the result of function "f"
struct FieldHandler {
    regex: Regex,
    f: fn(Captures) -> TimeFieldSpec,
}

fn parse_field(field: &String, field_handlers: &[&FieldHandler]) -> Vec<TimeFieldSpec> {
    let mut values: Vec<TimeFieldSpec> = Vec::new();

    for specifier in field.split(",") {
        for field_handler in field_handlers {
            if let Some(x) = field_handler.regex.captures(specifier) {
                values.push((field_handler.f)(x));
            }
        }
    }

    values
}

fn parse(ta_cron: &RawCron) -> TaCron {
    let all_handler = FieldHandler {
        regex: Regex::new(r"^(\*)$").unwrap(),
        f: |_capture: Captures| TimeFieldSpec::All,
    };

    let unique_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::Unique(capture.get(1).unwrap().as_str().parse::<u8>().unwrap())
        },
    };

    let range_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)-([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::Range(
                capture.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<u8>().unwrap(),
            )
        },
    };

    let step_handler = FieldHandler {
        regex: Regex::new(r"^\*/([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::Step(capture.get(1).unwrap().as_str().parse::<u8>().unwrap())
        },
    };

    let stepped_range_handler = FieldHandler {
        regex: Regex::new("^([0-9]+)-([0-9]+)/([0-9]+)$").unwrap(),
        f: |capture: Captures| {
            TimeFieldSpec::SteppedRange(
                capture.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                capture.get(3).unwrap().as_str().parse::<u8>().unwrap(),
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

    let source = ta_cron.source.clone();

    TaCron {
        minute: Minutes::from_time_field_specs(
            parse_field(&ta_cron.minute, &non_named_handlers),
            &source,
        ),
        hour: Hours::from_time_field_specs(
            parse_field(&ta_cron.hour, &non_named_handlers),
            &source,
        ),
        dom: DaysOfMonth::from_time_field_specs(
            parse_field(&ta_cron.dom, &named_handlers),
            &source,
        ),
        month: Months::from_time_field_specs(parse_field(&ta_cron.month, &named_handlers), &source),
        dow: DaysOfWeek::from_time_field_specs(parse_field(&ta_cron.dow, &named_handlers), &source),
        command: ta_cron.command.clone(),
    }
}

#[cfg(test)]
mod tests {

    use super::instantiate_readers;
    use std::collections::HashMap;

    #[test]
    fn there_is_readers_for_files() {
        let mut config: HashMap<String, Vec<String>> = HashMap::new();
        config.insert(
            "files".to_string(),
            vec!["crontab/foo".to_string(), "crontab/bar".to_string()],
        );
        let readers = instantiate_readers(&config);
        assert_eq!(readers.len(), 2);
    }

    #[test]
    fn there_is_no_readers() {
        let mut config: HashMap<String, Vec<String>> = HashMap::new();
        config.insert(
            "foo".to_string(),
            vec!["foo/bar".to_string(), "foo/baz".to_string()],
        );
        let readers = instantiate_readers(&config);
        assert_eq!(readers.len(), 0);
    }
}
