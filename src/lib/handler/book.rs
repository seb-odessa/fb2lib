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
                if !genre.is_empty() && self.disabled_genres.contains(&genre){
                    return false;
                }
            }
        }
        true
    }

    fn is_lang_allowed(&self, book: &FictionBook) -> bool {
        let lang = book.get_book_lang();
        !self.disabled_langs.contains(lang.as_str()) || lang.is_empty()
    }

    fn is_allowed(&self, book: &FictionBook) -> bool {
        self.is_genre_allowed(book) && self.is_lang_allowed(book)
    }
}

struct BookVisitor {
    count: usize,
    access: Access
}
impl BookVisitor {
    fn new(access: Access) -> Self {
        BookVisitor {
            count: 0,
            access: access
        }
    }
}
impl algorithm::Visitor<FictionBook> for BookVisitor {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            self.count += 1;
            let genres = format!("{}", book.get_book_genres().join(", "));
            let authors = format!("{}", book.get_book_authors_names().join(", "));
            let title = book.get_book_title();
            let sequences = format!("{}", book.get_book_sequences_desc().join(", "));
            let date = book.get_book_date();
            println!("{} : {} : {} : {} : {} : {}", self.count, genres, authors, title, sequences, date);
        }
    }
}

pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let langs: Vec<String> = sal::get_languages_disabled(&conn)?;
    let genres: Vec<String> = sal::get_genre_codes_disabled(&conn)?;

    println!("Disabled genres {}", genres.join(", "));

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
