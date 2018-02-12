use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess;
use visitor::author::{Author, AuthorVisitor};
use visitor::list::Book;

use std::path;

use sal::TASK::AUTHOR;
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


fn visit(conn: &sal::Connection, path: &str, name: &str, load: bool, visitor: &mut AuthorVisitor) -> Fb2Result<()> {
    if load {
        sal::set_archive_incomplete(conn, name, AUTHOR)?;
        if algorithm::visit(path, visitor).is_err() {
            sal::set_archive_failure(conn, name, AUTHOR)
        } else {
            sal::set_archive_complete(conn, name, AUTHOR)
        }
    } else {
        algorithm::visit(path, visitor)
    }
}

pub fn authors(db: &str, load: bool, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access_guard = create_access_guard(&conn)?;
    let authors = sal::select_people(&conn)?;
    let mut visitor = Author::new(access_guard, authors);
    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        let status = sal::get_archive_status(&conn, name, AUTHOR)?;
        let visit_required = !load || force || match status {
            STATUS::COMPLETE => false,
            STATUS::IGNORE => false,
            STATUS::FAILURE => true,
            STATUS::INCOMPLETE => false,
            STATUS::UNKNOWN => true,
        };
        if visit_required {
            visit(&conn, archive, name, load, &mut visitor)?;
        }
    }

    if load {
        sal::insert_people(&conn, &visitor.authors)?;
    } else {
        visitor.report();
    }

    Ok(())
}