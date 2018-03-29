use sal;
use algorithm;
use fb2parser::FictionBook;
use visitor::acess::AccessGuard;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Title {
    counter: usize,
    access: AccessGuard,
    titles: HashSet<String>,
    handled: HashSet<String>,
}
impl Title {
    pub fn new(access: AccessGuard, handled: HashSet<String>) -> Self {
        Title {
            counter: 0,
            access: access,
            titles: HashSet::new(),
            handled: handled,
        }
    }
}
impl sal::Save for Title {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_titles(&conn, &self.titles)?;
        self.handled = self.handled.union(&self.titles).map(|s| s.clone()).collect();
        self.titles.clear();
        self.counter = 0;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::TITLE
    }
    fn get_new_count(&self) -> usize {
        self.titles.len()
    }
    fn get_stored_count(&self) -> usize {
        self.handled.len()
    }
}
impl <'a> algorithm::Visitor<'a> for Title {
    type Type = FictionBook;
    fn visit(&mut self, book: &mut FictionBook) {
        self.counter += 1;
        if self.access.is_allowed(book) {
            let title = book.get_book_title();
            if !self.handled.contains(&title) {
                self.titles.insert(title);
            }
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        for title in &self.titles {
            println!("'{}'", title);
        }
        println!("=============================================");
        println!("Unique book titles was found {}", self.titles.len());
        println!("Total titles was found {}", self.counter);
    }
}
