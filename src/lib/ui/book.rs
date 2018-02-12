use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "book";
const CMD_HELP: &'static str = "Use to work with books";

const LS: &'static str = "ls";
const LS_HELP: &'static str = "List books from archive into the database";

const AUTHORS: &'static str = "authors";
const AUTHORS_HELP: &'static str = "Work with authors from archive into";

const LOAD: &'static str = "load";
const LOAD_HELP: &'static str = "Load to the database";

const FORCE: &'static str = "force";
const FORCE_HELP: &'static str = "Force load to the database";


pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let archive = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true).multiple(true);
    let load = Arg::with_name(LOAD).help(LOAD_HELP).long(LOAD).short("l").required(false);
    let force = Arg::with_name(FORCE).help(FORCE_HELP).long(FORCE).short("f").required(false);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(db)
        .subcommand(SubCommand::with_name(LS).about(LS_HELP).arg(archive.clone()))
        .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP)
            .arg(load.clone())
            .arg(force.clone())
            .arg(archive.clone())
        )
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (LS, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::book::ls(database, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        },
        (AUTHORS, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                let load = arg.is_present(LOAD);
                let force = arg.is_present(FORCE);
                handler::book::authors(database, load, force, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (_, _) => ui::usage(arg)
    }
}
