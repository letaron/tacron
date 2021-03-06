use crate::time_units::TimeUnitItem;

pub struct DaysOfMonth {}

impl TimeUnitItem for DaysOfMonth {
    fn name<'a>() -> &'a str {
        "days of month"
    }

    fn min() -> u8 {
        1
    }

    fn max() -> u8 {
        31
    }
}

#[cfg(test)]
mod tests {
    use crate::time_units::days_of_month::DaysOfMonth;
    use crate::time_units::TimeUnitItem;

    #[test]
    fn it_has_correct_boundaries() {
        assert_eq!(DaysOfMonth::min(), 1);
        assert_eq!(DaysOfMonth::max(), 31);
    }
}
