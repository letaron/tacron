extern crate chrono;
mod reader;
mod time_units;
use chrono::{Datelike, Local, Timelike};
use reader::crontab_reader::CrontabReader;
use reader::parse;
use std::thread;
use std::time;
use time_units::days_of_month::DaysOfMonth;
use time_units::days_of_week::DaysOfWeek;
use time_units::hours::Hours;
use time_units::minutes::Minutes;
use time_units::months::Months;
use time_units::TimeUnitItem;

// Represent a not-yet parsed line of a crontab
#[derive(Debug)]
pub struct RawCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    command: String,
    source: String,
}

#[derive(Debug)]
pub enum TimeFieldSpec {
    All,
    Unique(i8),
    NamedUnique(String),
    Range(i8, i8),
    NamedRange(String, String),
    Step(i8),
    SteppedRange(i8, i8, i8),
}

#[derive(Debug)]
pub struct TaCron {
    pub minute: Vec<TimeFieldSpec>,
    pub hour: Vec<TimeFieldSpec>,
    pub dom: Vec<TimeFieldSpec>,
    pub month: Vec<TimeFieldSpec>,
    pub dow: Vec<TimeFieldSpec>,
    pub command: String,
    pub source: String,
}

impl RawCron {
    fn new(
        minute: String, hour: String, dom: String, month: String, dow: String, command: String,
        source: String,
    ) -> RawCron {
        RawCron {
            minute,
            hour,
            dom,
            month,
            dow,
            command,
            source,
        }
    }
}

trait Reader {
    fn read(&self) -> Vec<RawCron>;

    fn tacrons(&self) -> Vec<TaCron> {
        let raw_crons = self.read();
        let mut tacrons = Vec::new();
        for raw_cron in raw_crons {
            tacrons.push(parse(&raw_cron));
        }
        tacrons
    }
}

fn execute(tacrons: &Vec<TaCron>) {

    let filtered = filter(tacrons);

    for tacron in filtered {
        println!("{:?}", tacron);
    }

}

fn filter(tacrons: &Vec<TaCron>) -> Vec<&TaCron> {
    let (today, now) = (Local::today(), Local::now());

    let mut filtered = Vec::new();

    println!(
        "\nThe current local datetime is: dow: {:02}, month: {:02}, dom: {:02}, hours: {:02}, minutes: {:02}",
        today.weekday().num_days_from_sunday(),
        today.month(),
        today.day(),
        now.hour(),
        now.minute()
    );

    for tacron in tacrons {
        println!("{:?}", tacron);

        let minutes = Minutes::from_time_field_specs(&tacron.minute);
        let hours = Hours::from_time_field_specs(&tacron.hour);
        let dom = DaysOfMonth::from_time_field_specs(&tacron.dom);
        let months = Months::from_time_field_specs(&tacron.month);
        let dow = DaysOfWeek::from_time_field_specs(&tacron.dow);

        // println!("minutes: {:?}", minutes.iter());
        // println!("hours: {:?}", hours.iter());
        // println!("dom: {:?}", dom.iter());
        // println!("months: {:?}", months.iter());
        // println!("dow: {:?}", dow.iter());

        if !dow.contains(&(today.weekday().num_days_from_sunday() as i8))
            || !months.contains(&(today.month() as i8))
            || !dom.contains(&(today.day() as i8))
            || !hours.contains(&(now.hour() as i8))
            || !minutes.contains(&(now.minute() as i8))
        {
            continue;
        }

        filtered.push(tacron);

        println!("exec {:?}", tacron.command);
    }

    filtered
}

fn main() {
    let reader = CrontabReader::new("fixtures/crontab".to_string());
    let tacrons = reader.tacrons();

    let main_loop_handler = thread::Builder::new()
        .name("main loop".into())
        .spawn(move || loop {
            execute(&tacrons);
            thread::sleep(time::Duration::from_millis(10000));
        })
        .unwrap();

    main_loop_handler.join().unwrap();
}
