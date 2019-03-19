extern crate regex;
use crate::reader::RawCron;
use crate::Reader;
use regex::Regex;
use std::fs;

pub struct FileReader {
    file: String,
}

impl FileReader {
    pub fn new(file: String) -> Self {
        FileReader { file }
    }
}

pub fn parse_content(content: String, name: &String) -> Vec<RawCron> {
    let mut tasks: Vec<RawCron> = Vec::new();
    let line_is_comment_regex = Regex::new(r"^\s*#").unwrap();
    let line_split_regex = Regex::new(r"\s+").unwrap();

    let mut line_number = 0;
    for line in content.split("\n") {
        line_number += 1;
        let source = format!("file_reader@{}:{}", name, line_number);

        if line.len() == 0 || line_is_comment_regex.is_match(line) {
            continue;
        }

        let cron: Vec<&str> = line_split_regex.split(line).collect();

        let times_specs;
        let command_index;

        if line.chars().next() == Some('@') {
            command_index = 1;
            times_specs = match cron[0] {
                "@yearly" | "@annually" => ["0", "0", "1", "1", "*"],
                "@monthly" => ["0", "0", "1", "*", "*"],
                "@weekly" => ["0", "0", "*", "*", "0"],
                "@daily" | "@midnight" => ["0", "0", "*", "*", "*"],
                "@hourly" => ["0", "*", "*", "*", "*"],
                x => {
                    println!(
                        "{}",
                        format!("[WARNING] {} - Invalid special value: {}", source, x)
                    );
                    continue;
                }
            };
        } else {
            command_index = 5;
            times_specs = [cron[0], cron[1], cron[2], cron[3], cron[4]]
        }

        tasks.push(RawCron::new(
            times_specs[0].to_string(),
            times_specs[1].to_string(),
            times_specs[2].to_string(),
            times_specs[3].to_string(),
            times_specs[4].to_string(),
            cron[command_index..].join(" ").to_string(),
            source,
        ));
    }

    tasks
}

impl Reader for FileReader {
    fn raw_crons(&self) -> Vec<RawCron> {
        let content = fs::read_to_string(&self.file).expect("Unable to read file");

        parse_content(content, &self.file)
    }
}

/// Push a reader in `readers` for each `crontabs` file
pub fn instantiate_file_readers(
    readers: &mut Vec<Box<Reader + Sync + Send>>, crontabs: &Vec<String>,
) {
    for crontab in crontabs {
        println!("[READER] file - loading: {:?}", crontab);
        readers.push(Box::new(FileReader::new(crontab.to_string())));
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::file_reader::{instantiate_file_readers, FileReader};
    use crate::Reader;

    #[test]
    fn it_reads() {
        let source = "fixtures/crontab";
        let reader = FileReader {
            file: source.to_string(),
        };

        let tasks = reader.raw_crons();
        assert_eq!(tasks.len(), 4);

        let task = &tasks[0];
        assert_eq!(task.minute, "0");
        assert_eq!(task.hour, "1");
        assert_eq!(task.dom, "2");
        assert_eq!(task.month, "*");
        assert_eq!(task.dow, "*");
        assert_eq!(task.command, "/foo/bar");
        assert_eq!(task.source, "file_reader@fixtures/crontab:5");

        let task = &tasks[1];
        assert_eq!(task.minute, "1");
        assert_eq!(task.hour, "2-3");
        assert_eq!(task.dom, "3,4");
        assert_eq!(task.month, "4");
        assert_eq!(task.dow, "5");
        assert_eq!(task.command, "baz \"foo\" 2>&1");
        assert_eq!(task.source, "file_reader@fixtures/crontab:6");
    }

    #[test]
    fn it_add_crontab_readers() {
        let mut readers: Vec<Box<Reader + Sync + Send>> = Vec::new();
        let crontabs = vec!["crontab1".to_string(), "crontab2".to_string()];

        instantiate_file_readers(&mut readers, &crontabs);
        assert_eq!(readers.len(), crontabs.len());
    }
}
