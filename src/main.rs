extern crate chrono;
mod reader;
// use chrono::{Local, Timelike};
use reader::crontab_reader::CrontabReader;

// Represent a not-yet parsed line of a crontab
#[derive(Debug)]
pub struct TaCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    action: String,
    origin: String,
}

impl TaCron {
    fn new(
        minute: String, hour: String, dom: String, month: String, dow: String, action: String,
        origin: String,
    ) -> TaCron {
        TaCron {
            minute: minute,
            hour: hour,
            dom: dom,
            month: month,
            dow: dow,
            action: action,
            origin: origin,
        }
    }
}

trait Reader {
    fn read(&self) -> Vec<TaCron>;
}

// fn execution_filter(ta_crons: &Vec<TaCron>) {
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
    let ta_crons = reader.read();

    for ta_cron in ta_crons {
        println!("{:?}", ta_cron);
        let task = reader::parse(&ta_cron);
        println!("{:?}", task);
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
