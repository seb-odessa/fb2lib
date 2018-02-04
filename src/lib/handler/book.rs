use sal;
use result::Fb2Result;
use fb2parser::FictionBook;
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

struct BookList {
    access: Access,
    books: Vec<String>,
}
impl BookList {
    fn new(access: Access) -> Self {
        BookList {
            access: access,
            books: Vec::new(),
        }
    }
    fn report(&self) {
        for row in &self.books {
            println!("{}", row);
        }
    }
}
impl algorithm::Visitor<FictionBook> for BookList {
    fn visit(&mut self, book: &FictionBook) {
        let mut count: usize = 0;
        if self.access.is_allowed(book) {
            count += 1;
            let genres = format!("{}", book.get_book_genres().join(", "));
            let authors = format!("{}", book.get_book_authors_names().join(", "));
            let title = book.get_book_title();
            let sequences = format!("{}", book.get_book_sequences_desc().join(", "));
            let date = book.get_book_date();
            let row = format!("{} : {} : {} : {} : {} : {}", count, genres, authors, title, sequences, date);
            self.books.push(row);
        }
    }
}

struct BookAuthors {
    access: Access,
    authors: HashSet<(String, String, String, String)>,
}
impl BookAuthors {
    fn new(access: Access) -> Self {
        BookAuthors {
            access: access,
            authors: HashSet::new(),
        }
    }
    fn report(&self) {
        for author in &self.authors {
            let (first_name, middle_name, last_name, nick_name) = author.clone();
            if first_name.is_empty() && middle_name.is_empty() && last_name.is_empty() {
                println!("{}", nick_name);
            } else {
                println!("{} {} {}", first_name, middle_name, last_name);
            }
        }
    }
}
impl algorithm::Visitor<FictionBook> for BookAuthors {
    fn visit(&mut self, book: &FictionBook) {
        if self.access.is_allowed(book) {
            for author in book.get_book_authors() {
                self.authors.insert(author);
            }

        }
    }
}

fn get_access(db: &str)-> Fb2Result<Access> {
    let conn = sal::get_connection(db)?;
    let langs: Vec<String> = sal::get_languages_disabled(&conn)?;
    let genres: Vec<String> = sal::get_genre_codes_disabled(&conn)?;
    let mut access = Access::new();
    access.disable_langs(langs);
    access.disable_genres(genres);
    Ok(access)
}

pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let mut visitor = BookList::new(get_access(db)?);
    for archive in archives {
        algorithm::visit(archive, &mut visitor)?;
    }
    visitor.report();
    Ok(())
}


pub fn authors(db: &str, load: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let mut visitor = BookAuthors::new(get_access(db)?);
    for archive in archives {
        algorithm::visit(archive, &mut visitor)?;
    }
    if load {

    } else {
        visitor.report();
    }

    Ok(())
}