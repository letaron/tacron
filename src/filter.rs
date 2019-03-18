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
        tacron.dow.contains(&(current_dow as u8))
            && tacron.month.contains(&(current_month as u8))
            && tacron.dom.contains(&(current_dom as u8))
            && tacron.hour.contains(&(current_hour as u8))
            && tacron.minute.contains(&(current_minute as u8))
    })
}
