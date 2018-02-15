use sal;
use algorithm;
use result::Fb2Result;
use visitor::acess;
use visitor::author::{Author, BookVisitor};
use visitor::book::Book;
use visitor::lang::Lang;
use visitor::genre::Genre;

use std::path;

use sal::TASK::{AUTHOR, LANGUAGE, GENRE};
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

fn visit(conn: &sal::Connection, archive: &str, name: &str, save: bool, visitor: &mut BookVisitor, task: sal::TASK) -> Fb2Result<()> {
    if save {
        sal::set_archive_incomplete(conn, name, task)?;
        if algorithm::visit(archive, visitor).is_err() {
            sal::set_archive_failure(conn, name, task)
        } else {
            sal::set_archive_complete(conn, name, task)
        }
    } else {
        algorithm::visit(archive, visitor)
    }
}

fn is_visit_required(save: bool, force: bool, status: sal::STATUS) -> bool {
    !save || 
    force || 
    match status {
            STATUS::COMPLETE => false,
            STATUS::IGNORE => false,
            STATUS::FAILURE => true,
            STATUS::INCOMPLETE => false,
            STATUS::UNKNOWN => true,
        }
}

pub fn authors(db: &str, save: bool, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access_guard = create_access_guard(&conn)?;
    let authors = sal::select_people(&conn)?;
    let mut visitor = Author::new(access_guard, authors);
    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        let status = sal::get_archive_status(&conn, name, AUTHOR)?;
        if is_visit_required(save, force, status) {
            visit(&conn, archive, name, save, &mut visitor, AUTHOR)?;
        }
    }
    if save {
        visitor.save(&conn)
    } else {
        visitor.report()
    }
}

pub fn langs(db: &str, save: bool, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let mut visitor = Lang::new();
    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        let status = sal::get_archive_status(&conn, name, LANGUAGE)?;
        if is_visit_required(save, force, status) {
            visit(&conn, archive, name, save, &mut visitor, LANGUAGE)?;
        }
    }
    if save {
        visitor.save(&conn)
    } else {
        visitor.report()
    }
}

pub fn genres(db: &str, only_unknown: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let genres: Vec<String> = sal::get_genre_codes(&conn)?;
    let mut visitor = Genre::new(genres);
    for archive in archives {
        visit(&conn, archive, "", false, &mut visitor, GENRE)?;
    }
    visitor.report(only_unknown)
}

