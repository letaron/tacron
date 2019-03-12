use crate::time_units::TimeUnitItem;

pub struct Hours {}

impl TimeUnitItem for Hours {
    fn min() -> i8 {
        0
    }

    fn max() -> i8 {
        23
    }
}
