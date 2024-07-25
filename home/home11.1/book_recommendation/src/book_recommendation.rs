use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct BookRecommendation {
    books: Vec<&'static str>,
}

impl BookRecommendation {
    pub fn new() -> Self {
        Self {
            books: vec![
                "The Topographer’s Clown",
                "The Chamber of Beaver",
                "The Ironer of Kanban",
                "The Piglet of Tire",
                "The Border of the Unix",
                "The Half-Time Convince",
                "The Earthly Pillows",
                "The Censorship of the Ping",
                "The True Powers",
                "The Overturn of the Ling",
            ],
        }
    }

    pub fn get_recommendation(&mut self) -> Option<&'static str> {
        if self.books.is_empty() {
            None
        } else {
            let mut rng = thread_rng();
            Some(*self.books.choose(&mut rng).unwrap())
        }
    }

    pub fn remove_book(&mut self, book: &str) {
        self.books.retain(|&b| b != book);
    }
}
