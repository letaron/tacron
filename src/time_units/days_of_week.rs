use crate::time_units::TimeUnitItem;

pub struct DaysOfWeek {}

impl TimeUnitItem for DaysOfWeek {
    fn min() -> i8 {
        0
    }

    fn max() -> i8 {
        7
    }

    fn value_from_name(name: &str) -> i8 {
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
