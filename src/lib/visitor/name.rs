use sal;
use types;
//use algorithm;
use result::Fb2Result;
use visitor::guard::Guard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub struct Name {
    counter: usize,
    guard: Guard,
    accepted: HashSet<String>,
    already_known: HashSet<String>,
}

impl Name {

    pub fn new(access: Guard, already_known: HashSet<String>) -> Self {
        Self {
            counter: 0,
            guard: access,
            accepted: HashSet::new(),
            already_known: already_known,
        }
    }

    fn try_add(&mut self, arg: &str) {
        let name = arg.trim();
        if !self.already_known.contains(name) {
            self.accepted.insert(name.to_string());
        }
    }

}
impl sal::Save for Name {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::save_names(&conn, &self.accepted)?;
        self.already_known = self.already_known.union(&self.accepted).map(|s| s.clone()).collect();
        self.accepted.clear();
        self.counter = 0;
        Ok(())
    }

    fn task(&self) -> sal::TASK {
        sal::TASK::NAME
    }
}

impl <'a> types::Visitor<'a> for Name {

    type Type = FictionBook;

    fn visit(&mut self, book: &FictionBook) {
        if self.guard.is_allowed(book) {
            for author in book.get_book_authors() {
                self.counter += 1;
                self.try_add(&author.0);
                self.try_add(&author.1);
                self.try_add(&author.2);
                self.try_add(&author.3);
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
