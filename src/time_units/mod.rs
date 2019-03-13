pub mod days_of_month;
pub mod days_of_week;
pub mod hours;
pub mod minutes;
pub mod months;

use crate::TimeFieldSpec;
use std::collections::btree_set::Iter;
use std::collections::BTreeSet;

pub struct TimeFieldValuesContainer {
    values: BTreeSet<i8>,
}

impl TimeFieldValuesContainer {
    pub fn new() -> Self {
        TimeFieldValuesContainer {
            values: BTreeSet::new(),
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
        panic!("{} is not known.", name)
    }

    /// Extract values for the whole field configuration
    fn from_time_field_values(time_field_values: &Vec<TimeFieldSpec>) -> TimeFieldValuesContainer {
        let mut container = TimeFieldValuesContainer::new();
        for time_field_value in time_field_values {
            for value in Self::from_time_field_value(time_field_value).iter() {
                container.insert(*value);
            }
        }

        container
    }

    /// Extract values for a unique TimeFieldSpec
    fn from_time_field_value(time_field_value: &TimeFieldSpec) -> TimeFieldValuesContainer {
        let mut container = TimeFieldValuesContainer::new();
        match *time_field_value {
            TimeFieldSpec::Unique(value) => container.insert(value),
            TimeFieldSpec::NamedUnique(ref name) => {
                container.insert(Self::value_from_name(&name));
            }
            TimeFieldSpec::Range(start, end) => {
                for value in start..(end + 1) {
                    container.insert(value);
                }
            }
            TimeFieldSpec::NamedRange(ref name_start, ref name_end) => {
                for value in
                    Self::value_from_name(&name_start)..(Self::value_from_name(&name_end) + 1)
                {
                    container.insert(value);
                }
            }
            TimeFieldSpec::SteppedRange(start, end, step) => {
                for value in (start..(end + 1)).step_by(step as usize) {
                    container.insert(value);
                }
            }
            TimeFieldSpec::Step(step) => {
                for value in (Self::min()..(Self::max() + 1)).step_by(step as usize) {
                    container.insert(value);
                }
            }
            TimeFieldSpec::All => {
                for value in Self::min()..(Self::max() + 1) {
                    container.insert(value);
                }
            }
        };
        container
    }
}
