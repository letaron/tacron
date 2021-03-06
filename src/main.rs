extern crate chrono;
extern crate signal_hook;

mod config;
mod filter;
mod reader;
mod time_units;

use chrono::Local;
use config::get_config;
use filter::filter_tacrons;
use reader::{instantiate_readers, retrieve_tacrons, Reader};
use std::{
    sync::{Arc, Mutex, RwLock},
    thread, time,
};
use time_units::TimeFieldValuesContainer;

pub struct TaCron {
    minute: TimeFieldValuesContainer,
    hour: TimeFieldValuesContainer,
    dom: TimeFieldValuesContainer,
    month: TimeFieldValuesContainer,
    dow: TimeFieldValuesContainer,
    command: String,
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
    let readers: Vec<Box<Reader + Sync + Send>>;
    {
        let config = get_config(); // will be dropped at the end of the inner scope
        readers = instantiate_readers(&config.readers);
    }
    let tacrons = retrieve_tacrons(&readers);
    println!("[INFO] {} TaCrons found", tacrons.len());

    let shared_tacrons = Arc::new(RwLock::new(tacrons));

    add_sighup_handler(readers, Arc::clone(&shared_tacrons));
    main_loop(shared_tacrons);
}

/// This function refresh the readers & tacrons on SIGHUP
fn add_sighup_handler(readers: Vec<Box<Reader + Sync + Send>>, tacrons: Arc<RwLock<Vec<TaCron>>>) {
    // signal_hook::register create a thread; reader need to be shared
    let shared_reader = Arc::new(Mutex::new(readers));

    let _signal = unsafe {
        signal_hook::register(signal_hook::SIGHUP, move || {
            println!("[INFO] SIGHUP received, refreshing...");
            let mut local_readers = shared_reader.lock().unwrap();
            let mut local_tacrons = tacrons.write().unwrap();

            // @todo try to replace directly the ref but may lead to mem leaks maybe
            local_readers.clear();
            let config = get_config();
            let mut readers = instantiate_readers(&config.readers);
            local_readers.append(&mut readers);

            // @todo try to replace directly the ref but may lead to mem leaks maybe
            local_tacrons.clear();
            let mut tacrons = retrieve_tacrons(&local_readers);
            local_tacrons.append(&mut tacrons);
        })
    };

    println!("[INFO] SIGHUP registered");
}

fn main_loop(tacrons: Arc<RwLock<Vec<TaCron>>>) {
    let main_loop_handler = thread::Builder::new()
        .name("main loop".into())
        .spawn(move || loop {
            {
                // create an inner scope so tacrons is released w/o waiting the next iteration
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
