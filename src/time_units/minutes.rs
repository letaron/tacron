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

#[cfg(test)]
mod tests {
    use crate::time_units::minutes::Minutes;
    use crate::time_units::TimeUnitItem;

    #[test]
    fn it_has_correct_boundaries() {
        assert_eq!(Minutes::min(), 0);
        assert_eq!(Minutes::max(), 59);
    }
}
