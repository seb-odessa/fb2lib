extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};
use std::error::Error;
use lib::result::Fb2Error;
use lib::subcommands::*;

const VERSION: &'static str = "v0.4.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";
const ARCHIVE: &'static str = "fb_archive.zip";
const FILE: &'static str = "fb_book.fb2";

const CMD_LS: &'static str = "ls";
const CMD_SHOW: &'static str = "show";
const CMD_XML: &'static str = "xml";
const CMD_FB2: &'static str = "fb2";
const CMD_INF: &'static str = "info";
const CMD_PARSE: &'static str = "parse";
const CMD_CHECK: &'static str = "check";

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let program = std::path::Path::new(&arguments[0]).file_name().unwrap().to_str().unwrap();
    let archive = Arg::with_name(ARCHIVE).help("Zip archive with books in FB2 format").index(1).required(true);
    let book = Arg::with_name(FILE).help("File in FB2 format").index(1).required(true);

    let cmd_ls = SubCommand::with_name(CMD_LS).about("List archive contents");
    let cmd_parse = SubCommand::with_name(CMD_PARSE).about("Parse all books in archive");
    let cmd_check = SubCommand::with_name(CMD_PARSE).about("Try parse all archive and print only failured books");
    let cmd_show = SubCommand::with_name(CMD_SHOW).about("Request to extract and print some kind of content");
    let cmd_show_xml = SubCommand::with_name(CMD_XML).about("Print XML content of the fb2 description").arg(book.clone());
    let cmd_show_fb2 = SubCommand::with_name(CMD_FB2).about("Print parsed FictionBook structure").arg(book.clone());
    let cmd_show_inf = SubCommand::with_name(CMD_INF).about("Print human readable info for the fb2 file").arg(book.required(false).clone());

    let app = App::new(program)
        .version(VERSION)
        .author(AUTHOR)
        .about("FictionBook Library Archive Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(archive)
        .subcommand(cmd_ls)
        .subcommand(cmd_check)
        .subcommand(cmd_parse)
        .subcommand(cmd_show
            .subcommand(cmd_show_xml)
            .subcommand(cmd_show_fb2)
            .subcommand(cmd_show_inf)
        )
        .get_matches();

    let archive = app.value_of(ARCHIVE).unwrap_or("");
    let result = match app.subcommand() {
        (CMD_LS,      Some(_)) => do_ls(&archive),
        (CMD_CHECK,   Some(_)) => do_check(&archive),
        (CMD_PARSE,   Some(_)) => do_parse(&archive),
        (CMD_SHOW,    Some(cmd)) => {
            match cmd.subcommand() {
                (CMD_XML,   Some(cmd)) => show_xml(&archive, &cmd.value_of(FILE).unwrap_or("")),
                (CMD_FB2,   Some(cmd)) => show_fb2(&archive, &cmd.value_of(FILE).unwrap_or("")),
                (CMD_INF,   Some(cmd)) => show_inf(&archive, &cmd.value_of(FILE).unwrap_or("")),
                (_,                 _) => Err(Fb2Error::UnsupportedSubCommand),
            }
        },
        ("",                _) => do_ls(&archive),
        _ => Err(Fb2Error::UnsupportedSubCommand),
    };

    match result {
        Ok(_) => {}
        Err(e) => println!("{}", e.description()),
    }
}
