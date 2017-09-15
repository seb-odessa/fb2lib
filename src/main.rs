extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};
use std::error::Error;
use lib::result::Fb2Error;
use lib::subcommands::*;


const VERSION: &'static str = "v0.1.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";
const ARCHIVE: &'static str = "fb_archive.zip";
const FILE: &'static str = "fb_book.fb2";

const CMD_LS: &'static str = "ls";
const CMD_DESC: &'static str = "desc";
const CMD_FB: &'static str = "fb";
const CMD_INFO: &'static str = "info";

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let program = std::path::Path::new(&arguments[0]).file_name().unwrap().to_str().unwrap();
    let archive = Arg::with_name(ARCHIVE).help("Zip archive with books in FB2 format").required(true).index(1);
    let book = Arg::with_name(FILE).help("File in FB2 format").required(true).index(1);           

    let app = App::new(program)
        .version(VERSION)
        .author(AUTHOR)
        .about("FictionBook Library Archive Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(archive)
        .subcommand(SubCommand::with_name(CMD_LS).about("List archive contents"))
        .subcommand(SubCommand::with_name(CMD_DESC).about("Print XML content of the fb2 description").arg(book.clone()))
        .subcommand(SubCommand::with_name(CMD_FB).about("Print parsed FictionBook structure").arg(book.clone()))
        .subcommand(SubCommand::with_name(CMD_INFO).about("Print human readable info for the fb2 file").arg(book.clone()))
        .get_matches();

    let archive = app.value_of(ARCHIVE).unwrap();
    let result = match app.subcommand() {
        (CMD_LS, Some(_)) => do_ls(&archive),
        (CMD_DESC,  Some(cmd)) => do_desc(&archive, &cmd.value_of(FILE).unwrap()),
        (CMD_FB,    Some(cmd)) => do_fb(&archive, &cmd.value_of(FILE).unwrap()),
        (CMD_INFO,  Some(cmd)) => do_info(&archive, &cmd.value_of(FILE).unwrap()),
        _ => Err(Fb2Error::UnsupportedSubCommand),
    };

    match result {
        Ok(_) => {}
        Err(e) => println!("{}", e.description()),
    }
}
