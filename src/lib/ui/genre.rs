use sal;
use result::Fb2Result;
use fb2parser::FictionBook;
use tools;
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
    let mut unknown = HashMap::new();    
    for (code, count) in &collector.genres {
        if let Some((_, _)) = sal::get_genre_name(&conn, code)? {
            total += count;
        } else {
            unknown.insert(code, count);
        }
    }
    for (code, count) in &unknown {
        println!("{} - {}", code, count);
    }
    println!("Total books was processed: {}", total);
    println!("Total unique genres was found {}", &collector.genres.len());
    println!("Unknown genres was found {}", &unknown.len());    
    Ok(())
}

pub fn display(db_file_name: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    println!("======== DISABLED ========");
    let mut grp = String::new();
    for (group, name) in sal::get_genres_disabled(&conn)? {
        if grp != group {
            println!("{}", tools::capitalize(group.clone()));
            grp = group.clone();
        }
        println!("\t{}", tools::capitalize(name));
    }
    println!("======== ENABLED ========");
    for (group, name) in sal::get_genres_enabled(&conn)? {
        if grp != group {
            println!("{}", tools::capitalize(group.clone()));
            grp = group.clone();
        }
        println!("\t{}", tools::capitalize(name));
    }
    println!("");
    Ok(())
}

pub fn enable(db_file_name: &str, name: &str, is_grp: bool) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    let re = algorithm::make_regex(name)?;
    if is_grp {
        for group in sal::get_genre_groups_disabled(&conn)? {
            if re.is_match(&group) {
                println!("The genre group '{}' was enabled", tools::capitalize(group));
            }
        }
    } else {
        for (_, genre) in sal::get_genres_disabled(&conn)? {
            if re.is_match(&genre) {
                println!("The genre '{}' was enabled", tools::capitalize(genre));
            }
        }
    }
    Ok(())
}

pub fn disable(db_file_name: &str, name: &str, is_grp: bool) -> Fb2Result<()> {
    let conn = sal::get_connection(db_file_name)?;
    let re = algorithm::make_regex(name)?;
    if is_grp {
        for group in sal::get_genre_groups_enabled(&conn)? {
            if re.is_match(&group) {
                println!("The genre group '{}' was disabled", tools::capitalize(group));
            }
        }
    } else {
        for (_, genre) in sal::get_genres_enabled(&conn)? {
            if re.is_match(&genre) {
                println!("The genre '{}' was disabled", tools::capitalize(genre));
            }
        }
    }
    Ok(())
}