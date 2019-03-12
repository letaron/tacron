use crate::time_units::TimeUnitItem;

pub struct DaysOfMonth {}

impl TimeUnitItem for DaysOfMonth {
    fn min() -> i8 {
        1
    }

    fn max() -> i8 {
        31
    }
}

pub fn days_in_month(month: i8) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => 28,
        _ => 31,
    }
}
