use crate::time_units::TimeUnitItem;

pub struct Hours {}

impl TimeUnitItem for Hours {
    fn min(&self) -> i8 {
        0
    }

    fn max(&self) -> i8 {
        23
    }
}
