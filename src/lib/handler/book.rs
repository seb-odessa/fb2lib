use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess;
use visitor::author::Author;
use visitor::list::Book;

use std::path;

use sal::LOADING::LANGUAGE;
use sal::STATUS;

fn create_access_guard(conn: &sal::Connection)-> Fb2Result<acess::AccessGuard> {
    let langs: Vec<String> = sal::get_languages_disabled(&conn)?;
    let genres: Vec<String> = sal::get_genre_codes_disabled(&conn)?;
    let mut access = acess::AccessGuard::new();
    access.disable_langs(langs);
    access.disable_genres(genres);
    Ok(access)
}

pub fn ls(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access_guard = create_access_guard(&conn)?;
    let genres = sal::get_genre_codes_and_groups(&conn)?;
    let mut visitor = Book::new(access_guard, genres);
    for archive in archives {
        algorithm::visit(archive, &mut visitor)?;
    }
    visitor.report();
    Ok(())
}

fn set_complete() -> Fb2Result<()> {

    Ok(())   
}

pub fn authors(db: &str, load: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access_guard = create_access_guard(&conn)?;
    let mut visitor = Author::new(access_guard);

    for archive in archives {
        let archive_name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        let status = sal::get_archive_status(&conn, archive_name, LANGUAGE)?;
        match status {
            STATUS::COMPLETE => {},
            STATUS::IGNORE => {},
            STATUS::FAILURE => {},
            STATUS::INCOMPLETE => {
                algorithm::visit(archive, &mut visitor)?;
            },
            STATUS::UNKNOWN => {
                algorithm::visit(archive, &mut visitor)?;
            },
        }        
    }

    if load {
        sal::insert_people(&conn, &visitor.authors)?;
    } else {
        visitor.report();
    }

    Ok(())
}