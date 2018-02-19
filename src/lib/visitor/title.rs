use sal;
use algorithm;
use fb2parser::FictionBook;
use visitor::acess::AccessGuard;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Title {
    access: AccessGuard,
    titles: HashSet<String>,
    ignore: HashSet<String>,
    complete: HashSet<String>,
}
impl Title {
    pub fn new(access: AccessGuard, ignore: HashSet<String>) -> Self {
        Title {
            access: access,
            titles: HashSet::new(),
            ignore: ignore,
            complete: HashSet::new(),
        }
    }
}
impl sal::Save<FictionBook> for Title {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_titles(&conn, &self.titles)?;
        self.complete = self.complete.union(&self.titles).map(|s| s.clone()).collect();
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::TITLE
    }    
}
impl algorithm::Visitor<FictionBook> for Title {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            let title = book.get_book_title();
            if !self.ignore.contains(&title) && !self.complete.contains(&title) {
                self.titles.insert(title);
            }
        }
    }
    fn report(&self) {
        for title in &self.titles {
            println!("'{}'", title);
        }
    }
}
