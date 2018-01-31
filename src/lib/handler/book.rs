use sal;
use result::Fb2Result;
use fb2parser::FictionBook;
//use tools;
use algorithm;

use std::collections::HashSet;

pub struct BookVisitor {
    disabled_genres: HashSet<String>,
    disabled_langs: HashSet<String>
}
impl BookVisitor {
    pub fn new(genres: HashSet<String>, langs: HashSet<String>) -> Self {
        BookVisitor {
            disabled_genres: genres,
            disabled_langs: langs,
        }
    }

    fn is_genre_allowed(&self, book: &FictionBook) -> bool {
        for genre in book.get_book_genres().into_iter() {
            for genre in genre.split(",") {
                let genre = genre.trim().to_lowercase();
                if !self.disabled_genres.contains(&genre) {
                    return true;
                }
            }
        }
        false
    }

    fn is_book_allowed(&self, book: &FictionBook) -> bool {
        self.is_genre_allowed(book)        
    }
}
impl algorithm::Visitor<FictionBook> for BookVisitor {
    fn visit(&mut self, book: &FictionBook) {
        let allowed = self.is_book_allowed(book);
        println!("The book is {}, {}", allowed, book);
    }
}

pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let _conn = sal::get_connection(db)?;
    let mut visitor = BookVisitor::new(HashSet::new(), HashSet::new());
    for archive in archives {
        println!("{}", archive);
        algorithm::visit(archive, &mut visitor)?;
    }
    Ok(())
}
