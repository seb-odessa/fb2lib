extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings};
use std::error::Error;
use lib::result::Fb2Error;
use lib::subcommands::*;

const VERSION: &'static str = "v0.4.3";
const AUTHOR: &'static str = "seb <seb@ukr.net>";
const ARCHIVE: &'static str = "FictionBook.zip";
const FILE: &'static str = "FictionBook.fb2";
const XML: &'static str = "FictionBook.xml";
const QUIET: &'static str = "QUIET";

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
    let archive = Arg::with_name(ARCHIVE).help("Zip archive with books in FB2 format").index(1).required(false);
    let book = Arg::with_name(FILE).help("file.fb2 in archive").index(1).required(true);
    let xml = Arg::with_name(XML).help("Xml file in FB2 format").index(1).required(true);
    let quiet = Arg::with_name(QUIET).short("q").long("quiet").help("Suppress console output during execution").required(false);

    let cmd_ls = SubCommand::with_name(CMD_LS).about("List archive contents");
    let cmd_parse = SubCommand::with_name(CMD_PARSE).about("Try parse xml into fb2 and print it").arg(xml.clone());
    let cmd_check = SubCommand::with_name(CMD_CHECK).about("Try parse all archive and print only failured books").arg(quiet);;
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
        (CMD_CHECK,   Some(cmd)) => {
            do_check(&archive, cmd.occurrences_of(QUIET) != 0)
        },
        (CMD_PARSE,   Some(arg)) => do_parse(&arg.value_of(XML).unwrap_or("")),
        (CMD_SHOW,    Some(cmd)) => {
            match cmd.subcommand() {
                (CMD_XML,   Some(cmd)) => show_xml(&archive, &cmd.value_of(FILE).unwrap_or("*")),
                (CMD_FB2,   Some(cmd)) => show_fb2(&archive, &cmd.value_of(FILE).unwrap_or("*")),
                (CMD_INF,   Some(cmd)) => show_inf(&archive, &cmd.value_of(FILE).unwrap_or("*")),
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
