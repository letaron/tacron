extern crate chrono;
extern crate signal_hook;
mod reader;
mod time_units;
use chrono::{Date, DateTime, Datelike, Local, Timelike};
use reader::{crontab_reader::CrontabReader, Reader};
use std::{
    sync::{Arc, Mutex, RwLock},
    thread, time,
};
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
    let mut readers: Vec<Box<Reader + Sync + Send>> = Vec::new();
    let mut tacrons: Vec<TaCron> = Vec::new();

    readers.push(Box::new(CrontabReader::new("fixtures/crontab".to_string())));
    readers.push(Box::new(CrontabReader::new(
        "fixtures/another_crontab".to_string(),
    )));

    for reader in &readers {
        let mut reader_tacrons = reader.tacrons();
        tacrons.append(&mut reader_tacrons)
    }

    let shared_tacrons = Arc::new(RwLock::new(tacrons));

    add_sighup_handler(readers, Arc::clone(&shared_tacrons));
    main_loop(shared_tacrons);
}

/// This function refresh the tacrons on SIGHUP
fn add_sighup_handler(readers: Vec<Box<Reader + Sync + Send>>, tacrons: Arc<RwLock<Vec<TaCron>>>) {
    // signal_hook::register create a thread; reader need to be shared
    let shared_reader = Arc::new(Mutex::new(readers));

    let _signal = unsafe {
        signal_hook::register(signal_hook::SIGHUP, move || {
            println!("SIGHUP received, refreshing tacrons...");
            let local_readers = shared_reader.lock().unwrap();
            let mut local_tacrons = tacrons.write().unwrap();

            local_tacrons.clear();
            for local_reader in local_readers.iter() {
                // @todo replace "as a ref" maybe, but may lead to memory leak ?
                for tacron in local_reader.tacrons() {
                    local_tacrons.push(tacron);
                }
            }
        })
    };
}

fn main_loop(tacrons: Arc<RwLock<Vec<TaCron>>>) {
    let main_loop_handler = thread::Builder::new()
        .name("main loop".into())
        .spawn(move || loop {
            {
                let (today, now) = (Local::today(), Local::now());
                let local_tacrons = tacrons.read().unwrap();
                let filtered = filter_tacrons(&local_tacrons, today, now);

                for tacron in filtered {
                    exec_command(tacron.command.clone())
                }
            }

            thread::sleep(time::Duration::from_millis(5000));
        })
        .unwrap();

    main_loop_handler.join().unwrap();
}
