use crate::time_units::TimeUnitItem;

pub struct Minutes {}

impl TimeUnitItem for Minutes {
    fn name<'a>() -> &'a str {
        "minutes"
    }

    fn min() -> u8 {
        0
    }

    fn max() -> u8 {
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
