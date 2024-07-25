mod config;

use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use config::Config;

fn main() {
    let mut favorite_words = Vec::new();
    for i in 0..5 {
        print!("Enter favorite word {}: ", i + 1);
        io::stdout().flush().unwrap();
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();
        let word = word.trim().to_string();
        favorite_words.push(word);
    }

    let mut handles = vec![];

    for word in favorite_words {
        let first_char = word.chars().next().unwrap_or('a').to_ascii_lowercase();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            let config = Config::get().lock().unwrap();
            if let Some((pre_word, post_word)) = config.get_pre_post(first_char) {
                println!("{} word {}", pre_word, post_word);
            } else {
                println!("No configuration for word starting with '{}'", first_char);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
