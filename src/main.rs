extern crate zip;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};

use std::io::Read;
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

    let matches = App::new(program)
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
                    .index(1)
                .arg(Arg::with_name(ARCHIVE)
                    .help("File in FB2 format")
                    .required(true)
                    .index(2),
                ),

        )
        .get_matches();

    let result = if let Some(matches) = matches.subcommand_matches(CMD_LS) {
        let archive_name = matches.value_of(ARCHIVE).unwrap();
        do_ls(&archive_name);
    } else if let Some(matches) = matches.subcommand_matches(CMD_CAT) {
        let archive_name = matches.value_of(ARCHIVE).unwrap();
        let book_name = matches.value_of(FILE).unwrap();
        do_cat(&archive_name, &book_name);
    };
/*
    match (result) {
        Ok() => {},
        Err(err) => { println!("{}", err.description()); },
    }
*/
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

        let bytes = zip_file.bytes().take(1024);

        for byte in bytes {
            //            print!("{}", byte.unwrap() as char);
        }
        println!("");
    }

    Ok(())
}


fn do_cat(archive_name: &str, filename: &str) -> ZipResult<()> {

    Ok(())
}