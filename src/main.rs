extern crate chrono;
extern crate signal_hook;
mod reader;
mod time_units;
use chrono::{Date, DateTime, Datelike, Local, Timelike};
use reader::crontab_reader::CrontabReader;
use reader::Reader;
// use std::io::{self, Write};
// use std::process::Command;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time;
use time_units::days_of_month::DaysOfMonth;
use time_units::days_of_week::DaysOfWeek;
use time_units::hours::Hours;
use time_units::minutes::Minutes;
use time_units::months::Months;
use time_units::TimeUnitItem;

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

fn filter_tacrons(
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

fn main_loop() {
    let reader = CrontabReader::new("fixtures/crontab".to_string());
    let tacrons = reader.tacrons();
    let boxed_reader: Box<Reader + Sync + Send> = Box::new(reader);
    let shared_reader = Arc::new(RwLock::new(boxed_reader));

    let shared_tacrons = Arc::new(RwLock::new(tacrons));
    let sig_tacrons = Arc::clone(&shared_tacrons);

    let _signal = unsafe {
        signal_hook::register(signal_hook::SIGHUP, move || {
            println!("SIGHUP catched, rereshing tacrons from reader");
            let local_reader = shared_reader.read().unwrap();
            let mut local_tacrons = sig_tacrons.write().unwrap();
            local_tacrons.clear();
            for tacron in local_reader.tacrons() {
                local_tacrons.push(tacron);
            }
        })
    };

    let main_loop_handler = thread::Builder::new()
        .name("main loop".into())
        .spawn(move || {
            loop {
                {
                    let (today, now) = (Local::today(), Local::now());
                    let local_tacrons = shared_tacrons.read().unwrap();
                    let filtered = filter_tacrons(&local_tacrons, today, now);

                    for tacron in filtered {
                        exec_command(tacron.command.clone())
                    }
                }

                thread::sleep(time::Duration::from_millis(10000));
            }
        })
        .unwrap();

    main_loop_handler.join().unwrap();
}

fn exec_command(command: String) {
    println!("Would have executed: {}", command);
    /*
    thread::Builder::new()
        .spawn(|| {
            // dirty trick to execute the command
            // otherwise we need to parse the command line to distinguate command form args
            // ie. Command::new("sleep 10") will not work as it look out for a command named "sleep 10"
            let _output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process");

            // println!("status: {}", output.status);
            // io::stdout().write_all(&output.stdout).unwrap();
            // io::stderr().write_all(&output.stderr).unwrap();
        })
        .unwrap();
        */
}

fn main() {
    main_loop();
}
