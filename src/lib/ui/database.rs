use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

// @todo Add method for storing FictionBook descriptions into the DB. This will allow

pub const CMD: &'static str = "database";
const CMD_HELP: &'static str = "Use to work with database structure";

const RESET: &'static str = "reset";
const RESET_HELP: &'static str = "Re-Initialize database (drop/create tables)";
const LOAD: &'static str = "load";
const LOAD_HELP: &'static str = "Load data to the database from the archive";
const SHOW: &'static str = "show";
const SHOW_HELP: &'static str = "Show data from the database";
const LINK: &'static str = "link";
const LINK_HELP: &'static str = "Make link between records in the database";
const UNLINK: &'static str = "unlink";
const UNLINK_HELP: &'static str = "Drop link between records in the database";

const TORRENT: &'static str = "torrent";
const TORRENT_HELP: &'static str = "Handle torrents";
const PROGRESS: &'static str = "progress";
const PROGRESS_HELP: &'static str = "Handle progress subsystem";
const FILTER: &'static str = "filter";
const FILTER_HELP: &'static str = "Handle filter";
const GENRE: &'static str = "genre";
const GENRE_HELP: &'static str = "Handle book genres";
const LANGS: &'static str = "langs";
const LANGS_HELP: &'static str = "Handle book languages";
const NAMES: &'static str = "names";
const NAMES_HELP: &'static str = "Handle people names";


const AUTHORS: &'static str = "authors";
const AUTHORS_HELP: &'static str = "Handle authors";
const TITLES: &'static str = "titles";
const TITLES_HELP: &'static str = "Handle book titles";
const SEQUENCES: &'static str = "sequences";
const SEQUENCES_HELP: &'static str = "Handle book sequences";
const DESC: &'static str = "descriptions";
const DESC_HELP: &'static str = "Handle books descriptions";

const FORCE: &'static str = "force";
const FORCE_HELP: &'static str = "Force save data to the database";
const PATTERN: &'static str = "pattern";
const PATTERN_HELP: &'static str = "Show records from database by pattern";
const LINK_SRC: &'static str = "source";
const LINK_SRC_HELP: &'static str = "The source of the link";
const LINK_DST: &'static str = "destination";
const LINK_DST_HELP: &'static str = "The destination of the link";


pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let database = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let arch = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true).multiple(true);
    let force = Arg::with_name(FORCE).help(FORCE_HELP).long(FORCE).short("f").required(false);
    let pattern = Arg::with_name(PATTERN).help(PATTERN_HELP).required(false);
    let src = Arg::with_name(LINK_SRC).help(LINK_SRC_HELP).required(true);
    let dst = Arg::with_name(LINK_DST).help(LINK_DST_HELP).required(true);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(database)
        .subcommand(
            SubCommand::with_name(RESET).about(RESET_HELP)
            .subcommand(SubCommand::with_name(TORRENT).about(TORRENT_HELP))
            .subcommand(SubCommand::with_name(PROGRESS).about(PROGRESS_HELP))
            .subcommand(SubCommand::with_name(FILTER).about(FILTER_HELP))
            .subcommand(SubCommand::with_name(LANGS).about(LANGS_HELP))
            .subcommand(SubCommand::with_name(GENRE).about(GENRE_HELP))
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP))
            .subcommand(SubCommand::with_name(DESC).about(DESC_HELP))
        )
        .subcommand(
            SubCommand::with_name(LOAD).about(LOAD_HELP)
            .subcommand(SubCommand::with_name(LANGS).about(LANGS_HELP).arg(force.clone()))
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(force.clone()).arg(arch.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(force.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(force.clone()))
            .subcommand(SubCommand::with_name(NAMES).about(NAMES_HELP).arg(force.clone()))
            .subcommand(SubCommand::with_name(DESC).about(DESC_HELP).arg(arch.clone()))
        )
        .subcommand(
            SubCommand::with_name(SHOW).about(SHOW_HELP)
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(pattern.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(pattern.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(pattern.clone()))
        )
        .subcommand(
            SubCommand::with_name(LINK).about(LINK_HELP)
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(src.clone()).arg(dst.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(src.clone()).arg(dst.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(src.clone()).arg(dst.clone()))
        )
        .subcommand(
            SubCommand::with_name(UNLINK).about(UNLINK_HELP)
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(src.clone()).arg(dst.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(src.clone()).arg(dst.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(src.clone()).arg(dst.clone()))
        )
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (RESET, Some(arg)) => handle_reset(database, arg),
        (LOAD, Some(arg)) => handle_load(database, arg),
        (SHOW, Some(arg)) => handle_show(database, arg),
        (LINK, Some(arg)) => handle_link(database, arg),
        (UNLINK, Some(arg)) => handle_unlink(database, arg),
        (_, _) => ui::usage(arg)
    }
}

fn handle_reset<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (TORRENT, Some(_)) => handler::database::reset(database, "torrent"),
        (PROGRESS, Some(_)) => handler::database::reset(database, "progress"),
        (FILTER, Some(_)) => handler::database::reset(database, "filter"),
        (LANGS, Some(_)) => handler::database::reset(database, "lang"),
        (GENRE, Some(_)) => handler::database::reset(database, "genre"),
        (AUTHORS, Some(_)) => handler::database::reset(database, "author"),
        (TITLES, Some(_)) => handler::database::reset(database, "title"),
        (SEQUENCES, Some(_)) => handler::database::reset(database, "sequence"),
        (DESC, Some(_)) => handler::database::reset(database, DESC),
        (_, _) => ui::usage(arg)
    }
}

