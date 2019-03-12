pub mod days_of_month;
pub mod days_of_week;
pub mod hours;
pub mod minutes;
pub mod months;

pub trait TimeUnitItem {
    fn min(&self) -> i8;
    fn max(&self) -> i8;

    fn validate(&self, value: i8) {
        if value < self.min() {
            panic!("{} must be at least {}", value, self.min());
        }

        if value > self.max() {
            panic!("{} must not be greater than {}", value, self.min());
        }
    }

    fn value_from_name(name: &str) -> i8 {
        panic!("{} is not known.")
    }
}
