extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};
use lib::subcommands::*;

const VERSION: &'static str = "v0.5.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";

const TORRENT: &'static str = "archive.torrent";
const ARCH: &'static str = "archive.zip";
const BOOK: &'static str = "book.fb2";
const XML: &'static str = "book.xml";
const DB: &'static str = "lib.rus.ec.db";

const QUIET: &'static str = "QUIET";

const CMD_LS: &'static str = "ls";
const CMD_PARSE: &'static str = "parse";
const CMD_CHECK: &'static str = "check";

const CMD_SHOW: &'static str = "show";
const CMD_SHOW_XML: &'static str = "xml";
const CMD_SHOW_FB2: &'static str = "fb2";
const CMD_SHOW_INF: &'static str = "info";
const CMD_SHOW_ZIP: &'static str = "zip";

const CMD_DB: &'static str = "db";
const CMD_DB_INIT: &'static str = "init";
const CMD_DB_DROP: &'static str = "drop";
const CMD_DB_CHECK: &'static str = "check";
const CMD_DB_REGISTER: &'static str = "register";
const CMD_DB_LOAD: &'static str = "load";


fn get<'a>(arg: &ArgMatches<'a>, name: &str) -> String {
    arg.value_of(name).unwrap_or("").to_string()
}

fn get_or<'a>(arg: &ArgMatches<'a>, name: &str, default: &str) -> String {
    arg.value_of(name).unwrap_or(default).to_string()
}

fn main() {
    let torrent = Arg::with_name(TORRENT)
        .help("torrent file for archive.zip")
        .required(true);

    let archive = Arg::with_name(ARCH)
        .help("an archive with books in FB2 format")
        .required(true);

    let book = Arg::with_name(BOOK)
        .help("a file name in archive")
        .required(false);

    let xml = Arg::with_name(XML)
        .help("a file name in FB2 format")
        .required(true);

    let db = Arg::with_name(DB)
        .help("a sqlite database file name")
        .required(true);

    let quiet = Arg::with_name(QUIET)
        .short("q")
        .long("quiet")
        .help("Suppress console output during execution")
        .required(false);

    //------------------------------------------------------------------------------------------------------//
    let cmd_ls = SubCommand::with_name(CMD_LS)
        .about("List archive content")
        .arg(archive.clone().clone());
    //------------------------------------------------------------------------------------------------------//
    let cmd_check = SubCommand::with_name(CMD_CHECK)
        .about("Try parse all archive and print only failured books")
        .arg(archive.clone().clone())
        .arg(quiet);
    //------------------------------------------------------------------------------------------------------//
    let cmd_parse = SubCommand::with_name(CMD_PARSE)
        .about("Try parse xml into fb2 and print it")
        .arg(xml.clone());
    //------------------------------------------------------------------------------------------------------//
    let cmd_show = SubCommand::with_name(CMD_SHOW)
        .about("Use to extract and do something with zip content")
        .arg(archive.clone());
    let cmd_show_xml = SubCommand::with_name(CMD_SHOW_XML)
        .about("Print XML content of the fb2 description")
        .arg(book.clone());
    let cmd_show_fb2 = SubCommand::with_name(CMD_SHOW_FB2)
        .about("Print parsed FictionBook structure")
        .arg(book.clone());
    let cmd_show_inf = SubCommand::with_name(CMD_SHOW_INF)
        .about("Print human readable info for the fb2 file")
        .arg(book.clone());
    let cmd_show_zip = SubCommand::with_name(CMD_SHOW_ZIP)
        .about("Print human readable info for the file in zip archive")
        .arg(book.clone());
    //------------------------------------------------------------------------------------------------------//
    let cmd_db = SubCommand::with_name(CMD_DB)
        .about("Use to manage operations with external Database")
        .arg(db.clone().required(false));
    let cmd_db_init = SubCommand::with_name(CMD_DB_INIT).about("Initialize DB (create tables)");
    let cmd_db_drop = SubCommand::with_name(CMD_DB_DROP).about("Cleanup DB (drop tables)");
    let cmd_db_load = SubCommand::with_name(CMD_DB_LOAD)
        .about("Load data from archive")
        .arg(archive.clone());
    let cmd_db_register = SubCommand::with_name(CMD_DB_REGISTER)
        .about("Load metainfo from torrent ito DB")
        .arg(torrent.clone());
    let cmd_db_check = SubCommand::with_name(CMD_DB_CHECK)
        .about("Check integrity of the downloaded archive file (sha1 check)")
        .arg(archive.clone());
    //------------------------------------------------------------------------------------------------------//

    let arguments: Vec<String> = std::env::args().collect();
    let program = std::path::Path::new(&arguments[0])
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let app = App::new(program)
        .version(VERSION)
        .author(AUTHOR)
        .about("FictionBook Library Archive Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(cmd_ls)
        .subcommand(cmd_check)
        .subcommand(cmd_parse)
        .subcommand(
            cmd_show
                .subcommand(cmd_show_xml)
                .subcommand(cmd_show_fb2)
                .subcommand(cmd_show_inf)
                .subcommand(cmd_show_zip),
        )
        .subcommand(
            cmd_db
                .subcommand(cmd_db_init)
                .subcommand(cmd_db_drop)
                .subcommand(cmd_db_load)
                .subcommand(cmd_db_register)
                .subcommand(cmd_db_check),
        )
        .get_matches();
    //------------------------------------------------------------------------------------------------------//
    let result = match app.subcommand() {
        (CMD_LS, Some(arg)) => do_ls(&get(&arg, ARCH)),
        (CMD_CHECK, Some(arg)) => do_check(&get(&arg, ARCH), arg.occurrences_of(QUIET) != 0),
        (CMD_PARSE, Some(arg)) => do_parse(&get(&arg, XML)),
        (CMD_SHOW, Some(cmd)) => {
            let archive = get(&cmd, ARCH);
            match cmd.subcommand() {
                (CMD_SHOW_XML, Some(cmd)) => show_xml(&archive, &get_or(&cmd, BOOK, "*")),
                (CMD_SHOW_FB2, Some(cmd)) => show_fb2(&archive, &get_or(&cmd, BOOK, "*")),
                (CMD_SHOW_INF, Some(cmd)) => show_inf(&archive, &get_or(&cmd, BOOK, "*")),
                (CMD_SHOW_ZIP, Some(cmd)) => show_zip(&archive, &get_or(&cmd, BOOK, "*")),
                (_, _) => {
                    app.usage();
                    Ok(())
                }
            }
        }
        (CMD_DB, Some(cmd)) => {
            let database = get_or(&cmd, DB, DB);
            match cmd.subcommand() {
                (CMD_DB_INIT, Some(_)) => db_init(&database),
                (CMD_DB_DROP, Some(_)) => db_drop(&database),
                (CMD_DB_REGISTER, Some(cmd)) => db_register(&database, &get(&cmd, TORRENT)),
                (CMD_DB_LOAD, Some(cmd)) => db_load(&database, &get(&cmd, ARCH)),
                (CMD_DB_CHECK, Some(cmd)) => db_check(&database, &get(&cmd, ARCH)),
                (_, _) => {
                    app.usage();
                    Ok(())
                }
            }
        }
        _ => {
            app.usage();
            Ok(())
        }
    };
    //------------------------------------------------------------------------------------------------------//
    if result.is_err() {
        println!("Error: {}", result.unwrap_err());
    }
}
