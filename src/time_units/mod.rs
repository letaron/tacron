pub mod days_of_month;
pub mod days_of_week;
pub mod hours;
pub mod minutes;
pub mod months;

use std::collections::BTreeSet;

#[derive(Debug)]
pub enum TimeFieldSpec {
    All,
    Unique(u8),
    NamedUnique(String),
    Range(u8, u8),
    NamedRange(String, String),
    Step(u8),
    SteppedRange(u8, u8, u8),
}

pub type TimeFieldValuesContainer = BTreeSet<u8>;

pub trait TimeUnitItem {
    fn min() -> u8;
    fn max() -> u8;
    fn name<'a>() -> &'a str;

    fn validate(value: u8) -> Result<(), String> {
        if value < Self::min() {
            return Err(format!(
                "Min for {} must be at least {}, {} given",
                Self::name(),
                Self::min(),
                value
            ));
        }

        if value > Self::max() {
            return Err(format!(
                "Max for {} must not be greater than {}, {} given",
                Self::name(),
                Self::max(),
                value
            ));
        }

        Ok(())
    }

    fn value_from_name(_name: &str) -> Result<u8, String> {
        Err(format!(
            "[WARNING] value_from_name is not valid fn for a {}",
            Self::name()
        ))
    }

    /// Extract values for the whole field configuration
    fn from_time_field_specs(
        time_field_specs: Vec<TimeFieldSpec>, source: &String,
    ) -> TimeFieldValuesContainer {
        let mut container = TimeFieldValuesContainer::new();
        for time_field_spec in time_field_specs {
            match Self::from_time_field_spec(time_field_spec) {
                Ok(values) => {
                    for value in values.iter() {
                        container.insert(*value);
                    }
                }
                Err(messsage) => println!("[WARNING] {} - {}", source, messsage),
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
                let value = Self::value_from_name(&name)?;
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
                let start = Self::value_from_name(&name_start)?;
                let end = Self::value_from_name(&name_end)?;
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
