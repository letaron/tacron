use super::file_reader::parse_content;
use crate::reader::RawCron;
use crate::Reader;
use std::process::Command;

pub struct CrontabReader {
    user: String,
}

impl CrontabReader {
    pub fn new(user: String) -> Self {
        CrontabReader { user }
    }
}

impl Reader for CrontabReader {
    fn raw_crons(&self) -> Vec<RawCron> {
        let output = Command::new("crontab")
            .arg("-u")
            .arg(&self.user)
            .arg("-l")
            .output()
            .expect(&format!(
                "Failed to execute crontab for user {}",
                &self.user
            ));

        if !output.status.success() {
            panic!("Crontab for user {} did not succeed", self.user)
        }

        let mut rawdata: Vec<u8> = Vec::new();
        for b in &output.stdout {
            rawdata.push(*b);
        }

        let content = match String::from_utf8(rawdata) {
            Ok(x) => x,
            _ => panic!("Could not read crontab output for user {}", &self.user),
        };

        parse_content(content, &self.user)
    }
}

/// Push a reader in `readers` for each crontabs installed for `users`
pub fn instantiate_crontab_readers(
    readers: &mut Vec<Box<Reader + Sync + Send>>, users: &Vec<String>,
) {
    for user in users {
        println!("[READER] crontab - loading: {:?}", user);
        readers.push(Box::new(CrontabReader::new(user.to_string())));
    }
}
