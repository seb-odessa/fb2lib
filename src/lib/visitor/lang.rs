use sal;
use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;
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
    
    pub fn report(&self) -> Fb2Result<()> {
        for lang in &self.langs {
            println!("'{}'", lang);
        }
        Ok(())
    }
    
    pub fn save(&self, conn: &sal::Connection) -> Fb2Result<()> {
        for lang in &self.langs {
            sal::insert_language(&conn, lang.to_lowercase().as_str().trim())?;
        }
        Ok(())
    }
}
impl algorithm::Visitor<FictionBook> for Lang {
    fn visit(&mut self, book: &FictionBook) {
        self.langs.insert(book.get_book_lang());
    }
}
