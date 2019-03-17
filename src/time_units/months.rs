use crate::time_units::TimeUnitItem;

pub struct Months {}

impl TimeUnitItem for Months {
    fn name<'a>() -> &'a str {
        "months"
    }

    fn min() -> u8 {
        1
    }

    fn max() -> u8 {
        12
    }

    fn value_from_name(name: &str) -> Result<u8, String> {
        match name {
            "jan" | "january" => Ok(1),
            "feb" | "february" => Ok(2),
            "mar" | "march" => Ok(3),
            "apr" | "april" => Ok(4),
            "may" => Ok(5),
            "jun" | "june" => Ok(6),
            "jul" | "july" => Ok(7),
            "aug" | "august" => Ok(8),
            "sep" | "september" => Ok(9),
            "oct" | "october" => Ok(10),
            "nov" | "november" => Ok(11),
            "dec" | "december" => Ok(12),
            _ => Err(format!("{} is not a valid month.", name)),
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
            assert_eq!(Months::value_from_name(month).unwrap(), i);
            assert_eq!(Months::value_from_name(&month[0..3]).unwrap(), i);
            i += 1
        }
    }

    #[test]
    fn it_should_err_when_unknow() {
        Months::value_from_name("foo").is_err();
    }
}
