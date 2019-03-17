use crate::time_units::TimeUnitItem;

pub struct Hours {}

impl TimeUnitItem for Hours {
    fn name<'a>() -> &'a str {
        "hours"
    }
    fn min() -> u8 {
        0
    }

    fn max() -> u8 {
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
