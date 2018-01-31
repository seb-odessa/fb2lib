use sal;
use result::Fb2Result;
use fb2parser::FictionBook;
//use tools;
use algorithm;

use std::collections::HashSet;
use std::iter::FromIterator;

struct Access {
    disabled_genres: HashSet<String>,
    disabled_langs: HashSet<String>
}
impl Access {
    fn new() -> Self {
        Access {
            disabled_genres: HashSet::new(),
            disabled_langs: HashSet::new(),
        }
    }
    
    fn disable_langs(&mut self, langs: Vec<String>) {
        self.disabled_langs = HashSet::from_iter(langs);
    }

    fn disable_genres(&mut self, genres: Vec<String>) {
        self.disabled_genres = HashSet::from_iter(genres);
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

    fn is_lang_allowed(&self, book: &FictionBook) -> bool {
        !self.disabled_langs.contains(book.get_book_lang().as_str())
    }

    fn is_allowed(&self, book: &FictionBook) -> bool {
        self.is_genre_allowed(book) && self.is_lang_allowed(book)
    }
}

struct BookVisitor {
    access: Access
}
impl BookVisitor {
    fn new(access: Access) -> Self {
        BookVisitor {
            access: access
        }
    }
}
impl algorithm::Visitor<FictionBook> for BookVisitor {
    fn visit(&mut self, book: &FictionBook) {
        let allowed = self.access.is_allowed(book);
        println!("The book is {}, {}", allowed, book);
    }
}

pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let langs: Vec<String> = sal::get_languages_disabled(&conn)?;
    let genres: Vec<String> = sal::get_genres_disabled(&conn)?.into_iter().map(|(_, genre)| genre).collect();
    let mut access = Access::new();
    access.disable_langs(langs);
    access.disable_genres(genres);

    let mut visitor = BookVisitor::new(access);
    for archive in archives {
        println!("{}", archive);
        algorithm::visit(archive, &mut visitor)?;
    }
    Ok(())
}
