use crate::time_units::TimeUnitItem;

pub struct DaysOfWeek {}

impl TimeUnitItem for DaysOfWeek {
    fn name<'a>() -> &'a str {
        "days of week"
    }

    fn min() -> u8 {
        0
    }

    fn max() -> u8 {
        7
    }

    fn value_from_name(name: &str) -> u8 {
        match name {
            "sun" | "sunday" => 0,
            "mon" | "monday" => 1,
            "tue" | "tuesday" => 2,
            "wed" | "wednesday" => 3,
            "thu" | "thursday" => 4,
            "fri" | "friday" => 5,
            "sat" | "saturday" => 6,
            _ => panic!("{} is not a valid day of week.", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time_units::days_of_week::DaysOfWeek;
    use crate::time_units::TimeUnitItem;

    #[test]
    fn it_has_correct_boundaries() {
        assert_eq!(DaysOfWeek::min(), 0);
        assert_eq!(DaysOfWeek::max(), 7);
    }

    #[test]
    fn it_has_correct_name_conversion() {
        let mut i = 0;
        for day in vec![
            "sunday",
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
        ] {
            assert_eq!(DaysOfWeek::value_from_name(day), i);
            assert_eq!(DaysOfWeek::value_from_name(&day[0..3]), i);
            i += 1
        }
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_unknow() {
        DaysOfWeek::value_from_name("foo");
    }
}
