pub mod days_of_month;
pub mod days_of_week;
pub mod hours;
pub mod minutes;
pub mod months;

use std::collections::btree_set::Iter;
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum TimeFieldSpec {
    All,
    Unique(i8),
    NamedUnique(String),
    Range(i8, i8),
    NamedRange(String, String),
    Step(i8),
    SteppedRange(i8, i8, i8),
}

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
    fn name<'a>() -> &'a str;

    fn validate(value: i8) -> Result<(), String> {
        if value < Self::min() {
            return Err(format!("{} must be at least {}", value, Self::min()));
        }

        if value > Self::max() {
            return Err(format!(
                "{} must not be greater than {}",
                value,
                Self::max()
            ));
        }

        Ok(())
    }

    fn value_from_name(_name: &str) -> i8 {
        panic!(
            "[ERROR] value_from_name is not valid fn for {}",
            Self::name()
        )
    }

    /// Extract values for the whole field configuration
    fn from_time_field_specs(time_field_specs: Vec<TimeFieldSpec>) -> TimeFieldValuesContainer {
        let mut container = TimeFieldValuesContainer::new();
        for time_field_spec in time_field_specs {
            match Self::from_time_field_spec(time_field_spec) {
                Ok(values) => {
                    for value in values.iter() {
                        container.insert(*value);
                    }
                }
                Err(messsage) => println!("[ERROR] {}", messsage),
            }
        }

        container
    }

    /// Extract values for a unique TimeFieldSpec
    fn from_time_field_spec(
        time_field_spec: TimeFieldSpec,
    ) -> Result<TimeFieldValuesContainer, String> {
        let mut container = TimeFieldValuesContainer::new();
        match time_field_spec {
            TimeFieldSpec::Unique(value) => {
                Self::validate(value)?;
                container.insert(value);
            }
            TimeFieldSpec::NamedUnique(ref name) => {
                let value = Self::value_from_name(&name);
                Self::validate(value)?;
                container.insert(value);
            }
            TimeFieldSpec::Range(start, end) => {
                Self::validate(start)?;
                Self::validate(end)?;
                if start > end {
                    return Err(format!(
                        "Start for {} must not be greater than end {}, {} given",
                        Self::name(),
                        end,
                        start
                    ));
                }
                for value in start..(end + 1) {
                    container.insert(value);
                }
            }
            TimeFieldSpec::NamedRange(ref name_start, ref name_end) => {
                let start = Self::value_from_name(&name_start);
                let end = Self::value_from_name(&name_end);
                Self::validate(start)?;
                Self::validate(end)?;
                if start > end {
                    return Err(format!(
                        "Start for {} must not be greater than end {}, {} given",
                        Self::name(),
                        end,
                        start
                    ));
                }
                for value in start..(end + 1) {
                    container.insert(value);
                }
            }
            TimeFieldSpec::SteppedRange(start, end, step) => {
                Self::validate(start)?;
                Self::validate(end)?;
                if start > end {
                    return Err(format!(
                        "Start for {} must not be greater than end {}, {} given",
                        Self::name(),
                        end,
                        start
                    ));
                }
                if step < 2 || step >= (end - start) {
                    return Err(format!(
                        "Step for {} must be > 2 and < {}, {} given",
                        Self::name(),
                        end - start,
                        step
                    ));
                }
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
        Ok(container)
    }
}
