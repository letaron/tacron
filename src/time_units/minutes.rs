use crate::time_units::TimeUnitItem;

pub struct Minutes {}

impl TimeUnitItem for Minutes {
    fn min(&self) -> i8 {
        0
    }

    fn max(&self) -> i8 {
        59
    }
}
