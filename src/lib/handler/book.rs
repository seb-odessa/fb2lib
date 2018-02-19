use sal;
use sal::{Save, STATUS};
use algorithm;
use algorithm::Visitor;
use fb2parser::FictionBook;

use result::Fb2Result;
use visitor::acess;
use visitor::author::Author;
use visitor::book::Book;
use visitor::lang::Lang;
use visitor::genre::Genre;
use visitor::title::Title;

use std::path;


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

fn visit_and_save<T>(conn: &sal::Connection, archive: &str, name: &str, visitor: &mut T) -> Fb2Result<()> 
    where T: Visitor<FictionBook> + Save<FictionBook> + 'static
{
    let task = visitor.task();
    sal::set_archive_started(conn, name, task)?;

    if algorithm::visit(archive, visitor).is_ok() {
        sal::set_archive_visited(conn, name, task)?;
    } else {
        return sal::set_archive_failure(conn, name, task);
    }
    if visitor.save(&conn).is_ok() {
        sal::set_archive_complete(conn, name, task)
    } else {
        sal::set_archive_failure(conn, name, task)
    }
}

fn is_complete(status: sal::STATUS) -> bool {
    match status {
            STATUS::STARTED => false,
            STATUS::VISITED => false,
            STATUS::COMPLETE => true,
            STATUS::FAILURE => false,
            STATUS::UNKNOWN => false,
        }
}

fn handle<T>(conn: &sal::Connection, save: bool, force: bool, archives: &Vec<&str>, mut visitor: T) -> Fb2Result<()> 
    where T: Visitor<FictionBook> + Save<FictionBook> + 'static
{
    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        let task = visitor.task();
        if save {
            print!("{} ", &name);
        }        
        let status = sal::get_archive_status(&conn, name, task)?;
        if save && (force || !is_complete(status)) {
            visit_and_save(&conn, archive, name, &mut visitor)?;
            println!("handled.");
        } else if !save {
            algorithm::visit(archive, &mut visitor)?;
        } else {
            println!("skiped.");
        }
    }
    if !save {
        visitor.report();
    }
    Ok(())
}

pub fn authors(db: &str, save: bool, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access_guard = create_access_guard(&conn)?;
    let ignore = sal::select_people(&conn)?;
    let visitor = Author::new(access_guard, ignore);
    handle(&conn, save, force, archives, visitor)
}

pub fn langs(db: &str, save: bool, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let langs = sal::select_languages(&conn)?;
    let visitor = Lang::new(langs);
    handle(&conn, save, force, archives, visitor)
}

pub fn titles(db: &str, save: bool, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let ignore = sal::select_title(&conn)?;
    let visitor = Title::new(ignore);
    handle(&conn, save, force, archives, visitor)
}

pub fn genres(db: &str, only_unknown: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let ignore: Vec<String> = if only_unknown {
        sal::get_genre_codes(&conn)?
    } else {
        Vec::new()
    };
    let visitor = Genre::new(ignore);
    handle(&conn, false, false, archives, visitor)
}


