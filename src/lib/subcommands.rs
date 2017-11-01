use out;
use tools;
use archive;
use algorithm::{apply_to_xml, apply_to_file};
use result::Fb2Result;
use result::Fb2Error;


use std::fs::File;

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let mut zip = archive::open(archive_name)?;
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        out::file_info(&file);
    }
    Ok(())
}

pub fn show_xml(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply_to_xml(zip, file_name, out::xml)
}

pub fn show_fb2(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply_to_xml(zip, file_name, out::fb2)
}

pub fn show_inf(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply_to_xml(zip, file_name, out::info)
}

pub fn show_zip(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = archive::open(archive_name)?;
    apply_to_file(zip, file_name, out::zip_info)
}


pub fn do_parse(file_name: &str) -> Fb2Result<()> {
    let mut file = File::open(file_name).map_err(|io| Fb2Error::Io(io))?;
    let xml = archive::load_header(&mut file)?;
    let fb2 = tools::into_utf8(xml).and_then(tools::into_fb2)?;
    println!("{}", fb2);
    Ok(())
}

pub fn do_check(archive_name: &str, quiet: bool) -> Fb2Result<()> {
    use std::io;
    use std::io::Write;
    let zip = archive::open(archive_name)?;
    let count = zip.len();
    let mut succ = 0;
    let mut curr = 0;
    if !quiet {
        print!("Progress:   %");
    }
    apply_to_xml(zip, "*", |file_name, xml| {
        match tools::into_fb2(xml) {
            Ok(_) => succ += 1,
            Err(_) => {
                if !quiet {
                    println!();
                }
                println!(
                    "./fb2lib {} show xml {} > {}",
                    archive_name,
                    &file_name,
                    &file_name
                )
            }
        }
        if !quiet {
            curr += 1;
            print!("\rProgress: {:3}%", 100 * (1 + curr) / count);
            io::stdout().flush()?;
        }
        Ok(())
    })?;
    if !quiet {
        println!("\nSucceeded {}/{} ({}%)", succ, count, 100 * succ / count);
    }
    Ok(())
}
