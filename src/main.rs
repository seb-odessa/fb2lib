extern crate lib;
extern crate zip;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};
use std::error::Error;
use lib::result::Fb2Error;
use lib::subcommands::{do_ls, do_cat};


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
                .arg(
                    Arg::with_name(ARCHIVE)
                        .help("Zip archive with books in FB2 format")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name(CMD_CAT)
                .about("Concatenate files and print on the standard output")
                .arg(
                    Arg::with_name(ARCHIVE)
                        .help("Zip archive with books in FB2 format")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name(FILE)
                        .help("File in FB2 format")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    let result = match app.subcommand() {
        (CMD_LS, Some(cmd)) => {
            let archive_name = cmd.value_of(ARCHIVE).unwrap();
            do_ls(&archive_name)
        }
        (CMD_CAT, Some(cmd)) => {
            let archive_name = cmd.value_of(ARCHIVE).unwrap();
            let book_name = cmd.value_of(FILE).unwrap();
            do_cat(&archive_name, &book_name)
        }
        _ => Err(Fb2Error::UnsupportedSubCommand),
    };

    match result {
        Ok(_) => {}
        Err(e) => println!("{}", e.description()),
    }
}
