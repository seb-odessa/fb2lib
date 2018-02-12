use algorithm;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub type AuthorVisitor = algorithm::Visitor<FictionBook>;

pub struct Author {
    access: AccessGuard,
    pub authors: HashSet<(String, String, String, String)>,
}
impl Author {
    pub fn new(access: AccessGuard) -> Self {
        Author {
            access: access,
            authors: HashSet::new(),
        }
    }
    pub fn report(&self) {
        for author in &self.authors {
            let (first_name, middle_name, last_name, nick_name) = author.clone();
            if first_name.is_empty() && middle_name.is_empty() && last_name.is_empty() && !nick_name.is_empty() {
                println!("{}", nick_name);
            } else {
                print!("{}", last_name);
                if !last_name.is_empty() && !first_name.is_empty() {
                    print!(" ");
                }
                print!("{}", first_name);
                
                if (!last_name.is_empty() || !first_name.is_empty()) && !middle_name.is_empty() {
                    print!(" ");
                }
                println!("{}", middle_name);                
            }
        }
    }
}
impl algorithm::Visitor<FictionBook> for Author {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {            
            for author in book.get_book_authors() {
                self.authors.insert(author);
            }

        }
    }
}
