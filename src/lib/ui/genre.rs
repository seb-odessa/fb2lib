use sal;
use result::Fb2Result;
use fb2parser::FictionBook;
use algorithm;

use std::collections::HashMap;

pub struct GenreCollector {
    genres: HashMap<String, usize>
}
impl GenreCollector {
    pub fn new() -> Self {
        GenreCollector{
            genres: HashMap::new()
        }
    }
}
impl algorithm::Visitor<FictionBook> for GenreCollector {
    fn visit(&mut self, book: &FictionBook) {
        for genre in book.get_book_genres().into_iter() {
            for genre in genre.split(",") {
                let genre = genre.trim().to_lowercase();
                 let counter = self.genres.entry(genre).or_insert(0);
                *counter += 1;
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
    let mut total = 0;
    for (code, count) in &collector.genres.clone() {
        if let Some(ref code) = sal::get_genre_name(&conn, code)? {
            collector.genres.remove(code);
            total += count;
        }
    }
    for (code, count) in &collector.genres {
        println!("{} - {}", code, count);
    }
    println!("Total: {}", total);
    Ok(())
}

