use sal;
use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;

use std::iter::FromIterator;
use std::collections::HashSet;

pub struct Title {
    titles: HashSet<String>,
    ignore: HashSet<String>,
}
impl Title {
    pub fn new(ignore: Vec<String>) -> Self {
        Title {
            titles: HashSet::new(),        
            ignore: HashSet::from_iter(ignore),
        }
    }
}
impl sal::Save<FictionBook> for Title {
    fn save(&self, _conn: &sal::Connection) -> Fb2Result<()> {
        for _ in &self.titles {
            //sal::insert_title(&conn, title.as_str().trim())?;
        }
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::TITLE
    }    
}
impl algorithm::Visitor<FictionBook> for Title {
    fn visit(&mut self, book: &FictionBook) {
        self.titles.insert(book.get_book_title());
    }
    fn report(&self) {
        for title in &self.titles {
            println!("'{}'", title);
        }
    }
}
