use sal;
use types;
use result::Fb2Result;
use visitor::guard::Guard;
use fb2parser::FictionBook;

use std::collections::HashSet;

pub type AuthorDesc = (String, String, String, String);

pub struct Author {
    counter: usize,
    access: Guard,
    authors: HashSet<AuthorDesc>,
    handled: HashSet<AuthorDesc>,
}
impl Author {
    pub fn new(access: Guard, handled: HashSet<AuthorDesc>) -> Self {
        Author {
            counter: 0,
            access: access,
            authors: HashSet::new(),
            handled: handled,
        }
    }
}
impl sal::Save for Author {
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
        sal::TASK::NAME
    }
}

impl <'a> types::Visitor<'a> for Author {

    type Type = FictionBook;

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

    fn get_visited(&self) -> usize {
        self.counter
    }

    fn get_accepted(&self) -> usize {
        self.authors.len()
    }

    fn get_already_known(&self) -> usize {
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
        println!("=============================================");
        println!("Unique authors was found {}", self.authors.len());
        println!("Total authors was found {}", self.counter);
    }
}
