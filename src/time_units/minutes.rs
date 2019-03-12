use crate::time_units::TimeUnitItem;

pub struct Minutes {}

impl TimeUnitItem for Minutes {
    fn min() -> i8 {
        0
    }

    fn max() -> i8 {
        59
    }
}
