mod reader;
use reader::crontab_reader::CrontabReader;

#[derive(Debug)]
struct TaCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    action: String,
    origin: String
}

impl TaCron {
    fn new(minute: String, hour: String, dom: String, month: String, dow: String, action: String, origin: String) -> TaCron {
        TaCron {minute: minute, hour: hour, dom: dom, month: month, dow: dow, action: action, origin: origin}
    }
}

trait Reader {
    fn read(&self) -> Vec<TaCron>;
}

fn main() {
    let tasks_reader = CrontabReader {};
    let tasks = tasks_reader.read();
    for task in tasks {
        println!("{:?}", task);
    }
}
