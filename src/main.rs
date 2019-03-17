extern crate chrono;
extern crate config;
extern crate signal_hook;

mod filter;
mod reader;
mod settings;
mod time_units;

use chrono::Local;
use filter::filter_tacrons;
use reader::{get_readers, Reader};
use settings::get_settings;
use std::{
    sync::{Arc, Mutex, RwLock},
    thread, time,
};

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
    let mut tacrons: Vec<TaCron> = Vec::new();
    let mut readers: Vec<Box<Reader + Sync + Send>>;

    {
        let settings = get_settings();
        readers = get_readers(&settings);
    }

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
            println!("[SIGHUP] received, refreshing tacrons...");
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

    println!("[SIGHUP] registered");
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
