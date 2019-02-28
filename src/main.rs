extern crate regex;

//struct TaCron {
//    minute: i8,
//    hour: i8,
//    dom: i8,
//    month: i8,
//    dow: i8,
//    action: String
//}


fn ta_cron_from_crontab_line(line: String) {
    let re = regex::Regex::new(r"[ ]+").unwrap();
    let mut data = re.split(&line);
    println!("{:?}", data.next());
    println!("{:?}", data.next());
}

fn main() {
    ta_cron_from_crontab_line("0        2          12             *                *            /usr/bin/find".to_string())
}
