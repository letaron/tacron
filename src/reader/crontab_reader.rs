extern crate regex;
use regex::Regex;
use crate::TaCron;
use crate::Reader;

pub struct CrontabReader {
}

impl Reader for CrontabReader {
    fn read(&self) -> Vec<TaCron> {
        let mut tasks: Vec<TaCron> = Vec::new();
        let lines = "0\t2 12 * *  /usr/bin/find\n1 2 3 4 5 ls\n#comment".to_string();

        let comment_regex = Regex::new(r"^\s?#").unwrap();

        for line in lines.split("\n") {

            if comment_regex.is_match(line) == true {
                continue;
            }

            tasks.push(
                tacron_from_crontab_line(line.to_string())
                );
        }

        tasks

    }
}

fn tacron_from_crontab_line(line: String) -> TaCron {
    let re = Regex::new(r"\s+").unwrap();
    let data = re.split(&line);
    let cron: Vec<&str> = data.collect();
    TaCron::new(
        cron[0].to_string(),
        cron[1].to_string(),
        cron[2].to_string(),
        cron[3].to_string(),
        cron[4].to_string(),
        cron[5].to_string(),
        "origin".to_string()
        )
}

