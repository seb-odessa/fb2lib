use ui;
use handler;
use result::Fb2Result;

use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "archive";
const CMD_HELP: &'static str = "Use to work with archives of FB2 books";

const LST: &'static str = "ls";
const LST_HELP: &'static str = "Print the list of files in the archive";
const CHECK: &'static str = "check";
const CHECK_HELP: &'static str = "Try parse all books in archive.";
const QUIET: &'static str = "QUIET";
const QUIET_HELP: &'static str = "Perform operation in QUIET mode";
const XML: &'static str = "xml";
const XML_HELP: &'static str = "Extract book(s) from archive as FB2 XML.";
const FB2: &'static str = "fb2";
const FB2_HELP: &'static str = "Extract book(s) from archive as FictionBook structure";
const INF: &'static str = "inf";
const INF_HELP: &'static str = "Extract book(s) brief information from archive";
const ZIP: &'static str = "zip";
const ZIP_HELP: &'static str = "Extract book(s) zipped file information from archive";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let arch = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true);
    let book = Arg::with_name(ui::BOOK_FILE).help(ui::BOOK_FILE_HELP).required(false);
    let quiet = Arg::with_name(QUIET).help(QUIET_HELP).required(false);

    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(arch)
        .subcommand(SubCommand::with_name(LST).about(LST_HELP))
        .subcommand(SubCommand::with_name(CHECK).about(CHECK_HELP).arg(quiet))
        .subcommand(SubCommand::with_name(XML).about(XML_HELP).arg(book.clone()))
        .subcommand(SubCommand::with_name(FB2).about(FB2_HELP).arg(book.clone()))
        .subcommand(SubCommand::with_name(INF).about(INF_HELP).arg(book.clone()))
        .subcommand(SubCommand::with_name(ZIP).about(ZIP_HELP).arg(book.clone()))
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let archive_name = arg.value_of(ui::ARCH_FILE).unwrap_or("").to_string();
    match arg.subcommand() {
        (LST, Some(_)) => {
            handler::archive::list_files(&archive_name)
        }
        (CHECK, Some(arg)) => {
            handler::archive::check_archive(&archive_name, arg.occurrences_of(QUIET) != 0)
        }
        (XML, Some(arg)) => {
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*");
            handler::archive::show_xml(&archive_name, book)
        }
        (FB2, Some(arg)) => {
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*");
            handler::archive::show_fb2(&archive_name, book)
        }
        (INF, Some(arg)) => {
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*");
            handler::archive::show_inf(&archive_name, book)
        }
        (ZIP, Some(arg)) => {
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*");
            handler::archive::show_zip(&archive_name, book)
        }
        (_, _) => {
            ui::usage(arg)
        }
    }
}

