use sal;
use types::{Visitor, Report};
use result::Fb2Result;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub struct Lang {
    counter: usize,
    accepted: HashSet<String>,
    already_known: HashSet<String>,
}

impl Lang {
    pub fn new(already_known: HashSet<String>) -> Self {
        Lang {
            counter: 0,
            accepted: HashSet::new(),
            already_known,
        }
    }
}

impl sal::Save for Lang {
    fn save(&mut self, conn: &mut sal::Connection) -> Fb2Result<()> {
        sal::save_languages(conn, &self.accepted)?;
        self.already_known = self.already_known.union(&self.accepted).map(|s| s.clone()).collect();
        self.accepted.clear();
        self.counter = 0;
        Ok(())
    }

    fn task(&self) -> sal::TASK {
        sal::TASK::LANGUAGE
    }
}

impl <'a> Visitor<'a> for Lang {

    type Type = FictionBook;

    fn visit(&mut self, book: &FictionBook) {
        self.counter += 1;
        let lang = book.get_book_lang().to_lowercase().as_str().trim().to_string();
        if !self.already_known.contains(&lang) {
            self.accepted.insert(lang);
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

impl Report for Lang {

    fn report(&self){
        println!("=============================== Languages ===============================");
        println!("Total books was visited {}.",self.get_visited());
        print!("Unique languages was discovered {}: ", self.get_accepted());
        let mut items: Vec<String> = self.accepted.iter().map(|s| s.clone()).collect();
        items.sort();
        for item in &items {
            print!("'{}' ", item);
        }
        println!();
    }
}