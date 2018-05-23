use sal;
use types;
use result::Fb2Result;
use visitor::guard::Guard;
use fb2parser::FictionBook;

use std::collections::HashSet;
use std::collections::HashMap;

pub type AuthorDesc = (i64, i64, i64, i64);

pub struct Author {
    counter: usize,
    guard: Guard,
    names: HashMap<String, i64>,
    accepted: HashSet<AuthorDesc>,
    already_known: HashSet<AuthorDesc>,
}
impl Author {
    pub fn new(guard: Guard, names: HashMap<String, i64>, already_known: HashSet<AuthorDesc>) -> Self {
        Author {
            counter: 0,
            guard,
            names,
            accepted: HashSet::new(),
            already_known,
        }
    }

    fn get_name_id(&self, name: &str) -> Option<i64> {
        self.names.get(name.trim()).map(|value| value.clone())
    }
}
impl sal::Save for Author {
    fn save(&mut self, conn: &sal::Connection) -> Fb2Result<()> {
        sal::save_people(&conn, &self.accepted)?;
        self.already_known = self.already_known.union(&self.accepted).map(|s| s.clone()).collect();
        self.accepted.clear();
        self.counter = 0;
        Ok(())
    }

    fn task(&self) -> sal::TASK {
        sal::TASK::AUTHOR
    }
}

impl <'a> types::Visitor<'a> for Author {

    type Type = FictionBook;

    fn visit(&mut self, book: &FictionBook) {
        self.counter += 1;
        if self.guard.is_allowed(book) {
            for author in book.get_book_authors() {
                if let Some(first_name_id) = self.get_name_id(&author.0) {
                    if let Some(middle_name_id) = self.get_name_id(&author.1) {
                        if let Some(last_name_id) = self.get_name_id(&author.2) {
                            if let Some(nick_name_id) = self.get_name_id(&author.3) {
                                let desc = (first_name_id, middle_name_id, last_name_id, nick_name_id);
                                if !self.already_known.contains(&desc) {
                                    self.accepted.insert(desc);
                                }
                            }
                        }
                    }
                }
//                if !self.handled.contains(&author) {
//                    self.authors.insert(author);
//                }
            }
//
//
//            if !self.already_known.contains(name) {
//                self.accepted.insert(name.to_string());
//            }
//

        }
    }

    fn get_visited(&self) -> usize {
        self.counter
    }

    fn get_accepted(&self) -> usize {
        self.accepted.len()
    }

    fn get_already_known(&self) -> usize {
        self.already_known.len()
    }

//    fn report(&self){
//        for author in &self.authors {
//            let (first_name, middle_name, last_name, nick_name) = author.clone();
//            if first_name.is_empty() && middle_name.is_empty() && last_name.is_empty() && !nick_name.is_empty() {
//                println!("{}", nick_name);
//            } else {
//                print!("{}", last_name);
//                if !last_name.is_empty() && !first_name.is_empty() {
//                    print!(" ");
//                }
//                print!("{}", first_name);
//
//                if (!last_name.is_empty() || !first_name.is_empty()) && !middle_name.is_empty() {
//                    print!(" ");
//                }
//                println!("{}", middle_name);
//            }
//        }
//        println!("=============================================");
//        println!("Unique authors was found {}", self.authors.len());
//        println!("Total authors was found {}", self.counter);
//    }
}
