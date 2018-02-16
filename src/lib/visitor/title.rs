use sal;
use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;

use std::collections::HashSet;

pub struct Title {
    titles: HashSet<String>,
    ignore: HashSet<String>,
}
impl Title {
    pub fn new(ignore: HashSet<String>) -> Self {
        Title {
            titles: HashSet::new(),        
            ignore: ignore,
        }
    }
}
impl sal::Save<FictionBook> for Title {
    fn save(&self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_title(&conn, &self.titles)?;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::TITLE
    }    
}
impl algorithm::Visitor<FictionBook> for Title {
    fn visit(&mut self, book: &FictionBook) {
        let title = book.get_book_title();
        if !self.ignore.contains(&title) {
            self.titles.insert(title);
        }        
    }
    fn report(&self) {
        for title in &self.titles {
            println!("'{}'", title);
        }
    }
}
