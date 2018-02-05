use algorithm;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::collections::HashMap;

pub struct Book {
    count: usize,
    access: AccessGuard,
    books: Vec<String>,
    genres: HashMap<String, String>,
}
impl Book {
    pub fn new(access: AccessGuard) -> Self {
        Book {
            count: 0,
            access: access,
            books: Vec::new(),
            genres: HashMap::new(),
        }
    }
    pub fn report(&self) {
        for row in &self.books {
            println!("{}", row);
        }
    }
    fn create_author(desc: &(String, String, String, String))->String {
        let (fname, mname, lname, nick) = desc.clone();
        if fname.is_empty() && mname.is_empty() && lname.is_empty() {
            return nick;
        } else {
            let mut result = lname; // фамилия
            if !fname.is_empty() {
                result.push_str(" ");
            }
            result.push_str(fname.as_str()); // имя
            if !mname.is_empty() {
                result.push_str(" ");
            }            
            result.push_str(mname.as_str()); // отчество
            return result;
        }
    }


    fn format(book: &FictionBook)->Vec<String> {
        let mut result = Vec::new();
        let title = book.get_book_title();
        let date = book.get_book_date();
        for author_desc in book.get_book_authors() {
            let author = Book::create_author(&author_desc);
            let mut description = format!("{:40} : {}", author, title);
            if !date.is_empty() {
                description.push_str(" (");
                description.push_str(date.as_str());
                description.push_str(")");
            }
            let sequences = book.get_book_sequences();
            if !sequences.is_empty() {
                description.push_str(" [");
                for sequence in  &sequences {
                    if &sequences[0] != sequence {
                        description.push_str(", ");
                    }
                    if sequence.1 > 0 {
                        description.push_str(format!("{} - {}", sequence.0, sequence.1).as_str());
                    } else {
                        description.push_str(format!("{}", sequence.0).as_str());
                    }
                    
                }
                description.push_str("]");
            }
            result.push(description);
        }
        result
    }
}
impl algorithm::Visitor<FictionBook> for Book {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            self.count += 1;
            for description in Book::format(book) {
                self.books.push(description);
            }
        }
    }
}
