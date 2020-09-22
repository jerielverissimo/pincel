use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::{collections::HashMap, env};

const HOME_ENV_VAR: &str = "HOME";
const CONFIG_DIR: &str = ".config/pincel";
const CONFIG_FILE_NAME: &str = "pincel.conf";
const SCREENSHOT_DIR_KEY_CONFIG: &str = "screenshot_dir";

type Configs = HashMap<String, String>;

pub struct Config {
    pub screenshot_dir: String,
    pub configs: Configs,
    config_file_path: PathBuf,
    configs_str: String,
    home_dir: String,
}

impl Config {
    pub fn new() -> Self {
        let home_dir = match env::var(HOME_ENV_VAR) {
            Ok(dir) => dir,
            _ => "~/".to_owned(),
        };

        let home_path = PathBuf::from(home_dir.to_string());
        let config_file_path = home_path.join(PathBuf::from(CONFIG_DIR));
        let mut config = Self {
            screenshot_dir: String::new(),
            configs: Configs::new(),
            config_file_path,
            configs_str: String::new(),
            home_dir,
        };

        match config.read_config_file() {
            Ok(contents) => config.configs_str = contents,
            Err(e) => println!("Error on reading file: {:?}", e),
        }

        config.configs = config.extract_configs();
        config.screenshot_dir = config.screenshot_dir();

        config
    }

    fn read_config_file(&self) -> std::io::Result<String> {
        if fs::read_dir(&self.config_file_path).is_err() {
            fs::create_dir(&self.config_file_path)?;
        }

        let file_path = PathBuf::from(CONFIG_FILE_NAME);
        let file_path = &self.config_file_path.join(file_path);
        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(_) => File::create(file_path)?,
        };

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn screenshot_dir(&self) -> String {
        let mut dir = match self.configs.get(SCREENSHOT_DIR_KEY_CONFIG) {
            Some(dir) => dir.to_owned(),
            None => String::from("~/Pictures"),
        };

        dir = self.normalize_relative_path(dir);

        dir
    }

    fn normalize_relative_path(&self, mut path: String) -> String {
        if path.starts_with("~/") {
            let scr_dir = path.split('~').collect::<Vec<&str>>()[1];
            path = self.home_dir.as_str().to_owned() + scr_dir;
        }
        path
    }

    fn extract_configs(&self) -> Configs {
        let mut configs = Configs::new();

        for line in self.configs_str.lines() {
            // ignore lines that starts with comments
            if line.starts_with('#') {
                continue;
            }

            let line = line.split_whitespace().collect::<String>();
            let line = line.split('#').collect::<Vec<&str>>()[0]; // ignore end line's comments
            let key_value = line.split('=').collect::<Vec<&str>>();

            let key = key_value[0];
            let value = key_value[1];

            let value = value.split('\"').collect::<Vec<&str>>();

            configs.insert(key.to_owned(), value[1].to_owned());
        }

        configs
    }
}
