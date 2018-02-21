use out;
use tools;
use archive;
use algorithm;
use result::Fb2Result;

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
