use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "filter";
const CMD_HELP: &'static str = "Use to manage filters";

const LANG: &'static str = "lang";
const LANG_HELP: &'static str = "Use to manage language filters";
const LANG_ARG: &'static str = "language";
const LANG_ARG_HELP: &'static str = "Language name. Use */./? as willdcards";
const LANG_DISPLAY: &'static str = "display";
const LANG_DISPLAY_HELP: &'static str = "Print list of disabled and enabled languages";
const LANG_ENABLE: &'static str = "enable";
const LANG_ENABLE_HELP: &'static str = "Remove specified language from filtered (disabled) list";
const LANG_DISABLE: &'static str = "disable";
const LANG_DISABLE_HELP: &'static str = "Add specified language to filtered (disabled) list";

const GENRE: &'static str = "genre";
const GENRE_HELP: &'static str = "Use to manage genre filters";
const GENRE_NAME: &'static str = "name";
const GENRE_NAME_HELP: &'static str = "Genre name. Use */./? as willdcards";
const GENRE_GROUP: &'static str = "group";
const GENRE_GROUP_HELP: &'static str = "Interpret <name> as name of group";
const GENRE_DISPLAY: &'static str = "display";
const GENRE_DISPLAY_HELP: &'static str = "Print list of disabled and enabled genres";
const GENRE_ENABLE: &'static str = "enable";
const GENRE_ENABLE_HELP: &'static str = "Remove specified genre from filtered (disabled) list";
const GENRE_DISABLE: &'static str = "disable";
const GENRE_DISABLE_HELP: &'static str = "Add specified genre to filtered (disabled) list";


pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let lang = Arg::with_name(LANG_ARG).help(LANG_ARG_HELP).required(true);
    let name = Arg::with_name(GENRE_NAME).help(GENRE_NAME_HELP).required(true);
    let group = Arg::with_name(GENRE_GROUP).help(GENRE_GROUP_HELP).short("g").required(false);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(db)
        .subcommand(
            SubCommand::with_name(LANG).about(LANG_HELP)
            .subcommand(SubCommand::with_name(LANG_DISPLAY).about(LANG_DISPLAY_HELP))
            .subcommand(SubCommand::with_name(LANG_ENABLE).about(LANG_ENABLE_HELP).arg(lang.clone()))
            .subcommand(SubCommand::with_name(LANG_DISABLE).about(LANG_DISABLE_HELP).arg(lang.clone()))
        )
        .subcommand(
            SubCommand::with_name(GENRE).about(GENRE_HELP)
            .subcommand(SubCommand::with_name(GENRE_DISPLAY).about(GENRE_DISPLAY_HELP))
            .subcommand(SubCommand::with_name(GENRE_ENABLE).about(GENRE_ENABLE_HELP).arg(group.clone()).arg(name.clone()))
            .subcommand(SubCommand::with_name(GENRE_DISABLE).about(GENRE_DISABLE_HELP).arg(group.clone()).arg(name.clone()))            
        )
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (LANG, Some(arg)) => handle_lang(&database, &arg),
        (GENRE, Some(arg)) => handle_genre(&database, &arg),
        (_, _) => ui::usage(arg)
    }
}

fn handle_genre<'a>(db_file_name: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (GENRE_DISPLAY, Some(_)) => {
            handler::genre::display(db_file_name)
        }
        (GENRE_ENABLE, Some(arg)) => {
            let name = arg.value_of(GENRE_NAME).unwrap_or("").to_lowercase();
            handler::genre::enable(db_file_name, &name, arg.is_present(GENRE_GROUP))
        }
        (GENRE_DISABLE, Some(arg)) => {
            let name = arg.value_of(GENRE_NAME).unwrap_or("").to_lowercase();
            handler::genre::disable(db_file_name, &name, arg.is_present(GENRE_GROUP))
        }
        (_, _) => ui::usage(arg)
    }    
}

fn handle_lang<'a>(db_file_name: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (LANG_DISPLAY, Some(_)) => {
            handler::lang::display(db_file_name)
        }
        (LANG_ENABLE, Some(arg)) => {
            let lang = arg.value_of(LANG_ARG).unwrap_or("");
            handler::lang::enable(db_file_name, lang)
        }
        (LANG_DISABLE, Some(arg)) => {
            let lang = arg.value_of(LANG_ARG).unwrap_or("");
            handler::lang::disable(db_file_name, lang)
        }
        (_, _) => ui::usage(arg)
    }
}

