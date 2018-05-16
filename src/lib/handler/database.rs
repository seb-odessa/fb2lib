use sal;
use archive;
use algorithm;

use sal::Save;
use algorithm::Visitor;
use result::{Fb2Result, Fb2Error};
use fb2parser::FictionBook;


use visitor::acess;
use visitor::author::Author;
use visitor::name::Name;
use visitor::lang::Lang;
use visitor::title::Title;
use visitor::sequence::Sequence;
use visitor::description::Description;
use visitor::collector::Collector;

use std::path;

/************************************ RESET HANDLERS *******************************************/
pub fn reset(db_file_name: &str, subsystem: &str) -> Fb2Result<()> {
    println!("reset({}, {})", db_file_name, subsystem);
    match subsystem {
        "torrent" => sal::reset(db_file_name, sal::SUBSYSTEM::TORRENT),
        "progress" => sal::reset(db_file_name, sal::SUBSYSTEM::PROGRESS),
        "filter" => sal::reset(db_file_name, sal::SUBSYSTEM::FILTER),
        "lang" => sal::reset(db_file_name, sal::SUBSYSTEM::LANGUAGE),
        "genre" => sal::reset(db_file_name, sal::SUBSYSTEM::GENRE),
        "author" => sal::reset(db_file_name, sal::SUBSYSTEM::PEOPLE),
        "sequence" => sal::reset(db_file_name, sal::SUBSYSTEM::SEQUENCES),
        "title" => sal::reset(db_file_name, sal::SUBSYSTEM::TITLES),
        "desc" => sal::reset(db_file_name, sal::SUBSYSTEM::DESC),
        _ => Err(Fb2Error::Custom(String::from("Unknown Subsystem")))
    }
}
/************************************* LOAD HANDLERS *******************************************/
pub fn load_authors(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let handled = sal::select_people(&conn)?;
    let visitor = Author::new(access, handled);
    handle(&conn, force, archives, visitor)
}
pub fn load_names(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let handled = sal::load_names(&conn)?;
    let visitor = Name::new(access, handled);
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
    let ignore = sal::load_titles(&conn)?;
    let visitor = Title::new(access, ignore);
    handle(&conn, force, archives, visitor)
}

pub fn load_sequences(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let ignore = sal::load_sequences(&conn)?;
    let visitor = Sequence::new(access, ignore);
    handle(&conn, force, archives, visitor)
}

pub fn load_dictionary(db: &str, force: bool, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let visitor = Collector::new(access, &conn)?;
    handle(&conn, force, archives, visitor)
}

pub fn load_descriptions(db: &str, archives: &Vec<&str>) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let access = create_access_guard(&conn)?;
    let mut visitor = Description::new(conn, access)?;

    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        print!("Processing {}", &name);
        visitor.select_archive(name)?;
        let task = visitor.task();
        let status = visitor.get_stat()?;
        if !is_complete(status) {
            print!(".");
            let result = visitor.set_stat(sal::STATUS::STARTED)
                .and_then(|()| archive::open(archive))
                .and_then(|zip| algorithm::visit_all(&zip, &mut visitor));

            match result {
                Ok(()) => {
                    visitor.set_stat(sal::STATUS::VISITED)?;
                    print!(".");
                },
                Err(e) => {
                    visitor.set_stat(sal::STATUS::FAILURE)?;
                    println!("{}", e);
                    return Err(e);
                }
            }

            let (added, total) = (visitor.get_new_count(), visitor.get_count());
            match visitor.save_collected() {
                Ok(()) => {
                    visitor.set_stat(sal::STATUS::COMPLETE)?;
                    print!(".");
                },
                Err(e) => {
                    visitor.set_stat(sal::STATUS::FAILURE)?;
                    println!("{}", e);
                    return Err(e);
                }
            }
            let added = format!("{}/{}", added, total);
            println!("Done.\t Added {:>11}. Current stored recods count {}", added, visitor.get_stored_count());
        } else {
            println!("...Skipped.");
        }
    }
    Ok(())
}

