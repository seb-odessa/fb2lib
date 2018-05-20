use sal;
use algorithm;
use fb2parser::FictionBook;
use visitor::acess::AccessGuard;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Sequence {
    counter: usize,
    access: AccessGuard,
    names: HashSet<String>,
    handled: HashSet<String>,
}
impl Sequence {
    pub fn new(access: AccessGuard, handled: HashSet<String>) -> Self {
        Sequence {
            counter: 0,
            access: access,
            names: HashSet::new(),
            handled: handled,
        }
    }
}
impl sal::Save for Sequence {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_sequences(&conn, &self.names)?;
        self.handled = self.handled.union(&self.names).map(|s| s.clone()).collect();
        self.names.clear();
        self.counter = 0;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::SEQUENCE
    }
    fn get_new_count(&self) -> usize {
        self.names.len()
    }
    fn get_stored_count(&self) -> usize {
        self.handled.len()
    }
}
impl <'a> algorithm::MutVisitor<'a> for Sequence {
    type Type = FictionBook;
    fn visit(&mut self, book: &mut FictionBook) {
        if self.access.is_allowed(&book) {
            self.counter += 1;
            let sequences = book.get_book_sequences();
            for sequence in &sequences {
                let name = format!("{}", sequence.0);
                if !self.handled.contains(&name) {
                    self.names.insert(name);
                }
            }
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        for name in &self.names {
            println!("'{}'", name);
        }
        println!("=============================================");
        println!("Unique book sequences was found {}", self.names.len());
        println!("Total sequences was found {}", self.counter);
    }
}
