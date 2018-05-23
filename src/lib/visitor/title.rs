use sal;
use types::{Visitor, Report};
use fb2parser::FictionBook;
use visitor::guard::Guard;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Title {
    counter: usize,
    guard: Guard,
    accepted: HashSet<String>,
    already_known: HashSet<String>,
}

impl Title {
    pub fn new(guard: Guard, already_known: HashSet<String>) -> Self {
        Title {
            counter: 0,
            guard,
            accepted: HashSet::new(),
            already_known,
        }
    }
}

impl sal::Save for Title {

    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::save_titles(&conn, &self.accepted)?;
        self.already_known = self.already_known.union(&self.accepted).map(|s| s.clone()).collect();
        self.accepted.clear();
        self.counter = 0;
        Ok(())
    }

    fn task(&self) -> sal::TASK {
        sal::TASK::TITLE
    }
}

impl <'a> Visitor<'a> for Title {

    type Type = FictionBook;

    fn visit(&mut self, book: &FictionBook) {
        self.counter += 1;
        if self.guard.is_allowed(book) {
            let title = book.get_book_title();
            if !self.already_known.contains(&title) {
                self.accepted.insert(title);
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

impl Report for Title {

    fn report(&self){
        println!("=============================== Titles ===============================");
        println!("Total books was visited {}.",self.get_visited());
        println!("Unique titles was discovered {}: ", self.get_accepted());
        let mut items: Vec<String> = self.accepted.iter().map(|s| s.clone()).collect();
        items.sort();
        let mut num = 1;
        for item in &items {
            println!("{:>5} {} ", num, item);
            num += 1;
        }
        println!();
    }
}
