extern crate yaml_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::yaml;

pub struct Config {
    pub readers: HashMap<String, Vec<String>>,
}

impl Config {
    fn from_readers(readers: HashMap<String, Vec<String>>) -> Self {
        Config { readers: readers }
    }
}

pub fn get_config() -> Config {
    let mut f = File::open("config.yaml").expect("Cannot open config.yaml");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Cannot read config.yaml");

    let docs = yaml::YamlLoader::load_from_str(&s).expect("Cannot parse config.yaml");
    let doc = &docs[0];

    let config_readers = &doc["readers"];
    if config_readers.is_badvalue() {
        panic!("No readers configured, exiting");
    }
    let readers = extract_readers(config_readers);

    Config::from_readers(readers)
}

fn extract_readers(config_readers: &yaml_rust::Yaml) -> HashMap<String, Vec<String>> {
    let mut readers: HashMap<String, Vec<String>> = HashMap::new();
    let mut crontab_readers_conf: Vec<String> = vec![];

    let config_crontab_paths = &config_readers["files"];
    for config_crontab_path in config_crontab_paths
        .as_vec()
        .expect("Crontabs readers must be an array")
    {
        crontab_readers_conf.push(
            config_crontab_path
                .as_str()
                .expect("Crontab file name must be a string")
                .to_string(),
        );
    }
    readers.insert("files".to_string(), crontab_readers_conf);

    readers
}
