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

    for r#type in vec!["files", "crontabs"] {
        let mut readers_conf: Vec<String> = vec![];

        let config_paths = &config_readers[r#type];
        if config_paths.is_badvalue() {
            println!("[CONFIG] key \"{}\" not found", r#type);
            continue;
        }

        if let Some(paths) = config_paths.as_vec() {
            for config_path in paths {
                if let Some(path) = config_path.as_str() {
                    readers_conf.push(path.to_string());
                } else {
                    println!("[CONFIG] Value for reader \"{}\" must be a string", r#type);
                }
            }
            readers.insert(r#type.to_string(), readers_conf);
        } else {
            println!(
                "[CONFIG] Values for reader \"{}\" must be in an array",
                r#type
            );
        }
    }

    readers
}
