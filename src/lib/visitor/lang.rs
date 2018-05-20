use sal;
use algorithm;
use result::Fb2Result;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub struct Lang {
    counter: usize,
    langs: HashSet<String>,
    handled: HashSet<String>,
}
impl Lang {
    pub fn new(handled: HashSet<String>) -> Self {
        Lang {
            counter: 0,
            langs: HashSet::new(),
            handled: handled,
        }
    }
}
impl sal::Save for Lang {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_languages(&conn, &self.langs)?;
        self.handled = self.handled.union(&self.langs).map(|s| s.clone()).collect();
        self.langs.clear();
        self.counter = 0;
        Ok(())
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::LANGUAGE
    }
    fn get_new_count(&self) -> usize {
        self.langs.len()
    }
    fn get_stored_count(&self) -> usize {
        self.handled.len()
    }
}
impl <'a> algorithm::Visitor<'a> for Lang {
    type Type = FictionBook;
    fn visit(&mut self, book: &FictionBook) {
        self.counter += 1;
        let lang = book.get_book_lang().to_lowercase().as_str().trim().to_string();
        if !self.handled.contains(&lang) {
            self.langs.insert(lang);
        }
    }

    fn get_visited(&self) -> usize {
        self.counter
    }

    fn report(&self) {
        println!("Visited {} languages", self.counter);
        for lang in &self.langs {
            print!("'{}' ", lang);
        }
        println!();
    }
}
