use ui;
use handler;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "torrent";
const CMD_HELP: &'static str = "Use to work with torrent (metainfo) files";

const LOAD: &'static str = "load";
const LOAD_HELP: &'static str = "Load metainfo from torrent into DB";
const CHECK: &'static str = "check";
const CHECK_HELP: &'static str = "Compare existing file's SHA1 hashes with the loaded into DB";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let archive = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true);
    let torrent = Arg::with_name(ui::TORRENT_FILE).help(ui::TORRENT_FILE_HELP).required(true);

    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(db)
        .subcommand(SubCommand::with_name(LOAD).about(LOAD_HELP).arg(torrent))
        .subcommand(SubCommand::with_name(CHECK).about(CHECK_HELP).arg(archive))
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (LOAD, Some(arg)) => {
            let torrent = arg.value_of(ui::TORRENT_FILE).unwrap_or("");
            handler::torrent::load(database, torrent)
        },
        (CHECK, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            handler::torrent::check(database, archive)
        }
        (_, _) => ui::usage(arg)
    }
}
