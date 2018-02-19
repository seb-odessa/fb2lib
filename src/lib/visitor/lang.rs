use sal;
use algorithm;
use result::Fb2Result;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub struct Lang {
    langs: HashSet<String>,
    ignore: HashSet<String>,
    complete: HashSet<String>,
}
impl Lang {
    pub fn new(ignore: HashSet<String>) -> Self {
        Lang {
            langs: HashSet::new(),
            ignore: ignore,
            complete: HashSet::new(),            
        }
    }
}
impl sal::Save<FictionBook> for Lang {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_languages(&conn, &self.langs)?;
        let complete: HashSet<String> = self.complete.union(&self.langs).map(|s| s.clone()).collect();
        self.complete = complete;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::LANGUAGE
    }
}
impl algorithm::Visitor<FictionBook> for Lang {
    fn visit(&mut self, book: &FictionBook) {
        let lang = book.get_book_lang();
        if !self.ignore.contains(&lang) && !self.complete.contains(&lang) {
            self.langs.insert(lang);
        }
    }
    fn report(&self) {
        for lang in &self.langs {
            println!("'{}'", lang);
        }
    }
}
