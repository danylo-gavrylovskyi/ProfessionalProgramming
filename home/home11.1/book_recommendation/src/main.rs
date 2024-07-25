mod book_recommendation;

use book_recommendation::BookRecommendation;
use std::io::{self, Write};

fn main() {
    let mut book_recommender = BookRecommendation::new();

    let mut name = String::new();
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    loop {
        match book_recommender.get_recommendation() {
            Some(recommendation) => {
                println!("How about: {}?", recommendation);
                println!("Do you want this book? (yes/no):");

                let mut response = String::new();
                io::stdin().read_line(&mut response).unwrap();
                let response = response.trim();

                if response == "yes" {
                    println!("Enjoy your book, {}!", name);
                    break;
                } else if response == "no" {
                    book_recommender.remove_book(&recommendation);
                } else {
                    println!("Invalid response. Please answer 'yes' or 'no'.");
                }
            }
            None => {
                println!("Au revoir, {}!", name);
                break;
            }
        }
    }
}
