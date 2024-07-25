use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Mutex;

pub struct Config {
    map: HashMap<char, (String, String)>,
}

impl Config {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        match File::open("/home/ubuntu/ProfProg/home/home5.2/singleton/src/config.txt") {
            Ok(file) => {
                let lines = io::BufReader::new(file).lines();
                for line in lines.flatten() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() == 3 {
                        let range = parts[0];
                        let pre_word = parts[1].to_string();
                        let post_word = parts[2].to_string();
                        let range_chars: Vec<char> = range.chars().collect();
                        if range_chars.len() == 3 && range_chars[1] == '-' {
                            let start = range_chars[0];
                            let end = range_chars[2];
                            for ch in start..=end {
                                map.insert(ch, (pre_word.clone(), post_word.clone()));
                            }
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error opening config file: {}", err);
            }
        }
        Config { map }
    }

    pub fn get() -> &'static Mutex<Config> {
        static INSTANCE: OnceCell<Mutex<Config>> = OnceCell::new();
        INSTANCE.get_or_init(|| Mutex::new(Config::new()))
    }

    pub fn get_pre_post(&self, ch: char) -> Option<(String, String)> {
        self.map.get(&ch).cloned()
    }
}
