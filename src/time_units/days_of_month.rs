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

