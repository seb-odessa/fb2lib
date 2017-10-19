use out;
use tools;
use archive::{open, load_fb2};
use zip::read::ZipFile;
use algorithm::apply;

use result::Fb2Result;
use result::Fb2Error;


use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io;

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, "*", out::file_info)
}

pub fn show_xml(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, file_name, out::xml)
}

pub fn show_fb2(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, file_name, out::fb2)
}

pub fn show_inf(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, file_name, out::info)
}

fn read_file(file_name: &str) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut file = File::open(file_name)?;
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn do_parse(file_name: &str) -> Fb2Result<()> {
    let fb = match read_file(file_name) {
        Ok(xml) => tools::into_utf8(xml).and_then(tools::into_fb2),
        Err(_) => Err(Fb2Error::FileNotFound(String::from(file_name))),
    }?;
    println!("{}", fb);
    Ok(())
}



pub fn do_check(archive_name: &str, quiet: bool) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    let count = zip.len();
    let mut succ = 0;
    let mut curr = 0;
    if !quiet {
        print!("Progress:   %");
    }
    apply(zip, "*", |file: &mut ZipFile| {
        match load_fb2(file) {
            Ok(_) => succ += 1,
            Err(_) => {
                if !quiet {
                    println!();
                }
                println!(
                    "./fb2lib {} show xml {} > {}",
                    archive_name,
                    &file.name(),
                    &file.name()
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
