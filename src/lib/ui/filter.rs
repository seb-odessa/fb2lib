use ui;
use sal;
use tools;
use archive;
use result::into;
use result::Fb2Result;
use clap::{App, Arg, SubCommand, ArgMatches};
use algorithm::apply_and_collect;

use std::sync::mpsc::channel;
use std::collections::HashSet;

pub const CMD: &'static str = "filter";
const CMD_HELP: &'static str = "Use to manage filters";

const LANG: &'static str = "lang";
const LANG_HELP: &'static str = "Use to manage language filters";

const LANG_LS: &'static str = "ls";
const LANG_LS_HELP: &'static str = "Print sorted unique list of languages from the specified archive.zip";
const LANG_ARG: &'static str = "language";
const LANG_ARG_HELP: &'static str = "Language name. Use <display> subcommand to show existing languages";

const LANG_DISPLAY: &'static str = "display";
const LANG_DISPLAY_HELP: &'static str = "Print list of disabled and enabled languages";

const LANG_ENABLE: &'static str = "enable";
const LANG_ENABLE_HELP: &'static str = "Remove specified language from filtered (disabled) list";

const LANG_DISABLE: &'static str = "disable";
const LANG_DISABLE_HELP: &'static str = "Add specified language to filtered (disabled) list";

const LANG_LOAD: &'static str = "load";
const LANG_LOAD_HELP: &'static str = "Load unique languages to the database";

pub fn add<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    let db = Arg::with_name(ui::DB_FILE).help(ui::DB_FILE_HELP).required(false);
    let archive = Arg::with_name(ui::ARCH_FILE).help(ui::ARCH_FILE_HELP).required(true);
    let lang = Arg::with_name(LANG_ARG).help(LANG_ARG_HELP).required(true);
    app.subcommand(
        SubCommand::with_name(CMD).about(CMD_HELP).arg(db)
        .subcommand(
            SubCommand::with_name(LANG).about(LANG_HELP)
            .subcommand(SubCommand::with_name(LANG_LS).about(LANG_LS_HELP).arg(archive.clone()))
            .subcommand(SubCommand::with_name(LANG_DISPLAY).about(LANG_DISPLAY_HELP))
            .subcommand(SubCommand::with_name(LANG_ENABLE).about(LANG_ENABLE_HELP).arg(lang.clone()))
            .subcommand(SubCommand::with_name(LANG_DISABLE).about(LANG_DISABLE_HELP).arg(lang.clone()))
            .subcommand(SubCommand::with_name(LANG_LOAD).about(LANG_LOAD_HELP).arg(archive.clone()))
        )
    )
}

pub fn handle<'a>(arg: &ArgMatches<'a>) -> Fb2Result<()> {
    let database = arg.value_of(ui::DB_FILE).unwrap_or(ui::DB_FILE);
    match arg.subcommand() {
        (LANG, Some(arg)) => handle_lang(&database, &arg),
        (_, _) => ui::usage(arg)
    }
}

fn handle_lang<'a>(db_file_name: &str, arg: &ArgMatches<'a>) -> Fb2Result<()> {
    match arg.subcommand() {
        (LANG_LS, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            lang_ls(db_file_name, archive)
        }
        (LANG_DISPLAY, Some(_)) => {
            lang_display(db_file_name)
        }
        (LANG_ENABLE, Some(arg)) => {
            let lang = arg.value_of(LANG_ARG).unwrap_or(LANG_ARG_HELP);
            lang_enable(db_file_name, lang)
        }
        (LANG_DISABLE, Some(arg)) => {
            let lang = arg.value_of(LANG_ARG).unwrap_or(LANG_ARG_HELP);
            lang_disable(db_file_name, lang)
        }
        (LANG_LOAD, Some(arg)) => {
            let archive = arg.value_of(ui::ARCH_FILE).unwrap_or("");
            lang_load(db_file_name, archive)
        }
        (_, _) => ui::usage(arg)
    }
}

fn extract_langs(db_file_name: &str, archive_name: &str) -> Fb2Result<Vec<String>> {
    println!("extract_langs({}, {})", db_file_name, archive_name);
    let zip = archive::open(archive_name)?;
    let (sender, receiver) = channel();
    apply_and_collect(zip, "*.fb2", sender, tools::into_fb2)?;
    let mut langs = HashSet::new();
    for fb2book in receiver.iter() {
        langs.insert(fb2book?.get_book_lang());
    }
    Ok(langs.into_iter().collect())
}

fn lang_load(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("lang_load({}, {})", db_file_name, archive_name);
    let langs = extract_langs(db_file_name, archive_name)?;
    let mut conn = sal::get_connection(db_file_name).map_err(into)?;
    let tx = conn.transaction().map_err(into)?;
    for lang in &langs {
        sal::insert_language(&tx, lang).map_err(into)?;
    }
    tx.commit().map_err(into)
}

fn lang_ls(db_file_name: &str, archive_name: &str) -> Fb2Result<()> {
    println!("lang_ls({}, {})", db_file_name, archive_name);
    for lang in &extract_langs(db_file_name, archive_name)? {
        println!("'{}'", lang);
    }
    Ok(())
}

fn lang_display(db_file_name: &str) -> Fb2Result<()> {
    println!("lang_display({})", db_file_name);
    let conn = sal::get_connection(db_file_name).map_err(into)?;
    print!("disabled languages: ");
    for lang in &sal::get_languages_disabled(&conn).map_err(into)? {
        print!("'{}' ", lang.1  );
    }
    println!("");
    print!("enabled languages: ");
    for lang in &sal::get_languages_enabled(&conn).map_err(into)? {
        print!("'{}' ", lang.1  );
    }
    println!("");
    Ok(())
}

fn lang_enable(db_file_name: &str, lang: &str) -> Fb2Result<()> {
    println!("lang_enable({}, {})", db_file_name, lang);
    let conn = sal::get_connection(db_file_name).map_err(into)?;
    sal::enable_language(&conn, lang).map_err(into)?;
    Ok(())
}

fn lang_disable(db_file_name: &str, lang: &str) -> Fb2Result<()> {
    println!("lang_disable({}, {})", db_file_name, lang);
    let conn = sal::get_connection(db_file_name).map_err(into)?;
    sal::disable_language(&conn, lang).map_err(into)?;
    Ok(())
}

