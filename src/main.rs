extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};
use lib::subcommands::*;

const VERSION: &'static str = "v0.5.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";

const TORRENT: &'static str = "archive.torrent";
const VALUE: &'static str = "value";
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

const CMD_DB: &'static str = "database";
const CMD_DB_CLEAN: &'static str = "cleanup";

const CMD_TORRENT: &'static str = "torrent";
const CMD_TORRENT_CHECK: &'static str = "check";
const CMD_TORRENT_LOAD: &'static str = "load";

const CMD_LANG: &'static str = "lang";
const CMD_LANG_LOAD: &'static str = "load";
const CMD_LANG_SHOW: &'static str = "show";
const CMD_LANG_IGNORE: &'static str = "ignore";


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

    let value = Arg::with_name(VALUE)
        .help("command argument")
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
        .about("Use to manage external Database structure")
        .arg(db.clone().required(false));
    let cmd_db_clean = SubCommand::with_name(CMD_DB_CLEAN).about("Re-Initialize DB (drop/create tables)");    
    //------------------------------------------------------------------------------------------------------//
    let cmd_torrent = SubCommand::with_name(CMD_TORRENT)
        .about("Use to manage external external torrent files")
        .arg(db.clone().required(false));
    let cmd_torrent_load = SubCommand::with_name(CMD_TORRENT_LOAD)
        .about("Load metainfo from torrent into DB")
        .arg(torrent.clone());
    let cmd_torrent_check = SubCommand::with_name(CMD_TORRENT_CHECK)
        .about("Check integrity of the downloaded archive file (sha1 check)")
        .arg(archive.clone());
    //------------------------------------------------------------------------------------------------------//
    let cmd_lang = SubCommand::with_name(CMD_LANG)
        .about("Use to work with languages")
        .arg(db.clone().required(false));
    let cmd_lang_load = SubCommand::with_name(CMD_LANG_LOAD)
        .about("Load languages from archive into DB")
        .arg(archive.clone());
    let cmd_lang_show = SubCommand::with_name(CMD_LANG_SHOW)
        .about("Print unique sorted list of languages from archive")
        .arg(archive.clone());
    let cmd_lang_ignore = SubCommand::with_name(CMD_LANG_IGNORE)
        .about("Add language to ignore list")
        .arg(value.clone());

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
                .subcommand(cmd_show_zip)
        )
        .subcommand(
            cmd_db
                .subcommand(cmd_db_clean)
        )
        .subcommand(
            cmd_torrent
                .subcommand(cmd_torrent_load)
                .subcommand(cmd_torrent_check)
        )
        .subcommand(
            cmd_lang
                .subcommand(cmd_lang_load)
                .subcommand(cmd_lang_show)
                .subcommand(cmd_lang_ignore)
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
                (CMD_DB_CLEAN, Some(_)) => db_cleanup(&database),
                (_, _) => {
                    app.usage();
                    Ok(())
                }
            }
        }
        (CMD_TORRENT, Some(cmd)) => {
            let database = get_or(&cmd, DB, DB);
            match cmd.subcommand() {
                (CMD_TORRENT_LOAD, Some(cmd)) => torrent_load(&database, &get(&cmd, TORRENT)),
                (CMD_TORRENT_CHECK, Some(cmd)) => torrent_check(&database, &get(&cmd, ARCH)),
                (_, _) => {
                    app.usage();
                    Ok(())
                }
            }
        }        
        (CMD_LANG, Some(cmd)) => {
            let database = get_or(&cmd, DB, DB);
            match cmd.subcommand() {
                (CMD_LANG_LOAD, Some(cmd)) => lang_load(&database, &get(&cmd, ARCH)),
                (CMD_LANG_SHOW, Some(cmd)) => lang_show(&database, &get(&cmd, ARCH)),
                (CMD_LANG_IGNORE, Some(cmd)) => lang_ignore(&database, &get(&cmd, VALUE)),
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
