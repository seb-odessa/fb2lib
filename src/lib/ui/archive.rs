use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "archive";
const CMD_HELP: &'static str = "Use to work with archives of FB2 books";

const SHOW: &'static str = "show";
const SHOW_HELP: &'static str = "Extract and print some book information";

const ZIP: &'static str = "zip";
const ZIP_HELP: &'static str = "Show book's offet, size and compression method in the archive";
const XML: &'static str = "xml";
const XML_HELP: &'static str = "Show book's description in a FB2 (XML) format.";
const FB2: &'static str = "fb2";
const FB2_HELP: &'static str = "Show book's description as a FictionBook structure";
const INF: &'static str = "inf";
const INF_HELP: &'static str = "Show book's brief description";
const BAD: &'static str = "bad";
const BAD_HELP: &'static str = "Show only broken books from archive";

const AUTHORS: &'static str = "authors";
const AUTHORS_HELP: &'static str = "Manage authors";
const LANGS: &'static str = "langs";
const LANGS_HELP: &'static str = "Manage languages";
const GENRES: &'static str = "genres";
const GENRES_HELP: &'static str = "Manage book's genres";
const TITLES: &'static str = "titles";
const TITLES_HELP: &'static str = "Manage book's titles";
const SEQUENCES: &'static str = "sequences";
const SEQUENCES_HELP: &'static str = "Manage book's sequences";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let arch = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true);
    let archs = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true).multiple(true);
    let book = Arg::with_name(ui::BOOK_FILE).help(ui::BOOK_FILE_HELP).required(false);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP)
        .subcommand(
            SubCommand::with_name(SHOW).about(SHOW_HELP)
            .subcommand(SubCommand::with_name(ZIP).about(ZIP_HELP).arg(arch.clone()).arg(book.clone()))
            .subcommand(SubCommand::with_name(XML).about(XML_HELP).arg(arch.clone()).arg(book.clone()))
            .subcommand(SubCommand::with_name(FB2).about(FB2_HELP).arg(arch.clone()).arg(book.clone()))
            .subcommand(SubCommand::with_name(INF).about(INF_HELP).arg(arch.clone()).arg(book.clone()))
            .subcommand(SubCommand::with_name(BAD).about(BAD_HELP).arg(arch.clone()).arg(book.clone()))
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(archs.clone()))
            .subcommand(SubCommand::with_name(LANGS).about(LANGS_HELP).arg(archs.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(archs.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(archs.clone()))
            .subcommand(SubCommand::with_name(GENRES).about(GENRES_HELP).arg(archs.clone()))
        )
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (SHOW, Some(arg)) => {
            handle_show(arg)
        }
        (_, _) => {
            ui::usage(arg)
        }
    }
}

fn handle_show<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (XML, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*.fb2");
            handler::archive::show_xml(&archive, book)
        }
        (FB2, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*.fb2");
            handler::archive::show_fb2(&archive, book)
        }
        (INF, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*.fb2");
            handler::archive::show_inf(&archive, book)
        }
        (ZIP, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*.fb2");
            handler::archive::show_zip(&archive, book)
        }
        (BAD, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            let book = arg.value_of(ui::BOOK_FILE).unwrap_or("*.fb2");
            handler::archive::show_bad(&archive, book)
        }        
        (AUTHORS, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::archive::authors(&archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (LANGS, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::archive::languages(&archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (TITLES, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::archive::titles(&archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (SEQUENCES, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::archive::sequences(&archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (GENRES, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::archive::genres(&archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (_, _) => ui::usage(arg)
    }
}
