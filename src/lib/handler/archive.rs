use out;
use tools;
use archive;
use algorithm;
use fb2parser::FictionBook;
use result::Fb2Result;

use visitor::acess::AccessGuard;
use visitor::author::Author;
use visitor::lang::Lang;
use visitor::genre::Genre;
use visitor::title::Title;
use visitor::sequence::Sequence;

use std::collections::HashSet;

pub fn show_xml(archive: &str, book: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    algorithm::apply(zip, book, out::xml)
}

pub fn show_fb2(archive: &str, book: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    algorithm::apply(zip, book, out::fb2)
}

pub fn show_inf(archive: &str, book: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    algorithm::apply(zip, book, out::info)
}

pub fn show_zip(archive: &str, book: &str) -> Fb2Result<()> {
    let zip = archive::open(archive)?;
    algorithm::apply_to_file(zip, book, out::zip_info)
}

pub fn show_files(archive: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive)?;
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        out::file_info(&file);
    }
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
    algorithm::apply_to_xml(zip, "*", |book, xml| {
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
        algorithm::visit(archive, &mut visitor)?;
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


