use sal;
use algorithm;
use result::Fb2Result;
use fb2parser::FictionBook;

use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Genre {
    genres: HashMap<String, usize>,
    ignore: HashSet<String>,
}
impl Genre {
    pub fn new(ignore: Vec<String>) -> Self {
        Genre {
            genres: HashMap::new(),
            ignore: HashSet::from_iter(ignore),
        }
    }
}
impl sal::Save<FictionBook> for Genre {
    fn save(&self, _: &sal::Connection) -> Fb2Result<()> {
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::GENRE
    }
}
impl algorithm::Visitor<FictionBook> for Genre {
    fn visit(&mut self, book: &FictionBook) {
        for genre in book.get_book_genres().into_iter() {
            for genre in genre.split(",") {
                let genre = genre.trim().to_lowercase();
                 let counter = self.genres.entry(genre).or_insert(0);
                *counter += 1;
            }
        }
    }
    fn report(&self) {
        let mut total = 0;
        let mut unknown = 0;
        for (code, count) in &self.genres {
            total += count;
            if !self.ignore.contains(code) {
                unknown += 1;
                println!("{} - {}", code, count);
            }
        }
        if !self.ignore.is_empty() {
            println!("Total unknown genres was found {}", unknown);
        }
        println!("Total genres was processed: {}", total);
        println!("Total unique genres was found {}", &self.genres.len());
    }
}
