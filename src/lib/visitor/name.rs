use sal;
use types;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub struct Name {
    counter: usize,
    access: AccessGuard,
    discovered: HashSet<String>,
    handled: HashSet<String>,
}
impl Name {
    pub fn new(access: AccessGuard, handled: HashSet<String>) -> Self {
        Self {
            counter: 0,
            access,
            discovered: HashSet::new(),
            handled,
        }
    }
    fn try_add(&mut self, arg: &str) {
        let name = arg.trim();
        if !self.handled.contains(name) {
            self.discovered.insert(name.to_string());
        }
    }

}
impl sal::Save for Name {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::save_names(&conn, &self.discovered)?;
        self.handled = self.handled.union(&self.discovered).map(|s| s.clone()).collect();
        self.discovered.clear();
        self.counter = 0;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::NAME
    }
    fn get_new_count(&self) -> usize {
        self.discovered.len()
    }
    fn get_stored_count(&self) -> usize {
        self.handled.len()
    }
}
impl <'a> types::Visitor<'a> for Name {

    type Type = FictionBook;
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
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

    fn report(&self){
        println!("=============================================");
        println!("Unique names was known {}", self.handled.len());
        println!("Unique names was found {}", self.discovered.len());
        println!("Total peoples was handled {}", self.counter);
    }
}
