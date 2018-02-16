use sal;
use algorithm;
use result::Fb2Result;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub struct Lang {
    langs: HashSet<String>,
}
impl Lang {
    pub fn new() -> Self {
        Lang {
            langs: HashSet::new(),
        }
    }
}
impl sal::Save<FictionBook> for Lang {
    fn save(&self, conn: &sal::Connection) -> Fb2Result<()> {
        for lang in &self.langs {
            sal::insert_language(&conn, lang.to_lowercase().as_str().trim())?;
        }
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::LANGUAGE
    }
}
impl algorithm::Visitor<FictionBook> for Lang {
    fn visit(&mut self, book: &FictionBook) {
        self.langs.insert(book.get_book_lang());
    }
    fn report(&self) {
        for lang in &self.langs {
            println!("'{}'", lang);
        }
    }
}
