use tools;
use archive;
use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;
use algorithm::Visitor;
use visitor::acess::AccessGuard;
use visitor::author::Author;
use visitor::lang::Lang;
use visitor::genre::Genre;
use visitor::title::Title;
use visitor::sequence::Sequence;
use visitor::header::{Header, Show};

use std::collections::HashSet;

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

pub fn check_archive(archive: &str, quiet: bool) -> Fb2Result<()> {
    use std::io;
    use std::io::Write;
    let zip = archive::open(archive)?;
    let count = zip.len();
    let mut succ = 0;
    let mut curr = 0;
    if !quiet {
        print!("Progress:   %");
    }
    algorithm::apply_to_xml(zip, "*.fb2", |book, xml| {
        match tools::into_fb2(xml) {
            Ok(_) => succ += 1,
            Err(_) => {
                if !quiet {
                    println!();
                }
                println!(
                    "The {} file contained unsupported FB2 file {}",
                    archive,
                    &book
                )
            }
        }
        if !quiet {
            curr += 1;
            print!("\rProgress: {:3}%", 100 * (1 + curr) / count);
            io::stdout().flush().unwrap();
        }
    })?;
    if !quiet {
        println!("\nSucceeded {}/{} ({}%)", succ, count, 100 * succ / count);
    }
    Ok(())
}

fn handle<T>(archives: &Vec<&str>, mut visitor: T) -> Fb2Result<()>
    where T: algorithm::Visitor<FictionBook> + 'static
{
    for archive in archives {
        algorithm::visit_deprecated(archive, &mut visitor)?;
    }
    visitor.report();
    Ok(())
}

pub fn authors(archives: &Vec<&str>) -> Fb2Result<()> {
    handle(archives, Author::new(AccessGuard::new(), HashSet::new()))
}
pub fn langs(archives: &Vec<&str>) -> Fb2Result<()> {
    handle(archives, Lang::new(HashSet::new()))
}
pub fn titles(archives: &Vec<&str>) -> Fb2Result<()> {
    handle(archives, Title::new(AccessGuard::new(), HashSet::new()))
}
pub fn sequences(archives: &Vec<&str>) -> Fb2Result<()> {
    handle(archives, Sequence::new(AccessGuard::new(), HashSet::new()))
}
pub fn genres(archives: &Vec<&str>) -> Fb2Result<()> {
    handle(archives, Genre::new(HashSet::new()))
}


