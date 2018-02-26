use sal;
use algorithm;

use sal::Save;
use result::Fb2Result;
use algorithm::Visitor;
use fb2parser::FictionBook;

use visitor::acess;
use visitor::author::Author;
use visitor::lang::Lang;
use visitor::title::Title;
use visitor::sequence::Sequence;

use std::path;

/************************************ RESET HANDLERS *******************************************/
pub fn reset_all(db_file_name: &str) -> Fb2Result<()> {
    println!("reset({})", db_file_name);
    sal::reset_tables(db_file_name)
}
/************************************* LOAD HANDLERS *******************************************/
pub fn load_authors(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let handled = sal::select_people(&conn)?;
    let visitor = Author::new(access, handled);
    handle(&conn, force, archives, visitor)
}
pub fn load_langs(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let langs = sal::select_languages(&conn)?;
    let visitor = Lang::new(langs);
    handle(&conn, force, archives, visitor)
}

pub fn load_titles(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let ignore = sal::select_titles(&conn)?;
    let visitor = Title::new(access, ignore);
    handle(&conn, force, archives, visitor)
}

pub fn load_sequences(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let ignore = sal::select_sequences(&conn)?;
    let visitor = Sequence::new(access, ignore);
    handle(&conn, force, archives, visitor)
}
/************************************* SHOW HANDLERS *******************************************/
pub fn show_authors(db: &str, pattern: &str) -> Fb2Result<()> {
    let re = algorithm::make_regex(pattern)?;
    let conn = sal::get_connection(db)?;
    let authors = sal::select_authors(&conn)?;
    for (id, use_id, name) in authors {
        if re.is_match(&name) {
            println!("{:>6} {:?} {}", id, use_id, name);
        }
    }
    Ok(())
}
pub fn show_titles(db: &str, pattern: &str) -> Fb2Result<()> {
    let re = algorithm::make_regex(pattern)?;
    let conn = sal::get_connection(db)?;
    let authors = sal::select_authors(&conn)?;
    for (id, use_id, name) in authors {
        if re.is_match(&name) {
            println!("{:>6} {:?} {}", id, use_id, name);
        }
    }
    Ok(())
}
pub fn show_sequences(db: &str, pattern: &str) -> Fb2Result<()> {
    let re = algorithm::make_regex(pattern)?;
    let conn = sal::get_connection(db)?;
    let authors = sal::select_authors(&conn)?;
    for (id, use_id, name) in authors {
        if re.is_match(&name) {
            println!("{:>6} {:?} {}", id, use_id, name);
        }
    }
    Ok(())
}
/************************************ PRIVATE HANDLERS *****************************************/
pub fn alias_authors(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::link_authors(&conn, source, destination));
    Ok(())
}
/************************************ PRIVATE HANDLERS *****************************************/
fn create_access_guard(conn: &sal::Connection)-> Fb2Result<acess::AccessGuard> {
    let langs: Vec<String> = sal::get_languages_disabled(&conn)?;
    let genres: Vec<String> = sal::get_genre_codes_disabled(&conn)?;
    let mut access = acess::AccessGuard::new();
    access.disable_langs(langs);
    access.disable_genres(genres);
    Ok(access)
}

fn is_complete(status: sal::STATUS) -> bool {
    match status {
        sal::STATUS::STARTED => false,
        sal::STATUS::VISITED => false,
        sal::STATUS::COMPLETE => true,
        sal::STATUS::FAILURE => false,
        sal::STATUS::UNKNOWN => false,
    }
}

fn visit<T>(conn: &sal::Connection, archive: &str, name: &str, force: bool, visitor: &mut T) -> Fb2Result<()>
    where T: Visitor<FictionBook> + Save<FictionBook> + 'static
{
    print!("Processing {}", &name);
    let task = visitor.task();
    let status = sal::get_archive_status(&conn, name, task)?;
    if force || !is_complete(status) {
        sal::set_archive_started(conn, name, task)?;
        print!(".");
        match algorithm::visit(archive, visitor) {
            Ok(()) => {
                sal::set_archive_visited(conn, name, task)?;
                print!(".");
            },
            Err(e) => {
                sal::set_archive_failure(conn, name, task)?;
                println!("{}", e);
                return Err(e);
            }
        }
        let (added, total) = (visitor.get_new_count(), visitor.get_total_count());
        match visitor.save(&conn) {
            Ok(()) => {
                sal::set_archive_complete(conn, name, task)?;
                print!(".");
            },
            Err(e) => {
                sal::set_archive_failure(conn, name, task)?;
                println!("{}", e);
                return Err(e);
            }
        }
        let added = format!("{}/{}", added, total);
        println!("Done.\t Added {:>11}. Current stored recods count {}", added, visitor.get_stored_count());
    } else {
        println!("...Skiped.");
    }
    Ok(())
}

fn handle<T>(conn: &sal::Connection, force: bool, archives: &Vec<&str>, mut visitor: T) -> Fb2Result<()>
    where T: Visitor<FictionBook> + Save<FictionBook> + 'static
{
    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        visit(&conn, archive, name, force, &mut visitor)?;
    }
    Ok(())
}

