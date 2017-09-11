extern crate zip;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};
use std::error::Error;
use std::io::{Read, ErrorKind};
use zip::result::ZipResult;


const VERSION: &'static str = "v0.1.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";
const ARCHIVE: &'static str = "fb_archive.zip";
const FILE: &'static str = "fb_book.fb2";

const CMD_LS: &'static str = "ls";
const CMD_CAT: &'static str = "cat";

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let program = std::path::Path::new(&arguments[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let app = App::new(program)
        .version(VERSION)
        .author(AUTHOR)
        .about("FictionBook Library Archive Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(CMD_LS)
                .about("List archive contents")
                .arg(Arg::with_name(ARCHIVE)
                    .help("Zip archive with books in FB2 format")
                    .required(true)
                    .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_CAT)
                .about("Concatenate files and print on the standard output")
                .arg(Arg::with_name(ARCHIVE)
                    .help("Zip archive with books in FB2 format")
                    .required(true)
                    .index(1),
                )
                .arg(Arg::with_name(FILE)
                    .help("File in FB2 format")
                    .required(true)
                    .index(2),
                ),
        )
        .get_matches();

    let result = match app.subcommand() {
        (CMD_LS, Some(cmd)) => {
            let archive_name = cmd.value_of(ARCHIVE).unwrap();
            make(do_ls(&archive_name))
        },
        (CMD_CAT, Some(cmd)) => {
            let archive_name = cmd.value_of(ARCHIVE).unwrap();
            let book_name = cmd.value_of(FILE).unwrap();
            make(do_cat(&archive_name, &book_name))
        },
        _ => {
            Err(std::io::Error::new(ErrorKind::Other, "SubCommand not found"))
        },
    };

    match result {
        Ok(_) => {},
        Err(e) => println!("{}", e.description()),
    }
}

fn make(src: ZipResult<()>) -> std::result::Result<(), std::io::Error> {
    match src {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(ErrorKind::Other, e.description())),
    }
}


fn do_ls(archive_name: &str) -> ZipResult<()> {
    let file = std::fs::File::open(&std::path::Path::new(archive_name))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let zip_file = archive.by_index(i)?;
        println!(
            "Filename: {}, {} / {}",
            zip_file.name(),
            zip_file.compressed_size(),
            zip_file.size()
        );
    }
    Ok(())
}

fn do_cat(archive_name: &str, file_name: &str) -> ZipResult<()> {
    let file = std::fs::File::open(&std::path::Path::new(archive_name))?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let zip_file = archive.by_index(i)?;
        println!(
            "Filename: {}, {} / {}",
            zip_file.name(),
            zip_file.compressed_size(),
            zip_file.size()
        );
        if zip_file.name() == file_name {
            let mut bytes = vec!();
            for byte in zip_file.bytes().take(8192) {
                match byte {
                    Ok(b) => bytes.push(b),
                    Err(e) => break,
                }
            }

            match String::from_utf8(bytes) {
                Ok(utf8) => {
                    print!("{}", utf8)
                },
                Err(e) => {
                        println!("Non UTF8");
                }
            };
        }
    }
    Ok(())
}