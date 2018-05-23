use sal;
use types::{Visitor, Report};
use result::Fb2Result;
use fb2parser::FictionBook;

use std::collections::HashSet;
use std::collections::HashMap;

pub struct Genre {
    counter: usize,
    accepted: HashMap<String, usize>,
    already_known: HashSet<String>,
}
impl Genre {
    pub fn new(handled: HashSet<String>) -> Self {
        Genre {
            counter: 0,
            accepted: HashMap::new(),
            already_known: handled,
        }
    }
}
impl sal::Save for Genre {
    fn save(&mut self, _: &mut sal::Connection) -> Fb2Result<()> {
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::GENRE
    }
}
impl <'a> Visitor<'a> for Genre {
    type Type = FictionBook;
    fn visit(&mut self, book: &FictionBook) {
        for genre in book.get_book_genres().into_iter() {
            for genre in genre.split(",") {
                self.counter += 1;
                let genre = genre.trim().to_lowercase();
                 let counter = self.accepted.entry(genre).or_insert(0);
                *counter += 1;
            }
        }
    }

    fn get_visited(&self) -> usize {
        self.counter
    }

    fn get_accepted(&self) -> usize {
        self.accepted.len()
    }

    fn get_already_known(&self) -> usize {
        self.already_known.len()
    }

}

impl Report for Genre {

    fn report(&self){
        println!("=============================== Genres ===============================");
        println!("Total books was visited {}.",self.get_visited());
        println!("Unique genres was discovered {}: ", self.get_accepted());
        let mut num = 1;
        for (code, count) in &self.accepted {
            println!("{:>5} {} - {}", num, code, count);
            num += 1;
        }
    }
}
