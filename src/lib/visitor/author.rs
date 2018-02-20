use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub type AuthorDesc = (String, String, String, String);

pub struct Author {
    counter: usize,
    access: AccessGuard,
    authors: HashSet<AuthorDesc>,
    handled: HashSet<AuthorDesc>,
}
impl Author {
    pub fn new(access: AccessGuard, handled: HashSet<AuthorDesc>) -> Self {
        Author {
            counter: 0,
            access: access,
            authors: HashSet::new(),
            handled: handled,
        }
    }
}
impl sal::Save<FictionBook> for Author {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::insert_people(&conn, &self.authors)?;
        self.handled = self.handled.union(&self.authors)
            .map(|a| (a.0.clone(), a.1.clone(), a.2.clone(), a.3.clone()))
            .collect();
        self.authors.clear();
        self.counter = 0;
        Ok(())
    }    
    fn task(&self) -> sal::TASK {
        sal::TASK::AUTHOR
    }
}
impl algorithm::Visitor<FictionBook> for Author {
    fn visit(&mut self, book: &FictionBook) {
        self.counter += 1;
        if self.access.is_allowed(book) {
            for author in book.get_book_authors() {
                if !self.handled.contains(&author) {
                    self.authors.insert(author);
                }
            }
        }
    }
    fn get_total_count(&self) -> usize {
        self.counter
    }
    fn get_new_count(&self) -> usize {
        self.authors.len()
    }
    fn get_stored_count(&self) -> usize {
        self.handled.len()
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
        println!("Authors already added {}", self.handled.len());
    }
}
