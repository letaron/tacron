use crate::time_units::TimeUnitItem;

pub struct Months {}

impl TimeUnitItem for Months {
    fn min() -> i8 {
        1
    }

    fn max() -> i8 {
        12
    }

    fn value_from_name(name: &str) -> i8 {
        match name {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => panic!("{} is not a valid month.", name),
        }
    }
}
