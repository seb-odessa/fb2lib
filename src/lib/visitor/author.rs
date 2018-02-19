use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub type AuthorDesc = (String, String, String, String);

pub struct Author {
    access: AccessGuard,
    authors: HashSet<AuthorDesc>,
    ignore: HashSet<AuthorDesc>,
    complete: HashSet<AuthorDesc>,
}
impl Author {
    pub fn new(access: AccessGuard, ignore: HashSet<AuthorDesc>) -> Self {
        Author {
            access: access,
            authors: HashSet::new(),
            ignore: ignore,
            complete: HashSet::new(),
        }
    }
}
impl sal::Save<FictionBook> for Author {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_people(&conn, &self.authors)?;
        self.complete = self.complete.union(&self.authors)
            .map(|a| (a.0.clone(), a.1.clone(), a.2.clone(), a.3.clone()))
            .collect();
        Ok(())        
    }
    fn task(&self) -> sal::TASK {
        sal::TASK::AUTHOR
    }
}
impl algorithm::Visitor<FictionBook> for Author {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            for author in book.get_book_authors() {
                if !self.ignore.contains(&author) && !self.complete.contains(&author){
                    self.authors.insert(author);
                }
            }

        }
    }
    fn report(&self){
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
    }
}
