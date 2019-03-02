mod reader;
use reader::crontab_reader::CrontabReader;
use std::{thread, time};

#[derive(Debug)]
struct TaCron {
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

fn execution_filter(tasks: &Vec<TaCron>) {
    for task in tasks {
        println!("{:?}", task);
    }
}

fn main() {
    let tasks_reader = CrontabReader::new("fixtures/crontab".to_string());
    let tasks = tasks_reader.read();

    let main_loop_handler = thread::Builder::new()
        .name("main loop".into())
        .spawn(move || loop {
            execution_filter(&tasks);
            thread::sleep(time::Duration::from_millis(1000));
        })
        .unwrap();

    main_loop_handler.join().unwrap();
}