/************************************* SHOW HANDLERS *******************************************/
pub fn show_authors(db: &str, pattern: &str) -> Fb2Result<()> {
    let re = algorithm::make_regex(pattern)?;
    let conn = sal::get_connection(db)?;
    let authors = sal::select_authors_joined(&conn)?;
    for (id, src_name, dst_name) in authors {
        if re.is_match(&src_name) {
            println!("{:>6} {} [{}]", id, dst_name, src_name);
        }
    }
    Ok(())
}
pub fn show_titles(db: &str, pattern: &str) -> Fb2Result<()> {
    let re = algorithm::make_regex(pattern)?;
    let conn = sal::get_connection(db)?;
    let titles = sal::select_titles_joined(&conn)?;
    for (id, src_name, dst_name) in titles {
        if re.is_match(&src_name) {
            println!("{:>6} {} [{}]", id, dst_name, src_name);
        }
    }
    Ok(())
}
pub fn show_sequences(db: &str, pattern: &str) -> Fb2Result<()> {
    let re = algorithm::make_regex(pattern)?;
    let conn = sal::get_connection(db)?;
    let titles = sal::select_sequences_joined(&conn)?;
    for (id, src_name, dst_name) in titles {
        if re.is_match(&src_name) {
            println!("{:>6} {} [{}]", id, dst_name, src_name);
        }
    }
    Ok(())
}
/********************************* LINK MANAGEMENT **************************************/
pub fn mk_link_authors(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::link_authors(&conn, source, destination));
    Ok(())
}
pub fn mk_link_titles(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::link_titles(&conn, source, destination));
    Ok(())
}
pub fn mk_link_sequences(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::link_sequences(&conn, source, destination));
    Ok(())
}
pub fn rm_link_authors(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::unlink_authors(&conn, source, destination));
    Ok(())
}
pub fn rm_link_titles(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::unlink_titles(&conn, source, destination));
    Ok(())
}
pub fn rm_link_sequences(db: &str, src: &str, dst: &str) -> Fb2Result<()> {
    let conn = sal::get_connection(db)?;
    let source: i64 = src.trim().parse().ok().unwrap_or_default();
    let destination: i64 = dst.trim().parse().unwrap_or_default();
    println!("{} -> {} : {:?}", src, dst, sal::unlink_sequences(&conn, source, destination));
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

fn visit<'a, T>(conn: &sal::Connection, archive: &str, name: &str, force: bool, visitor: &mut T) -> Fb2Result<()>
    where T: Visitor<'a, Type=FictionBook> + Save + 'static
{
    print!("Processing {}", &name);
    let task = visitor.task();
    let status = sal::get_archive_status(&conn, name, task)?;
    if force || !is_complete(status) {
        visitor.set_status(&conn, name, sal::STATUS::STARTED)?;
        print!(".");
        match algorithm::visit_books(archive, visitor) {
            Ok(()) => {
                visitor.set_status(&conn, name, sal::STATUS::VISITED)?;
                print!(".");
            },
            Err(e) => {
                visitor.set_status(&conn, name, sal::STATUS::FAILURE)?;
                println!("{}", e);
                return Err(e);
            }
        }
        let (added, total) = (visitor.get_new_count(), visitor.get_count());
        match visitor.save(&conn) {
            Ok(()) => {
                visitor.set_status(&conn, name, sal::STATUS::COMPLETE)?;
                print!(".");
            },
            Err(e) => {
                visitor.set_status(&conn, name, sal::STATUS::FAILURE)?;
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

fn handle<'a, T>(conn: &sal::Connection, force: bool, archives: &Vec<&str>, mut visitor: T) -> Fb2Result<()>
    where T: Visitor<'a, Type=FictionBook> + Save + 'static
{
    for archive in archives {
        let name = path::Path::new(archive).file_name().unwrap_or_default().to_str().unwrap_or_default();
        visit(&conn, archive, name, force, &mut visitor)?;
    }
    Ok(())
}
