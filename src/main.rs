extern crate regex;

use std::fmt::*;

struct TaCron {
    minute: String,
    hour: String,
    dom: String,
    month: String,
    dow: String,
    action: String
}

impl Debug for TaCron {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "TaCront {{ minute: {}}}", self.minute)
    }
}

impl TaCron {
    fn new(minute: String, hour: String, dom: String, month: String, dow: String, action: String) -> TaCron {
        TaCron {minute: minute, hour: hour, dom: dom, month: month, dow: dow, action: action}
    }
}


fn ta_cron_from_crontab_line(line: String) -> TaCron {
    let re = regex::Regex::new(r"\s+").unwrap();
    let data = re.split(&line);
    let cron: Vec<&str> = data.collect();
    return TaCron::new(
        cron[0].to_string(),
        cron[1].to_string(),
        cron[2].to_string(),
        cron[3].to_string(),
        cron[4].to_string(),
        cron[5].to_string()
    );
}

fn main() {
    let cron = ta_cron_from_crontab_line("0\t2          12             *                *            /usr/bin/find".to_string());
    println!("{:?}", cron);
}
