use fb2parser::FictionBook;

use std::iter::FromIterator;
use std::collections::HashSet;

pub struct AccessGuard {
    disabled_genres: HashSet<String>,
    disabled_langs: HashSet<String>
}
impl AccessGuard {
    pub fn new() -> Self {
        AccessGuard {
            disabled_genres: HashSet::new(),
            disabled_langs: HashSet::new(),
        }
    }

    pub fn disable_langs(&mut self, langs: Vec<String>) {
        self.disabled_langs = HashSet::from_iter(langs);
    }

    pub fn disable_genres(&mut self, genres: Vec<String>) {
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

    pub fn is_allowed(&self, book: &FictionBook) -> bool {
        self.is_genre_allowed(book) && self.is_lang_allowed(book)
    }
}
