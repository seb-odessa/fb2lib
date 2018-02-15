use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Genre {
    genres: HashMap<String, usize>,
    known: HashSet<String>,
}
impl Genre {
    pub fn new(genres: Vec<String>) -> Self {
        Genre {
            genres: HashMap::new(),
            known: HashSet::from_iter(genres),
        }
    }

    pub fn report(&self, unknown_only: bool) -> Fb2Result<()> {        
        let mut total = 0;
        let mut unknown = 0;
        for (code, count) in &self.genres {
            total += count;
            if !unknown_only || (unknown_only && !self.known.contains(code)) {
                unknown += 1;
                println!("{} - {}", code, count);
            }
        }
        if unknown_only {
            println!("Total unknown genres was found {}", unknown);
        }
        println!("Total genres was processed: {}", total);
        println!("Total unique genres was found {}", &self.genres.len());
        Ok(())
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
}
