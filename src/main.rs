extern crate chrono;
mod reader;
mod time_units;
// use chrono::{Local, Timelike};
use reader::crontab_reader::CrontabReader;
use reader::parse;
use time_units::minutes::Minutes;
use time_units::TimeUnitItem;

// Represent a not-yet parsed line of a crontab
#[derive(Debug)]
pub struct RawCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    action: String,
    origin: String,
}

#[derive(Debug)]
pub enum TimeFieldValue {
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
    pub minute: Vec<TimeFieldValue>,
    pub hour: Vec<TimeFieldValue>,
    pub dom: Vec<TimeFieldValue>,
    pub month: Vec<TimeFieldValue>,
    pub dow: Vec<TimeFieldValue>,
}

impl RawCron {
    fn new(
        minute: String, hour: String, dom: String, month: String, dow: String, action: String,
        origin: String,
    ) -> RawCron {
        RawCron {
            minute,
            hour,
            dom,
            month,
            dow,
            action,
            origin,
        }
    }
}

trait Reader {
    fn read(&self) -> Vec<RawCron>;

    fn tacrons(&self) -> Vec<TaCron> {
        let raw_crons = self.read();
        let mut ta_crons = Vec::new();
        for raw_cron in raw_crons {
            ta_crons.push(parse(&raw_cron));
        }
        ta_crons
    }
}

// fn execution_filter(ta_crons: &Vec<RawCron>) {
//     for ta_cron in ta_crons {
//         println!("{:?}", ta_crons);

//         let task = reader::parse(ta_cron);
//         println!("{:?}", task);
//     }

//     let now = Local::now();

//     println!(
//         "The current Local time is {:02}:{:02}:{:02}",
//         now.hour(),
//         now.minute(),
//         now.second()
//     );
// }

fn main() {
    let reader = CrontabReader::new("fixtures/crontab".to_string());
    let tacrons = reader.tacrons();

    for tacron in tacrons {
        println!("{:?}", tacron);

        let minutes = Minutes::from_time_field_values(&tacron.minute);
        println!("{:?}", minutes.iter());
    }

    // let main_loop_handler = thread::Builder::new()
    //     .name("main loop".into())
    //     .spawn(move || loop {
    //         execution_filter(&tasks);
    //         thread::sleep(time::Duration::from_millis(10000));
    //     })
    //     .unwrap();

    // main_loop_handler.join().unwrap();
}
