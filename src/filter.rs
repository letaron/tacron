use crate::time_units::days_of_month::DaysOfMonth;
use crate::time_units::days_of_week::DaysOfWeek;
use crate::time_units::hours::Hours;
use crate::time_units::minutes::Minutes;
use crate::time_units::months::Months;
use crate::time_units::TimeUnitItem;
use crate::TaCron;
use chrono::{Date, DateTime, Datelike, Local, Timelike};

pub fn filter_tacrons(
    tacrons: &Vec<TaCron>, today: Date<Local>, now: DateTime<Local>,
) -> impl Iterator<Item = &TaCron> {
    let (current_dow, current_month, current_dom, current_hour, current_minute) = (
        today.weekday().num_days_from_sunday(),
        today.month(),
        today.day(),
        now.hour(),
        now.minute(),
    );

    println!(
        "\nCurrent dow: {:02}, month: {:02}, dom: {:02}, hours: {:02}, minutes: {:02}",
        current_dow, current_month, current_dom, current_hour, current_minute
    );

    tacrons.iter().filter(move |tacron| {
        let (cron_minutes, cron_hours, cron_dom, cron_months, cron_dow) = (
            Minutes::from_time_field_specs(&tacron.minute),
            Hours::from_time_field_specs(&tacron.hour),
            DaysOfMonth::from_time_field_specs(&tacron.dom),
            Months::from_time_field_specs(&tacron.month),
            DaysOfWeek::from_time_field_specs(&tacron.dow),
        );

        cron_dow.contains(&(current_dow as i8))
            && cron_months.contains(&(current_month as i8))
            && cron_dom.contains(&(current_dom as i8))
            && cron_hours.contains(&(current_hour as i8))
            && cron_minutes.contains(&(current_minute as i8))
    })
}
