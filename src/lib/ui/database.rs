use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "database";
const CMD_HELP: &'static str = "Use to work with database structure";

const RESET: &'static str = "reset";
const RESET_HELP: &'static str = "Re-Initialize database (drop/create tables)";
const LOAD: &'static str = "load";
const LOAD_HELP: &'static str = "Load data to the database from the archive";
const SHOW: &'static str = "show";
const SHOW_HELP: &'static str = "Show data from the database";
const ALIAS: &'static str = "alias";
const ALIAS_HELP: &'static str = "Make alias in the database";

const AUTHORS: &'static str = "authors";
const AUTHORS_HELP: &'static str = "Work with authors from the archive";
const LANGS: &'static str = "langs";
const LANGS_HELP: &'static str = "Work with book languages from the archive";
const TITLES: &'static str = "titles";
const TITLES_HELP: &'static str = "Work with book titles from the archive";
const SEQUENCES: &'static str = "sequences";
const SEQUENCES_HELP: &'static str = "Work with book sequences from the archive";

const ALL: &'static str = "all";
const ALL_HELP: &'static str = "Perform subcommand for all";
const FORCE: &'static str = "force";
const FORCE_HELP: &'static str = "Force save data to the database";
const PATTERN: &'static str = "pattern";
const PATTERN_HELP: &'static str = "Show records from database by pattern";
const ALIAS_SRC: &'static str = "aliace_src";
const ALIAS_SRC_HELP: &'static str = "Make aliace for this source";
const ALIAS_DST: &'static str = "aliace_dst";
const ALIAS_DST_HELP: &'static str = "Make target known as destination";


pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let database = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let arch = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true).multiple(true);
    let all = Arg::with_name(ALL).help(ALL_HELP).long(ALL).required(false);
    let force = Arg::with_name(FORCE).help(FORCE_HELP).long(FORCE).short("f").required(false);
    let pattern = Arg::with_name(PATTERN).help(PATTERN_HELP).required(false);
    let src = Arg::with_name(ALIAS_SRC).help(ALIAS_SRC_HELP).required(true);
    let dst = Arg::with_name(ALIAS_DST).help(ALIAS_DST_HELP).required(true);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(database)
        .subcommand(
            SubCommand::with_name(RESET).about(RESET_HELP).arg(all)
        )
        .subcommand(
            SubCommand::with_name(LOAD).about(LOAD_HELP)
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(force.clone()).arg(arch.clone()))
            .subcommand(SubCommand::with_name(LANGS).about(LANGS_HELP).arg(force.clone()).arg(arch.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(force.clone()).arg(arch.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(force.clone()).arg(arch.clone()))
        )
        .subcommand(
            SubCommand::with_name(SHOW).about(SHOW_HELP)
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(pattern.clone()))
            .subcommand(SubCommand::with_name(TITLES).about(TITLES_HELP).arg(pattern.clone()))
            .subcommand(SubCommand::with_name(SEQUENCES).about(SEQUENCES_HELP).arg(pattern.clone()))
        )
        .subcommand(
            SubCommand::with_name(ALIAS).about(ALIAS_HELP)
            .subcommand(SubCommand::with_name(AUTHORS).about(AUTHORS_HELP).arg(src.clone()).arg(dst.clone()))
        )
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (RESET, Some(arg)) => handle_reset(database, arg),
        (LOAD, Some(arg)) => handle_load(database, arg),
        (SHOW, Some(arg)) => handle_show(database, arg),
        (ALIAS, Some(arg)) => handle_alias(database, arg),
        (_, _) => ui::usage(arg)
    }
}

fn handle_reset<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (ALL, Some(_)) => handler::database::reset_all(&database),
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
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                let force = arg.is_present(FORCE);
                handler::database::load_langs(database, force, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (TITLES, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                let force = arg.is_present(FORCE);
                handler::database::load_titles(database, force, &archives.collect::<Vec<&str>>())
            } else {
                ui::usage(arg)
            }
        }
        (SEQUENCES, Some(arg)) => {
            if let Some(archives) = arg.values_of(ui::ARCH_FILE) {
                let force = arg.is_present(FORCE);
                handler::database::load_sequences(database, force, &archives.collect::<Vec<&str>>())
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
        // (TITLES, Some(arg)) => {
        //     let pattern = arg.value_of(PATTERN).unwrap_or("*");
        //     handler::database::show_titles(database, pattern)
        // }
        // (SEQUENCES, Some(arg)) => {
        //     let pattern = arg.value_of(PATTERN).unwrap_or("*");
        //     handler::database::show_sequences(database, pattern)
        // }
        (_, _) => ui::usage(arg)
    }
}

fn handle_alias<'a>(database: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
         (AUTHORS, Some(arg)) => {
            let src = arg.value_of(ALIAS_SRC).unwrap_or("");
            let dst = arg.value_of(ALIAS_DST).unwrap_or("");
            handler::database::alias_authors(database, src, dst)
        }
        // (TITLES, Some(arg)) => {
        //     let pattern = arg.value_of(PATTERN).unwrap_or("*");
        //     handler::database::show_titles(database, pattern)
        // }
        // (SEQUENCES, Some(arg)) => {
        //     let pattern = arg.value_of(PATTERN).unwrap_or("*");
        //     handler::database::show_sequences(database, pattern)
        // }
        (_, _) => ui::usage(arg)
    }
}
