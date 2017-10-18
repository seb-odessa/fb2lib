use tools;
use archive::{apply, open, load_header, load_xml, load_fb2};
use result::Fb2Result;
use result::Fb2Error;
use zip::read::ZipFile;
use std::error::Error;

use tools::into_utf8;
use tools::into_fb2;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io;

fn print_file_info(file: ZipFile) -> Fb2Result<()> {
    println!(
        "{:16}{:10}{:10}",
        file.name(),
        file.size(),
        file.compressed_size()
    );
    Ok(())
}

pub fn do_ls(archive_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, "*", print_file_info)
}

fn print_xml(mut file: ZipFile) -> Fb2Result<()> {
    let xml = load_xml(&mut file)?;
    println!("{}", xml);
    Ok(())
}

pub fn show_xml(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, file_name, print_xml)
}

fn print_fb(mut file: ZipFile) -> Fb2Result<()> {
    let fb = load_header(&mut file).and_then(into_utf8).and_then(into_fb2)?;
    println!("{:#?}", fb);
    Ok(())
}

pub fn show_fb2(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, file_name, print_fb)
}

fn print_info(mut file: ZipFile) -> Fb2Result<()> {
    match load_fb2(&mut file) {
        Ok(fb) => println!("{:20}: {}", file.name(), fb),
        Err(err) => println!("{:20}: {}!!!", file.name(), err.description()),
    }
    Ok(())
}

pub fn show_inf(archive_name: &str, file_name: &str) -> Fb2Result<()> {
    let zip = open(archive_name)?;
    apply(zip, file_name, print_info)
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
    apply(zip, "*", |mut file| {
        match load_fb2(&mut file) {
            Ok(_) => succ += 1,
            Err(_) => {
                if !quiet {
                    println!();
                }
                println!(
                    "./fb2lib {} show xml {} > {}",
                    archive_name,
                    file.name(),
                    file.name()
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

