use sal;
use types;
use fb2parser::FictionBook;
use visitor::guard::Guard;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Sequence {
    counter: usize,
    guard: Guard,
    accepted: HashSet<String>,
    already_known: HashSet<String>,
}

impl Sequence {
    pub fn new(access: Guard, handled: HashSet<String>) -> Self {
        Sequence {
            counter: 0,
            guard: access,
            accepted: HashSet::new(),
            already_known: handled,
        }
    }
}

impl sal::Save for Sequence {

    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_sequences(&conn, &self.accepted)?;
        self.already_known = self.already_known.union(&self.accepted).map(|s| s.clone()).collect();
        self.accepted.clear();
        self.counter = 0;
        Ok(())
    }

    fn task(&self) -> sal::TASK {
        sal::TASK::SEQUENCE
    }
}

impl <'a> types::MutVisitor<'a> for Sequence {

    type Type = FictionBook;

    fn visit(&mut self, book: &mut FictionBook) {
        if self.guard.is_allowed(&book) {
            self.counter += 1;
            let sequences = book.get_book_sequences();
            for sequence in &sequences {
                let name = format!("{}", sequence.0);
                if !self.already_known.contains(&name) {
                    self.accepted.insert(name);
                }
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

    fn report(&self) {
        for name in &self.accepted {
            println!("'{}'", name);
        }
        println!("=============================================");
        println!("Unique book sequences was found {}", self.accepted.len());
        println!("Total sequences was found {}", self.counter);
    }
}
