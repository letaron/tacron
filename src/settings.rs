use std::collections::HashMap;

pub fn get_settings() -> HashMap<String, Vec<String>> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();

    settings.try_into::<HashMap<String, Vec<String>>>().unwrap()
}
