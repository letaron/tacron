extern crate regex;

#[derive(Debug)]
struct TaCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    action: String
}

trait Reader {
    fn read() -> Vec<TaCron>;
}

impl TaCron {
    fn new(minute: String, hour: String, dom: String, month: String, dow: String, action: String) -> TaCron {
        TaCron {minute: minute, hour: hour, dom: dom, month: month, dow: dow, action: action}
    }
}

struct CrontabReader {
}

impl Reader for CrontabReader {
    fn read() -> Vec<TaCron> {
        let line = "0\t2          12             *                *            /usr/bin/find".to_string();
        let re = regex::Regex::new(r"\s+").unwrap();
        let data = re.split(&line);
        let cron: Vec<&str> = data.collect();
        let mut tasks: Vec<TaCron> = Vec::new();

        tasks.push( 
            TaCron::new(
                cron[0].to_string(),
                cron[1].to_string(),
                cron[2].to_string(),
                cron[3].to_string(),
                cron[4].to_string(),
                cron[5].to_string()
                )
            );

        tasks
    }
}

fn main() {
    let tasks = CrontabReader::read();
    for task in tasks {
        println!("{:?}", task);
    }
}
