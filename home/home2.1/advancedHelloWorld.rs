use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};

const DATA_FILE: &str = "user_stats.txt";
const SECRET_WORD: &str = "bread";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Error: Invalid number of arguments");
        return;
    }

    let name = &args[1];

    let mut data = read_data();
    
    if args.len() == 3 {
        if args[2] == "delete" {
            data.remove(name);
            write_data(&data);
            println!("Statistics reset for {}", name);
        } else {
            println!("Error: Invalid command");
        }
    } else if name == SECRET_WORD {
        clear_data();
        println!("All history cleared.");
    } else {
        let count = data.entry(name.clone()).or_insert(0);
        *count += 1;

        if *count == 1 {
            println!("Welcome, {}!", name);
        } else {
            println!("Hello again(x{}), {}", count, name);
        }

        write_data(&data);
    }
}

fn read_data() -> HashMap<String, u32> {
    let mut data = HashMap::new();

    if let Ok(file) = File::open(DATA_FILE) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(' ').collect();
                if let Ok(count) = parts[1].parse() {
                    data.insert(parts[0].to_string(), count);
                }
            }
        }
    }

    data
}

fn write_data(data: &HashMap<String, u32>) {
    let mut file = File::create(DATA_FILE).expect("Unable to create data file");
    for (name, count) in data {
        writeln!(file, "{} {}", name, count).expect("Unable to write data to file");
    }
}

fn clear_data() {
    fs::remove_file(DATA_FILE).expect("Unable to clear data file");
}
