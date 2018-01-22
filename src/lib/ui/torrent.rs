use ui;
use sal;
use filesystem;
use result::into;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};

pub const CMD: &'static str = "torrent";
const CMD_HELP: &'static str = "Use to work with torrent (metainfo) files";

const LOAD: &'static str = "load";
const LOAD_HELP: &'static str = "Load metainfo from torrent into DB";
const CHECK: &'static str = "check";
const CHECK_HELP: &'static str = "Compare existing file's SHA1 hashes with the loaded into DB";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(true);
    let archive = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true);
    let torrent = Arg::with_name(ui::TORRENT_FILE).help(ui::TORRENT_FILE_HELP).required(true);

    let cmd = SubCommand::with_name(CMD).about(CMD_HELP).arg(db);
    let load = SubCommand::with_name(LOAD).about(LOAD_HELP).arg(torrent);
    let check = SubCommand::with_name(CHECK).about(CHECK_HELP).arg(archive);
    app.subcommand(
        cmd
        .subcommand(load)
        .subcommand(check)
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (LOAD, Some(arg)) => {
            let torrent = arg.value_of(ui::TORRENT_FILE).unwrap_or("");
            torrent_load(database, torrent)
        },
        (CHECK, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            torrent_check(database, archive)
        }
        (_, _) => ui::usage(arg)
    }
}

fn torrent_load(db_file_name: &str, torrent_name: &str) -> Fb2Result<()> {
    println!("torrent_load({}, {})", db_file_name, torrent_name);
    let metainfo = filesystem::load_torrent(torrent_name)?;
    println!("file name:     {}", &metainfo.get_file_name());
    println!("creation date: {}", &metainfo.get_creation_date());
    println!("info hash:     {}", &metainfo.get_info_hash());
    println!("total length:  {}", &metainfo.get_length());
    println!("piece length:  {}", &metainfo.get_piece_length());
    println!("piece count:   {}", &metainfo.get_piece_count());
    sal::register(db_file_name, metainfo).map_err(into)
}

fn torrent_check(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("torrent_check({}, {})", db_file_name, archive_name);
    filesystem::check_integrity(db_file_name, archive_name)
}
