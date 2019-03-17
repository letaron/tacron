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

#[cfg(test)]
mod tests {
    use crate::time_units::hours::Hours;
    use crate::time_units::TimeUnitItem;

    #[test]
    fn it_has_correct_boundaries() {
        assert_eq!(Hours::min(), 0);
        assert_eq!(Hours::max(), 23);
    }
}
