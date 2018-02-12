use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "database";
const CMD_HELP: &'static str = "Use to work with database structure";

const RESET: &'static str = "reset";
const RESET_HELP: &'static str = "Re-Initialize database (drop/create tables)";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(db)
        .subcommand(SubCommand::with_name(RESET).about(RESET_HELP))
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (RESET, Some(_)) => handler::database::reset(&database),
        (_, _) => ui::usage(arg)
    }
}
