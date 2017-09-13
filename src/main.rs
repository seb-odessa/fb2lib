extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};
use std::error::Error;
use lib::result::Fb2Error;
use lib::subcommands::{do_ls, do_info};


const VERSION: &'static str = "v0.1.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";
const ARCHIVE: &'static str = "fb_archive.zip";
const FILE: &'static str = "fb_book.fb2";

const CMD_LS: &'static str = "ls";
const CMD_INFO: &'static str = "info";

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
        .arg(
            Arg::with_name(ARCHIVE)
                .help("Zip archive with books in FB2 format")
                .required(true)
                .index(1),
        )
        .subcommand(SubCommand::with_name(CMD_LS).about("List archive contents"))
        .subcommand(SubCommand::with_name(CMD_INFO).about("Print description of fb2 file")
                .arg(
                    Arg::with_name(FILE)
                        .help("File in FB2 format")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let archive = app.value_of(ARCHIVE).unwrap();
    let result = match app.subcommand() {
        (CMD_LS, Some(_)) => {
            do_ls(&archive)
        },
        (CMD_INFO, Some(cmd)) => {
            let book = cmd.value_of(FILE).unwrap();
            do_info(&archive, &book)
        }
        _ => Err(Fb2Error::UnsupportedSubCommand),
    };

    match result {
        Ok(_) => {}
        Err(e) => println!("{}", e.description()),
    }
}
