use types;
use archive;
use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;
use types::MutVisitor;
//use visitor::acess::AccessGuard;
//use visitor::author::Author;
//use visitor::lang::Lang;
//use visitor::genre::Genre;
//use visitor::title::Title;
//use visitor::sequence::Sequence;
use visitor::header::{Header, Show};

//use std::collections::HashSet;

pub fn show_zip(archive: &str, pattern: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    let mut visitor = Header::new(Show::Zip);
    algorithm::visit(&zip, pattern, &mut visitor)?;
    visitor.report();
    Ok(())
}

pub fn show_xml(archive: &str, pattern: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    let mut visitor = Header::new(Show::Xml);
    algorithm::visit(&zip, pattern, &mut visitor)?;
    visitor.report();
    Ok(())
}

pub fn show_fb2(archive: &str, pattern: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    let mut visitor = Header::new(Show::Fb2);
    algorithm::visit(&zip, pattern, &mut visitor)?;
    visitor.report();
    Ok(())
}

pub fn show_inf(archive: &str, pattern: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    let mut visitor = Header::new(Show::Inf);
    algorithm::visit(&zip, pattern, &mut visitor)?;
    visitor.report();
    Ok(())
}

pub fn show_bad(archive: &str, pattern: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    let mut visitor = Header::new(Show::Bad);
    algorithm::visit(&zip, pattern, &mut visitor)?;
    visitor.report();
    Ok(())
}

fn handle<'a, T>(archives: &Vec<&str>, mut visitor: T) -> Fb2Result<()>
    where T: types::MutVisitor<'a, Type=FictionBook> + 'static
{
    for archive in archives {
        algorithm::visit_books(archive, &mut visitor)?;
    }
    visitor.report();
    Ok(())
}

pub fn authors(_archives: &Vec<&str>) -> Fb2Result<()> {
//    handle(archives, Author::new(AccessGuard::new(), HashSet::new()))
    Ok(())
}
pub fn langs(_archives: &Vec<&str>) -> Fb2Result<()> {
    //handle(archives, Lang::new(HashSet::new()))
    Ok(())
}
pub fn titles(_archives: &Vec<&str>) -> Fb2Result<()> {
//    handle(archives, Title::new(AccessGuard::new(), HashSet::new()))
    Ok(())
}
pub fn sequences(_archives: &Vec<&str>) -> Fb2Result<()> {
//    handle(archives, Sequence::new(AccessGuard::new(), HashSet::new()))
    Ok(())
}
pub fn genres(_archives: &Vec<&str>) -> Fb2Result<()> {
//    handle(archives, Genre::new(HashSet::new()))
    Ok(())
}
