use sal;
use algorithm;
use result::Fb2Result;
use fb2parser::FictionBook;

use std::collections::HashSet;
use std::collections::HashMap;

pub struct Genre {
    counter: usize,
    genres: HashMap<String, usize>,
    handled: HashSet<String>,
}
impl Genre {
    pub fn new(handled: HashSet<String>) -> Self {
        Genre {
            counter: 0,
            genres: HashMap::new(),
            handled: handled,
        }
    }
}
impl sal::Save for Genre {
    fn save(&mut self, _: &sal::Connection) -> Fb2Result<()> {
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::GENRE
    }
    fn get_new_count(&self) -> usize {
        self.genres.len()
    }
    fn get_stored_count(&self) -> usize {
        self.handled.len()
    }
}
impl <'a> algorithm::Visitor<'a> for Genre {
    type Type = FictionBook;
    fn visit(&mut self, book: &mut FictionBook) {
        for genre in book.get_book_genres().into_iter() {
            for genre in genre.split(",") {
                self.counter += 1;
                let genre = genre.trim().to_lowercase();
                 let counter = self.genres.entry(genre).or_insert(0);
                *counter += 1;
            }
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        let mut total = 0;
        let mut unknown = 0;
        for (code, count) in &self.genres {
            total += count;
            if !self.handled.contains(code) {
                unknown += 1;
                println!("{} - {}", code, count);
            }
        }
        if !self.handled.is_empty() {
            println!("Total unknown genres was found {}", unknown);
        }
        println!("Total genres was processed: {}", total);
        println!("Total unique genres was found {}", &self.genres.len());
    }
}
