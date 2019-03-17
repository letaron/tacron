use crate::time_units::TimeUnitItem;

pub struct Months {}

impl TimeUnitItem for Months {
    fn name<'a>() -> &'a str {
        "months"
    }

    fn min() -> i8 {
        1
    }

    fn max() -> i8 {
        12
    }

    fn value_from_name(name: &str) -> i8 {
        match name {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => panic!("{} is not a valid month.", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::time_units::months::Months;
    use crate::time_units::TimeUnitItem;

    #[test]
    fn it_has_correct_boundaries() {
        assert_eq!(Months::min(), 1);
        assert_eq!(Months::max(), 12);
    }

    #[test]
    fn it_has_correct_name_conversion() {
        let mut i = 1;
        for month in vec![
            "january",
            "february",
            "march",
            "april",
            "may",
            "june",
            "july",
            "august",
            "september",
            "october",
            "november",
            "december",
        ] {
            assert_eq!(Months::value_from_name(month), i);
            assert_eq!(Months::value_from_name(&month[0..3]), i);
            i += 1
        }
    }

    #[test]
    #[should_panic]
    fn it_should_panic_when_unknow() {
        Months::value_from_name("foo");
    }
}
