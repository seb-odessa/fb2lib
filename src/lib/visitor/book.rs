use algorithm;
use visitor::acess::AccessGuard;
use fb2parser::FictionBook;

use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;

pub type GenreMap = HashMap<String, String>;

pub struct Book {
    counter: usize,
    access: AccessGuard,
    books: Vec<String>,
    genres: HashMap<String, String>,
}
impl Book {
    pub fn new(access: AccessGuard, genres: GenreMap) -> Self {
        Book {
            counter: 0,
            access: access,
            books: Vec::new(),
            genres: genres,
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

    fn create_genres(&self, book: &FictionBook)->Vec<String> {
        let mut groups = HashSet::new();
        for genre in book.get_book_genres() {
            if let Some(group) = self.genres.get(genre.as_str()) {
                groups.insert(group.clone());
            }
        }
        Vec::from_iter(groups)
    }

    fn format(&self, book: &FictionBook)->Vec<String> {
        let mut result = Vec::new();
        let title = book.get_book_title();
        let date = book.get_book_date();

        for author_desc in book.get_book_authors() {
            let author = Book::create_author(&author_desc);
            let mut description = format!("{:40} : '{}'", author, title);
            if !date.is_empty() {
                description.push_str(" (");
                description.push_str(date.as_str());
                description.push_str(")");
            }
            let sequences = book.get_book_sequences();
            if !sequences.is_empty() {
                description.push_str(" [");
                for sequence in &sequences {
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

            let genres = self.create_genres(book);
            if !genres.is_empty() {
                description.push_str(" {");
                for genre in &genres{
                    if &genres[0] != genre {
                        description.push_str(", ");
                    }
                    description.push_str(format!("{}", genre).as_str());
                }
                description.push_str("}");
            }
            result.push(description);
        }
        result
    }
}
impl algorithm::Visitor<FictionBook> for Book {
    fn visit(&mut self, book: &FictionBook) {
        self.counter += 1;        
        if self.access.is_allowed(book) {
            for description in self.format(book) {
                self.books.push(description);
            }
        }
    }
    fn get_count(&self) -> usize {
        self.counter
    }
    fn report(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }
}
