use sal;
use result::Fb2Result;
use fb2parser::FictionBook;
use algorithm;

use std::collections::HashSet;

pub struct GenreCollector {
    genres: HashSet<String>
}
impl GenreCollector {
    pub fn new() -> Self {
        GenreCollector{
            genres: HashSet::new()
        }
    }
}
impl algorithm::Visitor<FictionBook> for GenreCollector {
    fn visit(&mut self, book: &FictionBook) {
        for genre in book.get_book_genres().into_iter() {
            for genre in genre.split(",") {
                self.genres.insert(genre.trim().to_lowercase());
            }
        }
    }
}


pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let mut collector = GenreCollector::new();
    for archive in archives {
        println!("{}", archive);
        algorithm::visit(archive, &mut collector)?;
    }
    for code in &collector.genres.clone() {
        if let Some(ref code) = sal::get_genre_name(&conn, code)? {
            collector.genres.remove(code);
        }
    }
    for code in &collector.genres {
        println!("{}", code);
    }
    Ok(())
}