fn handle_load<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (AUTHORS, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                let force = arg.is_present(FORCE);
                handler::database::load_authors(database, force, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (LANGS, Some(arg)) => {
            let force = arg.is_present(FORCE);
            handler::database::load_langs(database, force)
        }
        (TITLES, Some(arg)) => {
            let force = arg.is_present(FORCE);
            handler::database::load_titles(database, force)
        }
        (SEQUENCES, Some(arg)) => {
            let force = arg.is_present(FORCE);
            handler::database::load_sequences(database, force)
        }
        (NAMES, Some(arg)) => {
            let force = arg.is_present(FORCE);
            handler::database::load_names(database, force)
        }
        (DESC, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                handler::database::load_descriptions(database, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (_, _) => ui::usage(arg)
    }
}

fn handle_show<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
         (AUTHORS, Some(arg)) => {
            let pattern = arg.value_of(PATTERN).unwrap_or("*");
            handler::database::show_authors(database, pattern)
        }
        (TITLES, Some(arg)) => {
            let pattern = arg.value_of(PATTERN).unwrap_or("*");
            handler::database::show_titles(database, pattern)
        }
        (SEQUENCES, Some(arg)) => {
            let pattern = arg.value_of(PATTERN).unwrap_or("*");
            handler::database::show_sequences(database, pattern)
        }
        (_, _) => ui::usage(arg)
    }
}

fn handle_link<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
         (AUTHORS, Some(arg)) => {
            let src = arg.value_of(LINK_SRC).unwrap_or("");
            let dst = arg.value_of(LINK_DST).unwrap_or("");
            handler::database::mk_link_authors(database, src, dst)
        }
        (TITLES, Some(arg)) => {
            let src = arg.value_of(LINK_SRC).unwrap_or("");
            let dst = arg.value_of(LINK_DST).unwrap_or("");
            handler::database::mk_link_titles(database, src, dst)
        }
        (SEQUENCES, Some(arg)) => {
            let src = arg.value_of(LINK_SRC).unwrap_or("");
            let dst = arg.value_of(LINK_DST).unwrap_or("");
            handler::database::mk_link_sequences(database, src, dst)
        }
        (_, _) => ui::usage(arg)
    }
}

fn handle_unlink<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
         (AUTHORS, Some(arg)) => {
            let src = arg.value_of(LINK_SRC).unwrap_or("");
            let dst = arg.value_of(LINK_DST).unwrap_or("");
            handler::database::rm_link_authors(database, src, dst)
        }
        (TITLES, Some(arg)) => {
            let src = arg.value_of(LINK_SRC).unwrap_or("");
            let dst = arg.value_of(LINK_DST).unwrap_or("");
            handler::database::rm_link_titles(database, src, dst)
        }
        (SEQUENCES, Some(arg)) => {
            let src = arg.value_of(LINK_SRC).unwrap_or("");
            let dst = arg.value_of(LINK_DST).unwrap_or("");
            handler::database::rm_link_sequences(database, src, dst)
        }
        (_, _) => ui::usage(arg)
    }
}
