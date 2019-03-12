pub mod days_of_month;
pub mod days_of_week;
pub mod hours;
pub mod minutes;
pub mod months;

use crate::TimeFieldValue;
use std::collections::hash_set::Iter;
use std::collections::HashSet;

pub struct TimeUnitContainer {
    values: HashSet<i8>,
}

impl TimeUnitContainer {
    pub fn new() -> Self {
        TimeUnitContainer {
            values: HashSet::new(),
        }
    }

    pub fn contains(&self, value: &i8) -> bool {
        self.values.contains(value)
    }

    pub fn insert(&mut self, value: i8) {
        self.values.insert(value);
    }

    pub fn iter(&self) -> Iter<i8> {
        self.values.iter()
    }
}

pub trait TimeUnitItem {
    fn min() -> i8;
    fn max() -> i8;

    fn validate(&self, value: i8) {
        if value < Self::min() {
            panic!("{} must be at least {}", value, Self::min());
        }

        if value > Self::max() {
            panic!("{} must not be greater than {}", value, Self::max());
        }
    }

    fn value_from_name(name: &str) -> i8 {
        panic!("{} is not known.")
    }

    fn from_time_field_values(time_field_values: &Vec<TimeFieldValue>) -> TimeUnitContainer {
        let mut container = TimeUnitContainer::new();
        for time_field_value in time_field_values {
            for value in Self::from_time_field_value(time_field_value).iter() {
                container.insert(*value);
            }
        }

        container
    }

    fn from_time_field_value(time_field_value: &TimeFieldValue) -> TimeUnitContainer {
        let mut container = TimeUnitContainer::new();
        match *time_field_value {
            TimeFieldValue::Unique(value) => container.insert(value),
            TimeFieldValue::Range(start, end) => {
                for value in start..(end + 1) {
                    container.insert(value);
                }
            }
            TimeFieldValue::SteppedRange(start, end, step) => {
                let mut i = 0;
                for value in start..(end + 1) {
                    if i % step == 0 {
                        container.insert(value);
                    }
                    i += 1;
                }
            }
            TimeFieldValue::All => {
                for value in Self::min()..(Self::max() + 1) {
                    container.insert(value);
                }
            }
            // @todo add other TimeFieldValue enum
            _ => {}
        };
        container
    }
}
