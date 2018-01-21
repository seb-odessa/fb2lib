extern crate lib;
extern crate clap;

use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};
use lib::ui::archive;
use lib::ui::database;
use lib::ui::torrent;

use lib::subcommands::*;

const VERSION: &'static str = "v0.5.0";
const AUTHOR: &'static str = "seb <seb@ukr.net>";

const VALUE: &'static str = "value";
const ARCH: &'static str = "archive.zip";
const DB: &'static str = "lib.rus.ec.db";

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

    let archive = Arg::with_name(ARCH)
        .help("an archive with books in FB2 format")
        .required(true);

    let db = Arg::with_name(DB)
        .help("a sqlite database file name")
        .required(true);

    let value = Arg::with_name(VALUE)
        .help("command argument")
        .required(true);


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
        .subcommand(
            cmd_lang
                .subcommand(cmd_lang_load)
                .subcommand(cmd_lang_show)
                .subcommand(cmd_lang_ignore)
        );


    let app = torrent::add(database::add(archive::add(app)));

    //------------------------------------------------------------------------------------------------------//
    let result = match app.get_matches().subcommand() {
        (archive::CMD, Some(arg)) => archive::handle(arg),
        (database::CMD, Some(arg)) => database::handle(arg),
        (torrent::CMD, Some(arg)) => torrent::handle(arg),

        (CMD_LANG, Some(cmd)) => {
            let database = get_or(&cmd, DB, DB);
            match cmd.subcommand() {
                (CMD_LANG_LOAD, Some(cmd)) => lang_load(&database, &get(&cmd, ARCH)),
                (CMD_LANG_SHOW, Some(cmd)) => lang_show(&database, &get(&cmd, ARCH)),
                (CMD_LANG_IGNORE, Some(cmd)) => lang_ignore(&database, &get(&cmd, VALUE)),
                (_, _) => {
                    // app.usage();
                    Ok(())
                }
            }
        }
        _ => {
            // app.usage();
            Ok(())
        }
    };
    //------------------------------------------------------------------------------------------------------//
    if result.is_err() {
        println!("Error: {}", result.unwrap_err());
    }
}
