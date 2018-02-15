use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub type BookVisitor = algorithm::Visitor<FictionBook>;
pub type AuthorDesc = (String, String, String, String);

pub struct Author {
    access: AccessGuard,
    pub authors: HashSet<AuthorDesc>,
    ignore: HashSet<AuthorDesc>,
}
impl Author {
    pub fn new(access: AccessGuard, ignore: HashSet<AuthorDesc>) -> Self {
        Author {
            access: access,
            authors: HashSet::new(),
            ignore: ignore,
        }
    }
    pub fn save(&self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_people(&conn, &self.authors)
    }
    pub fn report(&self) -> Fb2Result<()>{
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
        println!("New authors was found {}", self.authors.len());
        println!("Authors already added {}", self.ignore.len());
        Ok(())
    }
}
impl algorithm::Visitor<FictionBook> for Author {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            for author in book.get_book_authors() {
                if !self.ignore.contains(&author) {
                    self.authors.insert(author);
                }
            }

        }
    }
}
