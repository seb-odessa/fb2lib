use sal;
use algorithm;
use fb2parser::FictionBook;
use visitor::acess::AccessGuard;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Sequence {
    access: AccessGuard,
    names: HashSet<String>,
    ignore: HashSet<String>,
    complete: HashSet<String>,
}
impl Sequence {
    pub fn new(access: AccessGuard, ignore: HashSet<String>) -> Self {
        Sequence {
            access: access,
            names: HashSet::new(),
            ignore: ignore,
            complete: HashSet::new(),
        }
    }
}
impl sal::Save<FictionBook> for Sequence {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_sequences(&conn, &self.names)?;
        self.complete = self.complete.union(&self.names).map(|s| s.clone()).collect();
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::SEQUENCE
    }    
}
impl algorithm::Visitor<FictionBook> for Sequence {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            let sequences = book.get_book_sequences();
            for sequence in &sequences {
                let name = format!("{}", sequence.0);
                if !self.ignore.contains(&name) && !self.complete.contains(&name) {
                    self.names.insert(name);
                }
            }
        }
    }
    fn report(&self) {
        for name in &self.names {
            println!("'{}'", name);
        }
    }
}
