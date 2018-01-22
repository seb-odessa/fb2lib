use ui;
use sal;
use result::into;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "filter";
const CMD_HELP: &'static str = "Use to work with book filters";
const DB_FILE: &'static str = "lib.rus.ec.db";
const DB_FILE_HELP: &'static str = "Database file name";

const LANG: &'static str = "lang";
const LANG_HELP: &'static str = "Language filter allows configure list of languages to ignore during book parsing";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(DB_FILE).help(DB_FILE_HELP).required(false);
    let cmd = SubCommand::with_name(CMD).about(CMD_HELP).arg(db);
    let lst = SubCommand::with_name(RESET).about(RESET_HELP);
    app.subcommand(
        cmd
        .subcommand(lst)
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(DB_FILE).unwrap_or(DB_FILE);
    match arg.subcommand() {
        (RESET, Some(_)) => reset(&database),
        (_, _) => ui::usage(arg)
    }
}

pub fn reset(db_file_name: &str) -> Fb2Result<()> {
    println!("reset({})", db_file_name);
    sal::cleanup_tables(db_file_name).map_err(into)
}
